<script lang="ts">
  import '../app.css';
  import { isAuthenticated } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  onMount(() => {
    if (!$isAuthenticated) goto('/login');
  });
</script>

<div class="layout">
  <!-- Top Panel -->
  <div class="top-panel">
    <nav class="top-menu">
      <ul>
        <li>File</li><li>Edit</li><li>Selection</li><li>View</li><li>Go</li><li>Run</li><li>Terminal</li><li>Help</li>
      </ul>
    </nav>
    <div class="search-bar">
      <input placeholder="Buscar" />
    </div>
    <div class="window-controls">
      <button class="minimize">−</button>
      <button class="maximize">□</button>
      <button class="close">×</button>
    </div>
  </div>

  <!-- Main Area -->
  <div class="main">
    {#if $isAuthenticated}
      <Sidebar />
    {/if}
    <div class="editor-area">
      <slot />
    </div>
  </div>

  <!-- Status Bar -->
  <div class="status-bar">
    <div class="status-left">Ln 1, Col 1  Spaces: 2  UTF-8  HTML</div>
    <div class="status-right">
      <span>Remote</span>
      <span>0</span>
    </div>
  </div>
</div>


<style>
  .login-page {
    width: 100%;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #1e1e1e;
  }

  .card {
    background: #252526;
    padding: 2rem;
    border-radius: 8px;
    width: 100%;
    max-width: 400px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
  }

  h1 {
    text-align: center;
    margin: 0 0 1.5rem;
    color: #ccc;
  }

  .btn-link {
    background: none;
    border: none;
    color: #007acc;
    font-size: 0.9rem;
    cursor: pointer;
    margin-top: 1rem;
    display: block;
  }
</style>