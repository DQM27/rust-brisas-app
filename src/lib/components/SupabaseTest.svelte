<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let loading = $state(false);
  let result = $state("");
  let config = $state<any>(null);

  // Keyring states
  let keyringInfo = $state("");
  let keyringExists = $state(false);
  let keyringResult = $state("");
  let keyringLoading = $state(false);

  // Formulario para credenciales de prueba
  let testUrl = $state("https://test.supabase.co");
  let testAnonKey = $state("test-anon-key-123");
  let testPassword = $state("mi-password-secreto");

  // Credenciales leídas
  let loadedCredentials = $state<any>(null);

  async function testConnection() {
    loading = true;
    result = "";

    try {
      const response = await invoke<string>("test_supabase_connection");
      result = `✅ ${response}`;
    } catch (err: any) {
      result = `❌ ${err}`;
    } finally {
      loading = false;
    }
  }

  async function getConfig() {
    try {
      config = await invoke("get_supabase_config");
    } catch (err: any) {
      console.error("Error al obtener config:", err);
    }
  }

  // ============================================
  // FUNCIONES DE KEYRING
  // ============================================

  async function getKeyringInfo() {
    try {
      keyringInfo = await invoke<string>("keyring_info");
      await checkKeyringExists();
    } catch (err: any) {
      console.error("Error al obtener info del keyring:", err);
    }
  }

  async function checkKeyringExists() {
    try {
      keyringExists = await invoke<boolean>("keyring_check");
    } catch (err: any) {
      console.error("Error al verificar keyring:", err);
      keyringExists = false;
    }
  }

  async function saveToKeyring() {
    keyringLoading = true;
    keyringResult = "";
    loadedCredentials = null;

    try {
      const response = await invoke<string>("keyring_save", {
        url: testUrl,
        anonKey: testAnonKey,
        dbPassword: testPassword,
      });
      keyringResult = `✅ ${response}`;
      await checkKeyringExists();
    } catch (err: any) {
      keyringResult = `❌ Error: ${err}`;
    } finally {
      keyringLoading = false;
    }
  }

  async function loadFromKeyring() {
    keyringLoading = true;
    keyringResult = "";
    loadedCredentials = null;

    try {
      const creds = await invoke("keyring_load");
      loadedCredentials = creds;
      keyringResult = "✅ Credenciales leídas exitosamente";
    } catch (err: any) {
      keyringResult = `❌ Error al leer: ${err}`;
    } finally {
      keyringLoading = false;
    }
  }

  async function deleteFromKeyring() {
    keyringLoading = true;
    keyringResult = "";
    loadedCredentials = null;

    try {
      const response = await invoke<string>("keyring_delete");
      keyringResult = `✅ ${response}`;
      await checkKeyringExists();
    } catch (err: any) {
      keyringResult = `❌ Error al eliminar: ${err}`;
    } finally {
      keyringLoading = false;
    }
  }

  $effect(() => {
    getConfig();
    getKeyringInfo();
  });
</script>

<div class="p-6 space-y-8">
  <!-- SECCIÓN SUPABASE -->
  <section>
    <h2 class="text-2xl font-bold text-white mb-4">🗄️ Prueba de Supabase</h2>

    {#if config}
      <div class="mb-4 p-4 bg-[#2d2d2d] rounded">
        <p class="text-gray-300">
          URL: <span class="text-[#007acc]">{config.url}</span>
        </p>
        <p class="text-gray-300">
          Anon Key: <span class="text-green-500"
            >{config.has_anon_key ? "✓ Configurada" : "✗ No configurada"}</span
          >
        </p>
      </div>
    {/if}

    <button
      onclick={testConnection}
      disabled={loading}
      class="px-4 py-2 bg-[#007acc] text-white rounded hover:bg-[#005a9e] disabled:opacity-50"
    >
      {loading ? "Probando..." : "Probar Conexión"}
    </button>

    {#if result}
      <div class="mt-4 p-4 bg-[#1e1e1e] border border-[#3c3c3c] rounded">
        <pre class="text-sm text-gray-300">{result}</pre>
      </div>
    {/if}
  </section>

  <!-- SEPARADOR -->
  <div class="border-t border-[#3c3c3c]"></div>

  <!-- SECCIÓN KEYRING -->
  <section>
    <h2 class="text-2xl font-bold text-white mb-4">
      🔐 Prueba de Keyring (Almacenamiento Seguro)
    </h2>

    <!-- Info del sistema -->
    <div class="mb-4 p-4 bg-[#2d2d2d] rounded">
      <p class="text-gray-300">
        Sistema: <span class="text-[#007acc] font-semibold"
          >{keyringInfo || "Cargando..."}</span
        >
      </p>
      <p class="text-gray-300">
        Estado:
        <span class={keyringExists ? "text-green-500" : "text-yellow-500"}>
          {keyringExists ? "✓ Credenciales guardadas" : "✗ Sin credenciales"}
        </span>
      </p>
    </div>

    <!-- Formulario de prueba -->
    <div class="mb-4 p-4 bg-[#252526] rounded space-y-3">
      <h3 class="text-lg font-semibold text-white mb-2">Datos de Prueba:</h3>

      <div>
        <label for="supabase-url" class="block text-sm text-gray-400 mb-1"
          >URL de Supabase:</label
        >
        <input
          id="supabase-url"
          type="text"
          bind:value={testUrl}
          class="w-full px-3 py-2 bg-[#1e1e1e] border border-[#3c3c3c] rounded text-white focus:outline-none focus:border-[#007acc]"
          placeholder="https://tu-proyecto.supabase.co"
        />
      </div>

      <div>
        <label for="supabase-anon-key" class="block text-sm text-gray-400 mb-1"
          >Anon Key:</label
        >
        <input
          id="supabase-anon-key"
          type="text"
          bind:value={testAnonKey}
          class="w-full px-3 py-2 bg-[#1e1e1e] border border-[#3c3c3c] rounded text-white focus:outline-none focus:border-[#007acc]"
          placeholder="eyJhbGciOi..."
        />
      </div>

      <div>
        <label
          for="supabase-db-password"
          class="block text-sm text-gray-400 mb-1">DB Password:</label
        >
        <input
          id="supabase-db-password"
          type="password"
          bind:value={testPassword}
          class="w-full px-3 py-2 bg-[#1e1e1e] border border-[#3c3c3c] rounded text-white focus:outline-none focus:border-[#007acc]"
          placeholder="tu-password-secreto"
        />
      </div>
    </div>

    <!-- Botones de acción -->
    <div class="flex gap-3 flex-wrap">
      <button
        onclick={saveToKeyring}
        disabled={keyringLoading}
        class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50"
      >
        {keyringLoading ? "⏳" : "💾"} Guardar en Keyring
      </button>

      <button
        onclick={loadFromKeyring}
        disabled={keyringLoading || !keyringExists}
        class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
      >
        {keyringLoading ? "⏳" : "📖"} Leer del Keyring
      </button>

      <button
        onclick={checkKeyringExists}
        disabled={keyringLoading}
        class="px-4 py-2 bg-[#007acc] text-white rounded hover:bg-[#005a9e] disabled:opacity-50"
      >
        {keyringLoading ? "⏳" : "🔍"} Verificar Estado
      </button>

      <button
        onclick={deleteFromKeyring}
        disabled={keyringLoading || !keyringExists}
        class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 disabled:opacity-50"
      >
        {keyringLoading ? "⏳" : "🗑️"} Eliminar del Keyring
      </button>
    </div>

    <!-- Resultado -->
    {#if keyringResult}
      <div class="mt-4 p-4 bg-[#1e1e1e] border border-[#3c3c3c] rounded">
        <pre class="text-sm text-gray-300">{keyringResult}</pre>
      </div>
    {/if}

    <!-- Credenciales leídas -->
    {#if loadedCredentials}
      <div class="mt-4 p-4 bg-[#1e1e1e] border border-green-500/30 rounded">
        <h4 class="text-lg font-semibold text-green-400 mb-2">
          📦 Credenciales Recuperadas:
        </h4>
        <div class="space-y-1 text-sm">
          <p class="text-gray-300">
            URL: <span class="text-[#007acc]">{loadedCredentials.url}</span>
          </p>
          <p class="text-gray-300">
            Anon Key: <span class="text-yellow-400"
              >{loadedCredentials.anon_key}</span
            >
          </p>
          <p class="text-gray-300">
            DB Password: <span class="text-red-400"
              >{loadedCredentials.db_password}</span
            >
          </p>
        </div>
      </div>
    {/if}

    <!-- Instrucciones -->
    <div class="mt-6 p-4 bg-blue-900/20 border border-blue-500/30 rounded">
      <h4 class="text-sm font-semibold text-blue-400 mb-2">
        ℹ️ Cómo verificar manualmente:
      </h4>
      <div class="text-xs text-gray-400 space-y-1">
        <p>
          <strong>Windows:</strong> Control Panel → Credential Manager → Windows
          Credentials → Busca "brisas-app"
        </p>
        <p>
          <strong>macOS:</strong> Abre "Keychain Access.app" → Busca "brisas-app"
        </p>
        <p>
          <strong>Linux:</strong> Ejecuta:
          <code class="bg-[#1e1e1e] px-1 rounded"
            >secret-tool lookup service brisas-app username supabase</code
          >
        </p>
      </div>
    </div>
  </section>
</div>
