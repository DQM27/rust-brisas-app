<script lang="ts">
  import { isAuthenticated } from '$lib/stores/auth';
  import LoginPage from './LoginPage.svelte';
  import TabsContainer from './app/+page.svelte';
</script>

{#if !$isAuthenticated}
  <LoginPage />
{:else}
  <TabsContainer />
{/if}