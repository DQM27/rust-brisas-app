<script lang="ts">
  import { isAuthenticated, login as setAuth } from '$lib/stores/auth';
  import LoginForm from '$lib/components/LoginForm.svelte';

  import { validateCredentials } from '$lib/logic/auth/validateCredentials';
  import { loginUser } from '$lib/logic/auth/loginUser';
  import { parseAuthError } from '$lib/logic/auth/parseAuthErrors';

  import { Splitpanes, Pane } from 'svelte-splitpanes';
  import Tabs from '$lib/components/layout/Tabs.svelte';
  import { tabsStore, openTab } from '$lib/stores/tabs'; 
  import { inspectionPanel } from '$lib/stores/ui';
  import { ChevronDown } from 'lucide-svelte';
  import { toast } from 'svelte-5-french-toast';

  import type LoginFormType from '$lib/components/LoginForm.svelte';

  let loading = $state(false);
  let formRef = $state<LoginFormType>();
  let inspectionContent = $state("27");

  // ----------------------------
  // LOGIN ORCHESTRATOR
  // ----------------------------
  async function handleLogin({ email, password }: { email: string; password: string }) {
    loading = true;

    // 1. Validación local (sync)
    const validation = validateCredentials(email, password);

    if (!validation.ok) {
      toast.error(validation.message, { icon: "✕" });
      loading = false;
      return;
    }

    // 2. Llamada a lógica de negocio (async)
    const result = await loginUser(email, password);

    if (!result.ok) {
      toast.error(parseAuthError({ message: result.message, code: result.code }).message, { icon: "✕" });
      loading = false;
      return;
    }

    // 3. Éxito → Guardar usuario en store
    setAuth(result.user);
    formRef?.reset();

    toast.success("Sesión iniciada correctamente", { icon: "✓" });
    loading = false;
  }

  // ----------------------------
  // Panel de inspección
  // ----------------------------
  function handleKeyPress(event: KeyboardEvent, handler: () => void): void {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handler();
    }
  }

  function closeInspectionPanel(): void {
    $inspectionPanel.visible = false;
  }

  function openSupabaseTest(): void {
    openTab({
      componentKey: 'supabase-test',
      title: 'Prueba Supabase',
      id: 'supabase-test'
    });
  }
</script>

{#if !$isAuthenticated}
  <!-- PANTALLA LOGIN -->
  <LoginForm bind:this={formRef} {loading} onSubmit={handleLogin} />
{:else}
  <!-- APP PRINCIPAL -->
  <div class="h-full bg-[#1e1e1e] relative">

    <!-- Botón temporal -->
    <button
      onclick={openSupabaseTest}
      class="absolute top-2 right-2 z-50 px-3 py-1.5 bg-[#007acc] text-white text-xs rounded hover:bg-[#005a9e]"
    >
      🧪 Probar Supabase
    </button>

    <Splitpanes horizontal class="default-theme">
      <!-- Contenido -->
      <Pane minSize={30} size={$inspectionPanel.visible ? 70 : 100}>
        <div class="h-full bg-[#1e1e1e]">
          <Tabs tabs={$tabsStore} />
        </div>
      </Pane>

      <!-- Panel inspección -->
      {#if $inspectionPanel.visible}
        <Pane minSize={20} size={30}>
          <div class="flex h-full flex-col bg-[#252526]">
            
            <!-- Header -->
            <div class="flex items-center justify-between border-b border-[#3c3c3c] bg-[#2d2d2d] px-3 py-2">
              <h4 class="text-xs font-semibold uppercase tracking-wide text-gray-300">
                Inspección
              </h4>

              <button
                class="flex items-center justify-center rounded p-1 text-gray-300 hover:bg-[#3c3c3c]"
                onclick={closeInspectionPanel}
                onkeydown={(e) => handleKeyPress(e, closeInspectionPanel)}
                type="button"
                title="Cerrar panel de inspección"
              >
                <ChevronDown size={16} />
              </button>
            </div>

            <!-- Contenido -->
            <div class="flex-1 overflow-y-auto p-3 text-sm text-gray-300">
              {inspectionContent}

              <div class="mt-3 flex flex-col gap-2">

                <div class="flex items-center justify-between border-b border-[#3c3c3c] py-1.5">
                  <span class="text-xs text-gray-500">Estado:</span>
                  <span class="text-xs font-semibold text-green-500">Conectado</span>
                </div>

                <div class="flex items-center justify-between border-b border-[#3c3c3c] py-1.5">
                  <span class="text-xs text-gray-500">Última actualización:</span>
                  <span class="text-xs font-semibold">{new Date().toLocaleTimeString()}</span>
                </div>

                <div class="flex items-center justify-between border-b border-[#3c3c3c] py-1.5">
                  <span class="text-xs text-gray-500">Registros hoy:</span>
                  <span class="text-xs font-semibold">1,247</span>
                </div>

              </div>
            </div>

          </div>
        </Pane>
      {/if}

    </Splitpanes>
  </div>
{/if}

<style>
  :global(.splitpanes__pane) { background: transparent; }
  :global(.splitpanes__splitter) { background: #2d2d2d; border: none; }
  :global(.splitpanes__splitter:hover) { background: #3c3c3c; }

  :global(.splitpanes--horizontal .splitpanes__splitter) {
    min-height: 6px;
    border-top: 1px solid #3c3c3c;
    border-bottom: 1px solid #3c3c3c;
  }
</style>
