
<script lang="ts">
  import { Splitpanes, Pane } from 'svelte-splitpanes';
  import AuthPanel from '$lib/components/auth/AuthPanel.svelte';
  import WelcomePanel from '$lib/components/WelcomePanel.svelte';
</script>

<div class="app">
  <Splitpanes class="default-theme">
    <Pane minSize={30} size={40}>
      <AuthPanel />
    </Pane>
    <Pane>
      <WelcomePanel />
    </Pane>
  </Splitpanes>
</div>

<style>
  .app {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }

  /* Estilos del splitpane */
  :global(.splitpanes) {
    background-color: #f5f7fa;
  }

  :global(.splitpanes__splitter) {
    background-color: #cbd5e0;
    position: relative;
    transition: background-color 0.2s;
  }

  :global(.splitpanes__splitter:hover) {
    background-color: #4299e1;
  }

  :global(.splitpanes__splitter::before) {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    transition: opacity 0.4s;
    background-color: rgba(66, 153, 225, 0.3);
    opacity: 0;
    z-index: 1;
  }

  :global(.splitpanes__splitter:hover::before) {
    opacity: 1;
  }

  :global(.splitpanes--vertical > .splitpanes__splitter::before) {
    left: -5px;
    right: -5px;
    height: 100%;
  }

  :global(.splitpanes--horizontal > .splitpanes__splitter::before) {
    top: -5px;
    bottom: -5px;
    width: 100%;
  }

  /* Panel contenedor */
  :global(.splitpanes__pane) {
    overflow: auto;
  }
</style>