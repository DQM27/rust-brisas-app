<!-- src/lib/components/layout/GlobalUIProviders.svelte -->
<script lang="ts">
	import {
		showShortcutsHelp,
		showSpotlight,
		showUserProfileModal,
		selectedUserProfile
	} from '$lib/stores/ui';
	import { openTab } from '$lib/stores/tabs';

	// Components
	import KeyboardShortcuts from '$lib/components/layout/KeyboardShortcuts.svelte';
	import SpotlightSearch from '$lib/components/shared/SpotlightSearch.svelte';
	import ShortcutHelpModal from '$lib/components/modals/ShortcutHelpModal.svelte';
	import PersonaQuickCardModal from '$lib/components/shared/PersonaQuickCardModal.svelte';
	import QuickUserSwitchModal from '$lib/components/modals/QuickUserSwitchModal.svelte';
	import ProfileModal from '$lib/components/user/ProfileModal.svelte';
	import GlobalConfirmModal from '$lib/components/shared/GlobalConfirmModal.svelte';
	import SessionWarningModal from '$lib/components/modals/SessionWarningModal.svelte';
	import ScreensaverPasswordModal from '$lib/components/ScreensaverPasswordModal.svelte';
	import { awaitingScreensaverPassword } from '$lib/stores/sessionStore';

	let { authenticated = false } = $props();
	let showPasswordModal = $derived($awaitingScreensaverPassword);
</script>

{#if authenticated}
	<KeyboardShortcuts />
	<SpotlightSearch />

	<ShortcutHelpModal isOpen={$showShortcutsHelp} on:close={() => ($showShortcutsHelp = false)} />

	<PersonaQuickCardModal />
	<QuickUserSwitchModal />
	<GlobalConfirmModal />

	<ProfileModal
		show={$showUserProfileModal}
		user={$selectedUserProfile}
		onClose={() => showUserProfileModal.set(false)}
		onEdit={() => {
			const user = $selectedUserProfile;
			showUserProfileModal.set(false);
			if (user) {
				openTab({
					componentKey: 'user-list',
					title: 'Lista Usuarios',
					id: 'users-list',
					focusOnOpen: true,
					data: {
						editUserId: user.id
					}
				});
			}
		}}
	/>
{/if}

<!-- Screensaver Password Modal (Global) -->
{#if showPasswordModal}
	<ScreensaverPasswordModal />
{/if}

<!-- Session Warning Modal (Always active if monitoring) -->
<SessionWarningModal />
