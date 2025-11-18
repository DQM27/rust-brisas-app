<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  
  let loading = $state(false);
  let result = $state('');
  let config = $state<any>(null);

  async function testConnection() {
    loading = true;
    result = '';
    
    try {
      const response = await invoke<string>('test_supabase_connection');
      result = `✅ ${response}`;
    } catch (err: any) {
      result = `❌ ${err}`;
    } finally {
      loading = false;
    }
  }

  async function getConfig() {
    try {
      config = await invoke('get_supabase_config');
    } catch (err: any) {
      console.error('Error al obtener config:', err);
    }
  }

  $effect(() => {
    getConfig();
  });
</script>

<div class="p-6">
  <h2 class="text-2xl font-bold text-white mb-4">Prueba de Supabase</h2>
  
  {#if config}
    <div class="mb-4 p-4 bg-[#2d2d2d] rounded">
      <p class="text-gray-300">URL: <span class="text-[#007acc]">{config.url}</span></p>
      <p class="text-gray-300">Anon Key: <span class="text-green-500">{config.has_anon_key ? '✓ Configurada' : '✗ No configurada'}</span></p>
    </div>
  {/if}

  <button
    onclick={testConnection}
    disabled={loading}
    class="px-4 py-2 bg-[#007acc] text-white rounded hover:bg-[#005a9e] disabled:opacity-50"
  >
    {loading ? 'Probando...' : 'Probar Conexión'}
  </button>

  {#if result}
    <div class="mt-4 p-4 bg-[#1e1e1e] border border-[#3c3c3c] rounded">
      <pre class="text-sm text-gray-300">{result}</pre>
    </div>
  {/if}
</div>