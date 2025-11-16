
<script lang="ts">
  export let type: 'error' | 'success' = 'error';
  export let message: string = '';
</script>

{#if message}
  <div class="alert alert-{type}">
    {message}
  </div>
{/if}
