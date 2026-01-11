<script lang="ts">
  import { Shield, TriangleAlert, Lock, Eye, EyeOff, X } from "lucide-svelte";
  import { fade, scale } from "svelte/transition";
  import { currentUser } from "$lib/stores/auth";

  interface Props {
    isOpen: boolean;
    onConfirm: (password: string) => void;
    onCancel: () => void;
  }

  let { isOpen, onConfirm, onCancel }: Props = $props();

  let password = $state("");
  let showPassword = $state(false);
  let inputRef = $state<HTMLInputElement>();

  $effect(() => {
    if (isOpen && inputRef) {
      inputRef.focus();
    }
  });

  function handleSubmit() {
    if (password) {
      onConfirm(password);
      password = ""; // Reset
    }
  }

  function handleCancel() {
    password = "";
    onCancel();
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
    transition:fade={{ duration: 200 }}
  >
    <div
      class="w-full max-w-[480px] bg-[#0d1117] rounded-xl shadow-2xl border border-white/10 overflow-hidden relative"
      transition:scale={{ duration: 200, start: 0.95 }}
    >
      <!-- Background Glow Effect -->
      <div
        class="absolute top-0 left-1/2 -translate-x-1/2 w-full h-32 bg-orange-500/10 blur-[60px] rounded-full pointer-events-none"
      ></div>

      <div class="p-8 relative z-10">
        <!-- Header -->
        <div class="flex items-start gap-4 mb-6">
          <div
            class="flex-none p-3 rounded-2xl bg-gradient-to-br from-orange-500/20 to-orange-600/5 ring-1 ring-orange-500/20 shadow-lg shadow-orange-900/20"
          >
            <Shield class="w-8 h-8 text-orange-400" strokeWidth={1.5} />
          </div>
          <div>
            <h2 class="text-xl font-bold text-white leading-tight">
              Desactivar Tu Cuenta
            </h2>
            <p class="text-gray-400 text-sm mt-1">
              Acción que requiere verificación
            </p>
          </div>
        </div>

        <!-- Warning Alert -->
        <div
          class="bg-yellow-900/20 border border-yellow-700/30 rounded-lg p-4 mb-6 flex items-start gap-3"
        >
          <TriangleAlert
            class="w-5 h-5 text-yellow-500 flex-none relative top-0.5"
          />
          <p class="text-sm text-yellow-200/80 leading-relaxed">
            <span class="font-semibold text-yellow-400">ADVERTENCIA:</span> Estás
            a punto de desactivar tu propia cuenta. Una vez desactivada, NO podrás
            iniciar sesión hasta que otro administrador te reactive. ¿Estás seguro
            de que deseas continuar?
          </p>
        </div>

        <!-- User Card -->
        {#if $currentUser}
          <div
            class="bg-white/5 border border-white/5 rounded-lg p-4 mb-6 flex items-center gap-4"
          >
            <div
              class="w-10 h-10 rounded-full bg-blue-600 flex items-center justify-center text-white font-bold text-lg"
            >
              {$currentUser.nombre[0].toUpperCase()}
            </div>
            <div class="flex-1 min-w-0">
              <p class="text-sm font-medium text-white truncate">
                {$currentUser.nombre}
                {$currentUser.apellido}
              </p>
              <p class="text-xs text-gray-500 truncate">
                {$currentUser.email}
              </p>
            </div>
          </div>
        {/if}

        <form
          onsubmit={(e) => {
            e.preventDefault();
            handleSubmit();
          }}
          class="space-y-6"
        >
          <!-- Password Input -->
          <div class="space-y-2">
            <label
              for="admin-pass"
              class="block text-sm font-medium text-gray-300"
            >
              Ingresa tu contraseña para confirmar
            </label>
            <div class="relative group">
              <div
                class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none"
              >
                <Lock
                  class="h-5 w-5 text-gray-500 group-focus-within:text-blue-500 transition-colors"
                />
              </div>
              <input
                bind:this={inputRef}
                id="admin-pass"
                type={showPassword ? "text" : "password"}
                bind:value={password}
                class="block w-full pl-10 pr-10 py-3 bg-black/40 border border-white/10 rounded-lg text-white placeholder-gray-600 focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all text-sm tracking-wide sm:text-base outline-none"
                placeholder="••••••••"
              />
              <button
                type="button"
                class="absolute inset-y-0 right-0 pr-3 flex items-center cursor-pointer text-gray-500 hover:text-gray-300 transition-colors"
                onclick={() => (showPassword = !showPassword)}
              >
                {#if showPassword}
                  <EyeOff class="h-5 w-5" />
                {:else}
                  <Eye class="h-5 w-5" />
                {/if}
              </button>
            </div>
          </div>

          <!-- Actions -->
          <div class="grid grid-cols-2 gap-3">
            <button
              type="button"
              onclick={handleCancel}
              class="px-4 py-2.5 rounded-lg border border-white/10 text-gray-300 font-medium hover:bg-white/5 hover:text-white transition-all text-sm"
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={!password}
              class="px-4 py-2.5 rounded-lg bg-orange-700 hover:bg-orange-600 text-white font-medium shadow-lg shadow-orange-900/20 disabled:opacity-50 disabled:cursor-not-allowed transition-all text-sm"
            >
              Sí, Desactivar Mi Cuenta
            </button>
          </div>

          <div
            class="flex items-center justify-center gap-2 text-xs text-gray-600 mt-2"
          >
            <Lock class="w-3 h-3" />
            Tu contraseña se verifica localmente y no se almacena
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}
