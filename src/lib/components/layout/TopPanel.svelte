<!-- src/lib/components/layout/TopPanel.svelte -->
<script>
  let searchQuery = '';
  
  function handleMinimize() { /* lógica */ }
  function handleMaximize() { /* lógica */ }
  function handleClose() { /* lógica */ }
</script>

<div class="top-panel">
  <nav class="top-menu">
    <ul>
      <slot />
    </ul>
  </nav>
  <div class="search-bar">
    <input bind:value={searchQuery} placeholder="Buscar" />
  </div>
  <div class="window-controls">
    <button on:click={handleMinimize}>−</button>
    <button on:click={handleMaximize}>square</button>
    <button on:click={handleClose} class="close">times</button>
  </div>
</div>

<style>
  .top-panel {
    background: #3c3c3c;
    padding: 0 8px;
    display: flex;
    align-items: center;
    height: 35px;
  }
  .top-menu ul {
    display: flex;
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .top-menu :global(li) {
    padding: 0;
  }
  .top-menu :global(.menu-btn) {
    background: none;
    border: none;
    color: inherit;
    font: inherit;
    padding: 0 12px;
    cursor: pointer;
    height: 100%;
  }
  .top-menu :global(.menu-btn:hover) {
    background: #2a2d2e;
  }
  .search-bar {
    flex: 1;
    max-width: 500px;
    margin: 0 16px;
    position: relative;
    background: #252526;
    border-radius: 4px;
  }
  .search-bar input {
    width: 100%;
    background: transparent;
    border: none;
    padding: 4px 8px 4px 28px;
    color: #fff;
    font-size: 13px;
    outline: none;
    text-align: center;
  }
  .search-bar input::placeholder {
    color: #858585;
    text-align: center;
  }
  .search-bar::before {
    content: '';
    position: absolute;
    left: 8px;
    top: 50%;
    transform: translateY(-50%);
    color: #9e9e9e;
    font-size: 14px;
  }
  .window-controls {
    display: flex;
  }
  .window-controls button {
    background: none;
    border: none;
    color: #fff;
    font-size: 16px;
    width: 46px;
    height: 100%;
    cursor: pointer;
  }
  .window-controls button:hover {
    background: #2a2d2e;
  }
  .close:hover {
    background: #e81123;
  }
</style>