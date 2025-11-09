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

<div class="app">
  {#if !$isAuthenticated}
    <div class="login-container">
      <!-- Login ahora está en /login, no se muestra aquí -->
      <div style="display: none;"></div>
    </div>
  {:else}
    <Splitpanes class="default-theme">
      <Pane minSize={20} size={25}>
        <div class="sidebar">
          <h3>Módulos</h3>
          <p style="color: #888; font-size: 12px;">Panel lateral para navegación</p>
          <!-- TODO: Sidebar con menú de navegación -->
        </div>
      </Pane>

      <Pane>
        <Tabs tabs={$tabsStore} />
      </Pane>
    </Splitpanes>
  {/if}
</div>

<style>
  .app {
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .login-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background: #1e1e1e;
  }

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