<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import '../app.css';
  import { isAuthenticated } from '$lib/stores/auth';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
</script>

<div class="layout">
  <!-- Top Panel (VS Code style) -->
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
  * { margin:0; padding:0; box-sizing:border-box; font-family:Segoe UI,Helvetica,Arial,sans-serif; }
  :global(html,body) { height:100%; overflow:hidden; background:#252526; color:#ccc; }
  .layout { display:flex; flex-direction:column; height:100vh; }

  /* TOP PANEL */
  .top-panel { 
    background:#3c3c3c; 
    padding:0 8px; 
    display:flex; 
    align-items:center; 
    height:35px; 
    -webkit-app-region: drag;
  }
  .top-menu ul { 
    display:flex; 
    list-style:none; 
    margin-right: 16px;
  }
  .top-menu li { 
    padding:0 12px; 
    font-size:13px; 
    cursor:pointer; 
    -webkit-app-region: no-drag;
  }
  .top-menu li:hover { background:#2a2d2e; }
  .search-bar { 
    flex:1; 
    max-width:500px; 
    margin:0 16px; 
    position:relative; 
    background:#252526; 
    border-radius:4px; 
  }
  .search-bar input { 
    width:100%; 
    background:transparent; 
    border:none; 
    padding:4px 8px 4px 28px; 
    color:#fff; 
    font-size:13px; 
    outline:none; 
    text-align:center; 
    -webkit-app-region: no-drag;
  }
  .search-bar input::placeholder { color:#858585; text-align:center; }
  .search-bar::before { 
    content:''; 
    position:absolute; 
    left:8px; 
    top:50%; 
    transform:translateY(-50%); 
    color:#9e9e9e; 
    font-size:14px; 
  }
  .window-controls { 
    display:flex; 
    -webkit-app-region: no-drag;
  }
  .window-controls button { 
    background:none; 
    border:none; 
    color:#fff; 
    font-size:16px; 
    width:46px; 
    height:100%; 
    cursor:pointer; 
  }
  .window-controls button:hover { background:#2a2d2e; }
  .close:hover { background:#e81123; }

  /* MAIN LAYOUT */
  .main {
    display: flex;
    flex-direction: row;
    flex: 1;
    width: 100%;
    overflow: hidden;
  }

  .editor-area {
    flex: 1;
    background: #1e1e1e;
    overflow: auto;
  }

  /* STATUS BAR */
  .status-bar { 
    background:#007acc; 
    height:22px; 
    display:flex; 
    align-items:center; 
    padding:0 8px; 
    font-size:12px; 
    -webkit-app-region: no-drag;
  }
  .status-left { flex:1; }
  .status-right { display:flex; gap:16px; }
</style>