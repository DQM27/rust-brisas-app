<script lang="ts">
  import { onMount } from 'svelte';
  import { Splitpanes, Pane } from 'svelte-splitpanes';
  import AuthPanel from '$lib/components/auth/AuthPanel.svelte';
  import Tabs from '$lib/components/layout/Tabs.svelte';
  import { tabsStore, activeTabId } from '$lib/stores/tabs';
  import { isAuthenticated, checkSession } from '$lib/stores/auth';

  onMount(() => {
   
    checkSession();
  });
</script>

<div class="app">
  {#if !$isAuthenticated}
    <div class="login-container">
      <AuthPanel />
    </div>

  {:else}
    <Splitpanes class="default-theme">
      
      <Pane>
        <Tabs tabs={$tabsStore} />
       
      </Pane>

       <Pane minSize={10} size={10}>
         <div class="sidebar">
          <h3>Módulos</h3>
          <p style="color: #888; font-size: 12px;">Panel lateral para navegación</p>
          
        </div>
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
    width: 100%;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary);
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