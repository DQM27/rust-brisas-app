import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import { sessionSettings } from './sessionSettingsStore';
import { logout, currentUser } from './auth';
import { openTab, activeTabId } from './tabs';
import { getSystemIdleMinutes } from '$lib/services/systemIdleService';

// =============================================================================
// TYPES
// =============================================================================

export type SessionMode = 'active' | 'screensaver' | 'locked';

interface SessionState {
    mode: SessionMode;
    lastActivityTime: number;
    screensaverActive: boolean;
    awaitingPasswordForScreensaver: boolean;
}

// =============================================================================
// STORE
// =============================================================================

const initialState: SessionState = {
    mode: 'active',
    lastActivityTime: Date.now(),
    screensaverActive: false,
    awaitingPasswordForScreensaver: false,
};

const sessionState = writable<SessionState>(initialState);

// =============================================================================
// ACTIVITY TRACKING (Kept for manual reset after screensaver exit)
// =============================================================================

let activityListenersActive = false;
let checkIntervalId: ReturnType<typeof setInterval> | null = null;
let debounceTimeout: ReturnType<typeof setTimeout> | null = null;
let screensaverCooldown = false; // Flag to prevent immediate deactivation after screensaver activates

/**
 * Updates the last activity time (debounced to avoid excessive updates)
 * NOW ONLY USED to exit screensaver when password is not required
 */
function recordActivity(): void {
    // Don't record activity if we're waiting for screensaver password
    const state = get(sessionState);
    if (state.awaitingPasswordForScreensaver) {
        return;
    }

    // Don't record activity during screensaver cooldown period
    if (screensaverCooldown) {
        console.log('[Session] Ignoring activity during screensaver cooldown');
        return;
    }

    if (debounceTimeout) {
        clearTimeout(debounceTimeout);
    }

    debounceTimeout = setTimeout(() => {
        sessionState.update((s) => ({
            ...s,
            lastActivityTime: Date.now(),
        }));

        // If screensaver is active and no password required, exit it
        const currentState = get(sessionState);
        const settings = get(sessionSettings);

        if (currentState.screensaverActive && !settings.screensaverRequiresPassword) {
            exitScreensaver();
        }
    }, 100); // 100ms debounce
}

/**
 * Event handlers for activity detection
 */
const activityEvents = ['mousemove', 'keydown', 'click', 'scroll', 'touchstart'];

function attachActivityListeners(): void {
    if (!browser || activityListenersActive) return;

    activityEvents.forEach((event) => {
        window.addEventListener(event, recordActivity, { passive: true });
    });

    activityListenersActive = true;
}

function detachActivityListeners(): void {
    if (!browser || !activityListenersActive) return;

    activityEvents.forEach((event) => {
        window.removeEventListener(event, recordActivity);
    });

    activityListenersActive = false;
}

// =============================================================================
// TIMEOUT CHECKING (Configurable: System-Wide or App-Only Detection)
// =============================================================================

/**
 * Checks if timeouts have been reached and triggers appropriate actions
 * Uses either system-wide or app-only idle detection based on user settings
 */
async function checkTimeouts(): Promise<void> {
    const state = get(sessionState);
    const settings = get(sessionSettings);
    const user = get(currentUser);

    // Don't check timeouts if not logged in
    if (!user) {
        return;
    }

    // Get idle time based on user preference
    let inactiveMinutes: number;

    if (settings.useSystemWideDetection) {
        // System-wide detection: detects PC idle time (mouse, keyboard globally)
        inactiveMinutes = await getSystemIdleMinutes();
        console.log(`[Session] System-wide idle for ${inactiveMinutes} minutes`);
    } else {
        // App-only detection: detects activity only within this app
        const now = Date.now();
        inactiveMinutes = (now - state.lastActivityTime) / 1000 / 60;
        console.log(`[Session] App-only idle for ${inactiveMinutes.toFixed(1)} minutes`);
    }

    // Check for complete logout timeout
    if (settings.enableCompleteTimeout && inactiveMinutes >= settings.completeTimeoutMinutes) {
        console.log('[Session] Complete logout timeout reached');
        performCompleteLogout();
        return;
    }

    // Check for screensaver timeout (only if not already in screensaver)
    if (
        settings.enableScreensaver &&
        !state.screensaverActive &&
        inactiveMinutes >= settings.screensaverTimeoutMinutes
    ) {
        console.log('[Session] Screensaver timeout reached');
        enterScreensaver();
        return;
    }
}

/**
 * Starts the interval-based timeout checker
 */
function startTimeoutChecker(): void {
    if (checkIntervalId !== null) {
        return; // Already running
    }

    // Check every 10 seconds
    checkIntervalId = setInterval(checkTimeouts, 10000);
    console.log('[Session] Timeout checker started');
}

/**
 * Stops the interval-based timeout checker
 */
function stopTimeoutChecker(): void {
    if (checkIntervalId !== null) {
        clearInterval(checkIntervalId);
        checkIntervalId = null;
        console.log('[Session] Timeout checker stopped');
    }
}

// =============================================================================
// SCREENSAVER MANAGEMENT
// =============================================================================

/**
 * Enters screensaver mode
 */
export function enterScreensaver(): void {
    console.log('[Session] Entering screensaver mode');

    // Set cooldown to prevent activity detection during tab opening
    screensaverCooldown = true;
    console.log('[Session] Activating screensaver cooldown (1000ms)');

    // Open/focus the welcome tab
    const welcomeTabId = 'welcome';
    openTab({
        componentKey: 'welcome',
        title: 'Bienvenida',
        id: welcomeTabId,
        focusOnOpen: true,
    });

    // Update session state
    sessionState.update((s) => ({
        ...s,
        mode: 'screensaver',
        screensaverActive: true,
    }));

    // Clear cooldown after delay
    setTimeout(() => {
        screensaverCooldown = false;
        console.log('[Session] Screensaver cooldown cleared - now detecting activity');
    }, 1000);

    // The WelcomePanel will handle entering fullscreen via isKioskMode
    // This will be triggered in the layout component when it observes screensaverActive
}

/**
 * Attempts to exit screensaver mode
 * If password required, shows password modal
 * Otherwise, exits immediately
 */
export function attemptExitScreensaver(): void {
    const settings = get(sessionSettings);
    const state = get(sessionState);

    if (!state.screensaverActive) {
        return;
    }

    if (settings.screensaverRequiresPassword) {
        console.log('[Session] Password required to exit screensaver');
        sessionState.update((s) => ({
            ...s,
            awaitingPasswordForScreensaver: true,
        }));
        // The layout component will show the password modal
    } else {
        console.log('[Session] Exiting screensaver (no password required)');
        exitScreensaver();
    }
}

/**
 * Exits screensaver mode (after password verification or if no password required)
 */
export function exitScreensaver(): void {
    console.log('[Session] Exiting screensaver mode');

    sessionState.update((s) => ({
        ...s,
        mode: 'active',
        screensaverActive: false,
        awaitingPasswordForScreensaver: false,
        lastActivityTime: Date.now(), // Reset activity timer
    }));

    // The layout component will handle exiting fullscreen
}

/**
 * Cancels screensaver password prompt and performs full logout
 */
export async function cancelScreensaverPassword(): Promise<void> {
    console.log('[Session] Screensaver password cancelled, logging out');

    // First exit fullscreen if we're in it
    if (browser) {
        try {
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            const appWindow = getCurrentWindow();
            const isFullscreen = await appWindow.isFullscreen();

            if (isFullscreen) {
                console.log('[Session] Exiting fullscreen before logout');
                await appWindow.setFullscreen(false);
            }
        } catch (e) {
            console.error('[Session] Error exiting fullscreen before logout:', e);
        }
    }

    performCompleteLogout();
}

// =============================================================================
// LOGOUT MANAGEMENT
// =============================================================================

/**
 * Performs a complete logout (closes all tabs, clears session)
 */
function performCompleteLogout(): void {
    console.log('[Session] Performing complete logout');

    // Stop all session monitoring
    stopSession();

    // Call the existing logout function (which handles tab closure)
    logout();
}

// =============================================================================
// SESSION LIFECYCLE
// =============================================================================

/**
 * Starts session monitoring (call on login)
 */
export function startSession(): void {
    if (!browser) return;

    console.log('[Session] Starting session monitoring');

    // Reset state
    sessionState.set({
        mode: 'active',
        lastActivityTime: Date.now(),
        screensaverActive: false,
        awaitingPasswordForScreensaver: false,
    });

    // Start activity tracking
    attachActivityListeners();

    // Start timeout checking
    startTimeoutChecker();
}

/**
 * Stops session monitoring (call on logout)
 */
export function stopSession(): void {
    if (!browser) return;

    console.log('[Session] Stopping session monitoring');

    // Stop activity tracking
    detachActivityListeners();

    // Stop timeout checking
    stopTimeoutChecker();

    // Clear any pending debounce
    if (debounceTimeout) {
        clearTimeout(debounceTimeout);
        debounceTimeout = null;
    }

    // Reset state
    sessionState.set(initialState);
}

// =============================================================================
// EXPORTS
// =============================================================================

// Read-only derived stores for components to subscribe to
export const isScreensaverActive = derived(sessionState, ($state) => $state.screensaverActive);
export const awaitingScreensaverPassword = derived(
    sessionState,
    ($state) => $state.awaitingPasswordForScreensaver
);
export const sessionMode = derived(sessionState, ($state) => $state.mode);

// For debugging (can be removed in production)
export const sessionDebugInfo = derived(
    [sessionState, sessionSettings],
    ([$state, $settings]) => {
        const now = Date.now();
        const inactiveSeconds = Math.floor((now - $state.lastActivityTime) / 1000);
        const inactiveMinutes = Math.floor(inactiveSeconds / 60);

        return {
            mode: $state.mode,
            screensaverActive: $state.screensaverActive,
            awaitingPassword: $state.awaitingPasswordForScreensaver,
            inactiveSeconds,
            inactiveMinutes,
            screensaverTimeoutMinutes: $settings.screensaverTimeoutMinutes,
            completeTimeoutMinutes: $settings.completeTimeoutMinutes,
            screensaverEnabled: $settings.enableScreensaver,
            completeTimeoutEnabled: $settings.enableCompleteTimeout,
        };
    }
);
