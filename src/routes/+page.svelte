<script lang="ts">
  import { onMount } from 'svelte';
  import { isAuthenticated } from '$lib/stores/auth';
  import { tabsStore, openTab } from '$lib/stores/tabs';
  import { get } from 'svelte/store';
  import LoginPage from './LoginPage.svelte';
   import Main from './MainContent.svelte';
  
  // Cuando se autentica, inicializar tabs
  $effect(() => {
    if ($isAuthenticated) {
      const tabs = get(tabsStore);
      if (tabs.length === 0) {
        openTab({ componentKey: 'welcome', title: 'Bienvenida', id: 'welcome' });
      }
    }
  });
</script>

{#if !$isAuthenticated}
  <LoginPage />
{:else}
  
{/if}