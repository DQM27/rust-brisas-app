<script lang="ts">
	import { isAuthenticated } from '$lib/stores/auth';
	import LoginPage from './LoginPage.svelte';
	import AppShell from '$lib/components/layout/AppShell.svelte';
</script>

<!-- 
  Access Control Layer: Single point of authentication check.
  Routes to LoginPage or AppShell based on auth state.
-->
{#if !$isAuthenticated}
	<LoginPage />
{:else}
	<AppShell />
{/if}
