<script lang="ts">
  import { generalSettings } from "$lib/stores/settingsStore";
  import { scale } from "svelte/transition";
  import { Check, X, Layout, Type, Power, FlaskConical } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { toast } from "svelte-5-french-toast";

  // Estado del modo demo (se carga desde el backend)
  let showDemoMode = $state(false);
  let loadingDemo = $state(false);

  // Cargar configuración al montar
  onMount(async () => {
    try {
      const config = await invoke<any>("get_app_config");
      showDemoMode = config?.setup?.show_demo_mode ?? false;
    } catch (e) {
      console.warn("No se pudo cargar config de demo:", e);
    }
  });

  async function toggleDemoMode() {
    loadingDemo = true;
    try {
      const newValue = !showDemoMode;
      await invoke("toggle_demo_mode", { enabled: newValue });
      showDemoMode = newValue;
      toast.success(
        newValue ? "Modo demo habilitado" : "Modo demo deshabilitado",
        { icon: newValue ? "🧪" : "🔒" },
      );
    } catch (e: any) {
      console.error("Error toggling demo mode:", e);
      toast.error("Error al cambiar modo demo");
    }
    loadingDemo = false;
  }

  // ==========================================================================
  // Toggle Component (reusable)
  // ==========================================================================
</script>

<!-- Reusable Toggle Switch -->
{#snippet toggleSwitch(
  checked: boolean,
  onChange: () => void,
  srLabel: string,
  disabled: boolean = false,
)}
  <button
    onclick={onChange}
    {disabled}
    class="relative inline-flex h-7 w-12 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed
    {checked ? 'bg-green-500' : 'bg-gray-300 dark:bg-gray-700'}"
  >
    <span class="sr-only">{srLabel}</span>
    <span
      class="pointer-events-none inline-flex h-6 w-6 items-center justify-center transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out
      {checked ? 'translate-x-5' : 'translate-x-0'}"
    >
      {#if checked}
        <Check size={12} class="text-green-600" strokeWidth={3} />
      {:else}
        <X size={12} class="text-gray-400" strokeWidth={3} />
      {/if}
    </span>
  </button>
{/snippet}

<!-- Setting Row -->
{#snippet settingRow(
  Icon: any,
  iconBg: string,
  iconColor: string,
  label: string,
  checked: boolean,
  onChange: () => void,
  disabled: boolean = false,
)}
  <div class="flex items-center justify-between py-3">
    <div class="flex items-center gap-3">
      <div class="p-2 rounded-md {iconBg}">
        <Icon size={18} class={iconColor} />
      </div>
      <span class="text-secondary font-medium">{label}</span>
    </div>
    {@render toggleSwitch(checked, onChange, label, disabled)}
  </div>
{/snippet}

<div
  class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
  in:scale={{ duration: 300, start: 0.95 }}
>
  <div class="mb-6">
    <h2 class="text-2xl font-bold text-primary">Ajustes Generales</h2>
    <p class="text-secondary mt-1">
      Configura las preferencias del sistema e interfaz de usuario.
    </p>
  </div>

  <div class="grid gap-4 max-w-3xl pb-8">
    <!-- ================================================================== -->
    <!-- UI ELEMENTS CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5">
      <div class="flex items-center gap-4 mb-4">
        <div
          class="p-3 rounded-lg bg-violet-100 text-violet-600 dark:bg-violet-900/30 dark:text-violet-400"
        >
          <Layout size={22} />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-primary">Interfaz</h3>
          <p class="text-sm text-secondary">
            Configura los elementos de la interfaz.
          </p>
        </div>
      </div>

      <div class="divide-y divide-emphasis">
        {@render settingRow(
          Type,
          "bg-cyan-50 dark:bg-cyan-900/20",
          "text-cyan-500",
          "Texto de Bienvenida",
          $generalSettings.showWelcomeText,
          () => generalSettings.toggleWelcomeText(),
        )}

        {@render settingRow(
          Layout,
          "bg-violet-50 dark:bg-violet-900/20",
          "text-violet-500",
          "Tarjetas de Módulos",
          $generalSettings.showWelcomeCards,
          () => generalSettings.toggleCards(),
        )}
      </div>
    </div>

    <!-- ================================================================== -->
    <!-- SYSTEM CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5">
      <div class="flex items-center gap-4 mb-4">
        <div
          class="p-3 rounded-lg bg-slate-100 text-slate-600 dark:bg-slate-800 dark:text-slate-400"
        >
          <Power size={22} />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-primary">Sistema</h3>
          <p class="text-sm text-secondary">
            Configuraciones de arranque y sistema.
          </p>
        </div>
      </div>

      <div class="divide-y divide-emphasis">
        {@render settingRow(
          Power,
          "bg-gray-50 dark:bg-gray-900/20",
          "text-gray-500",
          "Deshabilitar Setup Wizard",
          $generalSettings.disableSetupWizard,
          () => generalSettings.toggleSetupWizard(),
        )}

        <!-- Modo Demo Toggle -->
        {@render settingRow(
          FlaskConical,
          "bg-amber-50 dark:bg-amber-900/20",
          "text-amber-500",
          "Mostrar Modo Demo en Login",
          showDemoMode,
          toggleDemoMode,
          loadingDemo,
        )}
      </div>

      <!-- Nota de seguridad para modo demo -->
      {#if showDemoMode}
        <div
          class="mt-3 flex items-start gap-2 p-3 rounded-md bg-amber-500/10 border border-amber-500/20"
        >
          <span class="text-amber-500 text-lg">⚠️</span>
          <p class="text-xs text-amber-600 dark:text-amber-400">
            El modo demo está habilitado. Se mostrará el enlace "¿Modo Demo?" en
            la pantalla de login. Desactívalo antes de desplegar a producción.
          </p>
        </div>
      {/if}
    </div>

    <!-- ================================================================== -->
    <!-- RESET BUTTON -->
    <!-- ================================================================== -->
    <div class="flex justify-end pt-2">
      <button
        class="btn-base bg-red-100 hover:bg-red-200 dark:bg-red-900/30 dark:hover:bg-red-900/50 text-red-600 dark:text-red-400 text-sm"
        onclick={() => {
          if (
            confirm(
              "¿Restaurar todas las configuraciones a sus valores por defecto?",
            )
          ) {
            generalSettings.reset();
          }
        }}
      >
        Restaurar Todo
      </button>
    </div>
  </div>
</div>
