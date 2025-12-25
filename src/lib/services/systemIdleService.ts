/**
 * System Idle Service
 * 
 * Provides system-wide idle detection using Windows API GetLastInputInfo()
 * Detects when the user last interacted with ANY application, not just this app.
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * Get system-wide idle time in milliseconds
 * 
 * @returns Idle time in milliseconds, or 0 if detection fails
 */
export async function getSystemIdleTime(): Promise<number> {
    try {
        const idleMs = await invoke<number>('get_system_idle_time');
        return idleMs;
    } catch (error) {
        console.error('[SystemIdle] Error getting system idle time:', error);
        return 0;
    }
}

/**
 * Get system-wide idle time in seconds
 * 
 * @returns Idle time in seconds
 */
export async function getSystemIdleSeconds(): Promise<number> {
    const idleMs = await getSystemIdleTime();
    return Math.floor(idleMs / 1000);
}

/**
 * Get system-wide idle time in minutes
 * 
 * @returns Idle time in minutes
 */
export async function getSystemIdleMinutes(): Promise<number> {
    const idleMs = await getSystemIdleTime();
    return Math.floor(idleMs / 1000 / 60);
}

/**
 * Check if system has been idle for more than X minutes
 * 
 * @param minutes - Threshold in minutes
 * @returns True if system has been idle longer than threshold
 */
export async function isSystemIdleFor(minutes: number): Promise<boolean> {
    const idleMinutes = await getSystemIdleMinutes();
    return idleMinutes >= minutes;
}
