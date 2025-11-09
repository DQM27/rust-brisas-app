<!-- src/routes/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { Splitpanes, Pane } from 'svelte-splitpanes';
  import Tabs from '$lib/components/layout/Tabs.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import { tabsStore } from '$lib/stores/tabs';
  import { isAuthenticated, checkSession } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  onMount(() => {
    const authenticated = checkSession();
    if (!authenticated) {
      goto('/login');
    }
  });
</script>

{#if $isAuthenticated}
  <Splitpanes class="default-theme">
    <!-- Sidebar a la IZQUIERDA (52px) -->
    <Pane minSize={52} size={52} maxSize={52}>
      <Sidebar />
    </Pane>

    <!-- Tabs en el centro (resto del espacio) -->
    <Pane>
      <Tabs tabs={$tabsStore} />
    </Pane>
  </Splitpanes>
{:else}
  <div style="display: none;"></div>
{/if}

<style>
  :global(body, html, #svelte) {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  :global(.splitpanes__pane) {
    background: #1e1e1e;
  }
</style>