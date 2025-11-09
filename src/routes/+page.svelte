<!-- src/routes/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { Splitpanes, Pane } from 'svelte-splitpanes';
  import Tabs from '$lib/components/layout/Tabs.svelte';
  import { tabsStore } from '$lib/stores/tabs';
  import { isAuthenticated, checkSession } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  onMount(() => {
    checkSession();
    if (!$isAuthenticated) {
      goto('/login');
    }
  });
</script>

{#if $isAuthenticated}
  <Splitpanes class="default-theme">
    <Pane minSize={20} size={25}>
      <div class="sidebar">
        <h3>Módulos</h3>
        <p style="color: #888; font-size: 12px;">Panel lateral para navegación</p>
      </div>
    </Pane>

    <Pane>
      <Tabs tabs={$tabsStore} />
    </Pane>
  </Splitpanes>
{:else}
  <div style="display: none;"></div>
{/if}

<style>
  .sidebar {
    height: 100%;
    padding: 16px;
    background: #252526;
    color: #ccc;
    overflow-y: auto;
  }

  .sidebar h3 {
    margin: 0 0 12px 0;
    font-size: 14px;
    font-weight: 600;
    color: #fff;
  }
</style>