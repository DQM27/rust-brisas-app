<script lang="ts">
  import { Clock, Shield, Moon, Power, RotateCcw } from "lucide-svelte";
  import { sessionSettings } from "$lib/stores/sessionSettingsStore";

  // Local state for editing
  let screensaverMinutes = $state($sessionSettings.screensaverTimeoutMinutes);
  let completeMinutes = $state($sessionSettings.completeTimeoutMinutes);

  // Update local state when store changes (external changes)
  $effect(() => {
    screensaverMinutes = $sessionSettings.screensaverTimeoutMinutes;
    completeMinutes = $sessionSettings.completeTimeoutMinutes;
  });

  // Save changes to store (debounced by the store itself)
  function updateScreensaverTimeout() {
    sessionSettings.setScreensaverTimeout(screensaverMinutes);
  }

  function updateCompleteTimeout() {
    sessionSettings.setCompleteTimeout(completeMinutes);
  }

  function handleReset() {
    if (confirm("¿Restaurar configuración de sesión a valores por defecto?")) {
      sessionSettings.reset();
      // Local state will update via $effect
    }
  }

  // Format minutes for display
  function formatTime(minutes: number): string {
    if (minutes < 60) {
      return `${minutes} min`;
    }
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    if (mins === 0) {
      return `${hours} h`;
    }
    return `${hours} h ${mins} min`;
  }
</script>

<div class="flex flex-col gap-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h3 class="text-lg font-semibold text-primary flex items-center gap-2">
        <Shield size={20} class="text-accent" />
        Seguridad de Sesión
      </h3>
      <p class="text-sm text-tertiary mt-1">
        Configura timeouts de inactividad para proteger tu sesión
      </p>
    </div>
    <button
      type="button"
      onclick={handleReset}
      class="flex items-center gap-2 px-3 py-2 text-sm rounded-lg border border-surface-tertiary text-secondary hover:bg-surface-3 transition-colors"
      title="Restaurar valores por defecto"
    >
      <RotateCcw size={16} />
      Restaurar
    </button>
  </div>

  <!-- App Lock Settings (cuando app pierde foco) -->
  <div class="rounded-lg border border-emphasis bg-surface-1 p-4">
    <div class="flex items-start gap-3 mb-4">
      <div
        class="flex items-center justify-center w-10 h-10 rounded-lg bg-amber-500/10"
      >
        <Shield size={20} class="text-amber-500" />
      </div>
      <div class="flex-1">
        <div class="flex items-center justify-between mb-1">
          <h4 class="font-medium text-primary">
            Bloqueo por Inactividad Local (App)
          </h4>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={$sessionSettings.enableAppLock}
              onchange={() => sessionSettings.toggleAppLock()}
              class="sr-only peer"
            />
            <div
              class="w-11 h-6 bg-gray-300 rounded-full peer peer-checked:bg-green-500 peer-focus:ring-2 peer-focus:ring-green-500 transition-colors after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-transform peer-checked:after:translate-x-full"
            ></div>
          </label>
        </div>
        <p class="text-sm text-secondary mb-3">
          Bloquea la app por falta de interacción interna (incluso si está
          visible)
        </p>

        {#if $sessionSettings.enableAppLock}
          <div class="space-y-3 ml-1 pl-3 border-l-2 border-amber-500/30">
            <!-- Timeout Duration -->
            <div>
              <label
                for="app-lock-timeout"
                class="block text-sm font-medium text-secondary mb-2"
              >
                Tiempo sin usar la app: <span
                  class="text-amber-500 font-semibold"
                  >{formatTime($sessionSettings.appLockTimeoutMinutes)}</span
                >
              </label>
              <div class="flex items-center gap-3">
                <input
                  id="app-lock-timeout"
                  type="range"
                  min="1"
                  max="120"
                  step="1"
                  value={$sessionSettings.appLockTimeoutMinutes}
                  oninput={(e) =>
                    sessionSettings.setAppLockTimeout(
                      Number((e.target as HTMLInputElement).value),
                    )}
                  class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-amber-500"
                />
                <input
                  type="number"
                  min="1"
                  max="120"
                  value={$sessionSettings.appLockTimeoutMinutes}
                  onchange={(e) =>
                    sessionSettings.setAppLockTimeout(
                      Number((e.target as HTMLInputElement).value),
                    )}
                  class="w-20 px-2 py-1 text-sm rounded border border-emphasis bg-surface-2 text-primary focus:outline-none focus:ring-2 focus:ring-amber-500"
                />
              </div>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Screensaver Settings (cuando PC está inactiva) -->
  <div class="rounded-lg border border-emphasis bg-surface-1 p-4">
    <div class="flex items-start gap-3 mb-4">
      <div
        class="flex items-center justify-center w-10 h-10 rounded-lg bg-blue-500/10"
      >
        <Moon size={20} class="text-blue-500" />
      </div>
      <div class="flex-1">
        <div class="flex items-center justify-between mb-1">
          <h4 class="font-medium text-primary">
            Protector de Pantalla Interactivo
          </h4>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={$sessionSettings.enableScreensaver}
              onchange={() => sessionSettings.toggleScreensaver()}
              class="sr-only peer"
            />
            <div
              class="w-11 h-6 bg-gray-300 rounded-full peer peer-checked:bg-green-500 peer-focus:ring-2 peer-focus:ring-green-500 transition-colors after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-transform peer-checked:after:translate-x-full"
            ></div>
          </label>
        </div>
        <p class="text-sm text-secondary mb-3">
          Muestra el panel de bienvenida en pantalla completa después de
          inactividad
        </p>

        {#if $sessionSettings.enableScreensaver}
          <div class="space-y-3 ml-1 pl-3 border-l-2 border-emphasis">
            <!-- Timeout Duration -->
            <div>
              <label
                for="screensaver-timeout"
                class="block text-sm font-medium text-secondary mb-2"
              >
                Tiempo de inactividad: <span class="text-accent font-semibold"
                  >{formatTime(screensaverMinutes)}</span
                >
              </label>
              <div class="flex items-center gap-3">
                <input
                  id="screensaver-timeout"
                  type="range"
                  min="1"
                  max="120"
                  step="1"
                  bind:value={screensaverMinutes}
                  oninput={updateScreensaverTimeout}
                  class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-accent"
                />
                <input
                  type="number"
                  min="1"
                  max="120"
                  bind:value={screensaverMinutes}
                  onchange={updateScreensaverTimeout}
                  class="w-20 px-2 py-1 text-sm rounded border border-emphasis bg-surface-2 text-primary focus:outline-none focus:ring-2 focus:ring-accent"
                />
              </div>
            </div>

            <!-- Password Requirement -->
            <div
              class="flex items-center justify-between p-3 rounded-lg bg-surface-2"
            >
              <div class="flex items-center gap-2">
                <Shield size={16} class="text-accent" />
                <div>
                  <div class="text-sm font-medium text-primary">
                    Requiere contraseña para salir
                  </div>
                  <div class="text-xs text-tertiary">
                    {$sessionSettings.screensaverRequiresPassword
                      ? "Solicita contraseña al interactuar"
                      : "Sale al interactuar (menos seguro)"}
                  </div>
                </div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={$sessionSettings.screensaverRequiresPassword}
                  onchange={() => sessionSettings.toggleScreensaverPassword()}
                  class="sr-only peer"
                />
                <div
                  class="w-11 h-6 bg-gray-300 rounded-full peer peer-checked:bg-green-500 peer-focus:ring-2 peer-focus:ring-green-500 transition-colors after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-transform peer-checked:after:translate-x-full"
                ></div>
              </label>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Complete Logout Settings -->
  <div class="rounded-lg border border-emphasis bg-surface-1 p-4">
    <div class="flex items-start gap-3 mb-4">
      <div
        class="flex items-center justify-center w-10 h-10 rounded-lg bg-red-500/10"
      >
        <Power size={20} class="text-red-500" />
      </div>
      <div class="flex-1">
        <div class="flex items-center justify-between mb-1">
          <h4 class="font-medium text-primary">Cierre de Sesión Automático</h4>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              checked={$sessionSettings.enableCompleteTimeout}
              onchange={() => sessionSettings.toggleCompleteTimeout()}
              class="sr-only peer"
            />
            <div
              class="w-11 h-6 bg-gray-300 rounded-full peer peer-checked:bg-green-500 peer-focus:ring-2 peer-focus:ring-green-500 transition-colors after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-transform peer-checked:after:translate-x-full"
            ></div>
          </label>
        </div>
        <p class="text-sm text-secondary mb-3">
          Cierra sesión completamente y elimina todas las pestañas después de
          inactividad prolongada
        </p>

        {#if $sessionSettings.enableCompleteTimeout}
          <div class="space-y-3 ml-1 pl-3 border-l-2 border-red-500/30">
            <!-- Timeout Duration -->
            <div>
              <label
                for="complete-timeout"
                class="block text-sm font-medium text-secondary mb-2"
              >
                Tiempo de inactividad: <span class="text-red-500 font-semibold"
                  >{formatTime(completeMinutes)}</span
                >
              </label>
              <div class="flex items-center gap-3">
                <input
                  id="complete-timeout"
                  type="range"
                  min="5"
                  max="240"
                  step="5"
                  bind:value={completeMinutes}
                  oninput={updateCompleteTimeout}
                  class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-red-500"
                />
                <input
                  type="number"
                  min="5"
                  max="240"
                  step="5"
                  bind:value={completeMinutes}
                  onchange={updateCompleteTimeout}
                  class="w-20 px-2 py-1 text-sm rounded border border-emphasis bg-surface-2 text-primary focus:outline-none focus:ring-2 focus:ring-red-500"
                />
              </div>
            </div>

            <!-- Validation Warning -->
            {#if $sessionSettings.enableScreensaver && completeMinutes <= screensaverMinutes}
              <div
                class="flex items-start gap-2 p-3 rounded-lg bg-amber-500/10 border border-amber-500/20"
              >
                <svg
                  class="w-5 h-5 text-amber-500 flex-shrink-0 mt-0.5"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fill-rule="evenodd"
                    d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                    clip-rule="evenodd"
                  />
                </svg>
                <div class="text-xs text-amber-600 dark:text-amber-400">
                  El cierre de sesión debe ocurrir después del protector de
                  pantalla. Se ajustará automáticamente.
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Info Box -->
  <div class="rounded-lg border border-blue-500/20 bg-blue-500/5 p-4">
    <div class="flex items-start gap-3">
      <Clock size={20} class="text-blue-500 flex-shrink-0 mt-0.5" />
      <div class="text-sm text-secondary">
        <p class="font-medium text-primary mb-1">
          💡 Comportamiento de la sesión
        </p>
        <ul class="space-y-1 list-disc list-inside">
          <li>
            La app <strong>no recordará tu sesión</strong> al cerrarse y abrirse
            de nuevo
          </li>
          <li>
            El contador de inactividad se reinicia con cualquier interacción
            (mouse, teclado)
          </li>
          <li>
            Puedes habilitar uno, ambos, o ningún timeout según tus necesidades
          </li>
          <li>Los cambios se guardan automáticamente</li>
        </ul>
      </div>
    </div>
  </div>
</div>
