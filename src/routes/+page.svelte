<script lang="ts">
  import { isAuthenticated, login } from '$lib/stores/auth';
  import LoginForm from '$lib/components/LoginForm.svelte';
  import { auth } from '$lib/api/auth';
  import { Splitpanes, Pane } from 'svelte-splitpanes';
  import Tabs from '$lib/components/layout/Tabs.svelte';
  import { tabsStore, openTab } from '$lib/stores/tabs';  // ← Importar openTab
  import { inspectionPanel } from '$lib/stores/ui';
  import { ChevronDown } from 'lucide-svelte';
  import { toast } from 'svelte-5-french-toast';

  import type LoginFormType from '$lib/components/LoginForm.svelte';

  let loading = $state(false);
  let formRef = $state<LoginFormType>();
  let inspectionContent = $state("27");

  // ----------------------------
  // Funciones de login
  // ----------------------------
  async function handleLogin(data: { email: string; password: string }) {
    loading = true;

    try {
      const user = await auth.login(data.email, data.password);
      login(user);
      formRef?.reset();
      toast.success('Sesión iniciada correctamente', {
        duration: 3000,
        icon: '✓'
      });
    } catch (err) {
      toast.error('Credenciales inválidas. Verifica tu email y contraseña.', {
        duration: 4000,
        icon: '✕'
      });
    } finally {
      loading = false;
    }
  }

  // ----------------------------
  // Funciones de panel de inspección
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

  // ← NUEVA FUNCIÓN
  function openSupabaseTest(): void {
    openTab({
      componentKey: 'supabase-test',
      title: 'Prueba Supabase',
      id: 'supabase-test'
    });
  }
</script>

{#if !$isAuthenticated}
  <!-- Pantalla de Login -->
  <LoginForm bind:this={formRef} {loading} onSubmit={handleLogin} />
{:else}
  <!-- App Principal -->
  <div class="h-full bg-[#1e1e1e] relative">
    <!-- ← BOTÓN TEMPORAL -->
    <button
      onclick={openSupabaseTest}
      class="absolute top-2 right-2 z-50 px-3 py-1.5 bg-[#007acc] text-white text-xs rounded hover:bg-[#005a9e]"
    >
      🧪 Probar Supabase
    </button>

    <Splitpanes horizontal class="default-theme">
      <!-- Contenido principal -->
      <Pane minSize={30} size={$inspectionPanel.visible ? 70 : 100}>
        <div class="h-full bg-[#1e1e1e]">
          <Tabs tabs={$tabsStore} />
        </div>
      </Pane>

      <!-- Panel de inspección -->
      {#if $inspectionPanel.visible}
        <Pane minSize={20} size={30}>
          <div class="flex h-full flex-col bg-[#252526]">
            <!-- Header -->
            <div class="flex items-center justify-between border-b border-[#3c3c3c] bg-[#2d2d2d] px-3 py-2">
              <h4 class="text-xs font-semibold uppercase tracking-wide text-gray-300">
                Inspección
              </h4>
              <button
                class="flex items-center justify-center rounded p-1 text-gray-300 hover:bg-[#3c3c3c] focus:outline-none focus:ring-2 focus:ring-[#007acc] focus:ring-offset-1 focus:ring-offset-[#2d2d2d]"
                onclick={closeInspectionPanel}
                onkeydown={(e) => handleKeyPress(e, closeInspectionPanel)}
                type="button"
                title="Cerrar panel de inspección"
              >
                <ChevronDown size={16} />
              </button>
            </div>

            <!-- Content -->
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
  /* Splitpanes */
  :global(.splitpanes__pane) {
    background: transparent;
  }

  :global(.splitpanes__splitter) {
    background: #2d2d2d;
    border: none;
  }

  :global(.splitpanes__splitter:hover) {
    background: #3c3c3c;
  }

  :global(.splitpanes--horizontal .splitpanes__splitter) {
    min-height: 6px;
    border-top: 1px solid #3c3c3c;
    border-bottom: 1px solid #3c3c3c;
  }
</style>