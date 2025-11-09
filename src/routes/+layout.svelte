<!-- src/routes/+layout.svelte -->
<script>
  import '../app.css';
  import { isAuthenticated } from '$lib/stores/auth';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import TopPanel from '$lib/components/layout/TopPanel.svelte';
  import StatusBar from '$lib/components/layout/StatusBar.svelte';
  import Toast from '$lib/components/Toast.svelte';
  import { toast } from 'svelte-5-french-toast';
  
  function showSuccess() {
    toast.success('Archivo guardado');
  }
</script>

<div class="layout">
  <TopPanel>
    <li><button class="menu-btn">File</button></li>
    <li><button class="menu-btn">Edit</button></li>
    <li><button class="menu-btn">Selection</button></li>
    <li><button class="menu-btn">View</button></li>
    <li><button class="menu-btn">Go</button></li>
    <li><button class="menu-btn" on:click={showSuccess}>Run</button></li>
    <li><button class="menu-btn">Terminal</button></li>
    <li><button class="menu-btn">Help</button></li>
  </TopPanel>

  <div class="main">
    {#if $isAuthenticated}<Sidebar />{/if}
    <main class="content"><slot /></main>
  </div>

  <StatusBar line={1} col={1} encoding="UTF-8" language="HTML" remote={false} notifications={0} />
</div>

<Toast />

<style>
  .layout { display: flex; flex-direction: column; height: 100vh; }
  .main { display: flex; flex: 1; overflow: hidden; }
  .content { flex: 1; overflow: auto; background: #1e1e1e }
  .menu-btn { background: none; border: none; color: inherit; font: inherit; padding: 0 12px; cursor: pointer; height: 100%; }
  .menu-btn:hover { background: #2a2d2e; }
</style>