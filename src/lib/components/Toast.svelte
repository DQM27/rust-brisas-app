<!-- src/lib/components/Toast.svelte -->
<script>
  import { Toaster } from 'svelte-5-french-toast';
</script>

<Toaster 
  position="bottom-right"
  toastOptions={{
    style: 'background: #1e1e1e; color: #fff; border: 1px solid #007acc;',
    className: 'custom-toast'
  }}
/>