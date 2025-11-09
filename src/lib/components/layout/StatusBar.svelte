
<script>
  export let line = 1;
  export let col = 1;
  export let encoding = 'UTF-8';
  export let language = 'HTML';
  export let remote = false;
  export let notifications = 0;
</script>

<div class="status-bar">
  <div class="status-left">
    Ln {line}, Col {col}  Spaces: 2  {encoding}  {language}
  </div>
  <div class="status-right">
    {#if remote}<span>Remote</span>{/if}
    <span>{notifications}</span>
  </div>
</div>

<style>
  .status-bar { background:#007acc; height:22px; display:flex; align-items:center; padding:0 8px; font-size:12px; }
  .status-left { flex:1; }
  .status-right { display:flex; gap:16px; }
</style>