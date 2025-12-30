<script lang="ts">
  import { preventDefault } from "svelte/legacy";
  import { onMount } from "svelte";
  import { X, Minus, Lock } from "lucide-svelte";
  import { exitApp } from "$lib/services/keyringService";
  import { loginStore } from "$lib/stores/loginStore.svelte";
  import { DEMO_USERS } from "$lib/config/demoUsers";
  import { validateLoginForm } from "$lib/logic/auth/loginValidation";

  interface Props {
    loading?: boolean;
    showDemoLink?: boolean;
    onSubmit: (data: { email: string; password: string }) => void;
    onDemoLogin?: (email: string) => void;
  }

  let {
    loading = false,
    showDemoLink = false,
    onSubmit,
    onDemoLogin,
  }: Props = $props();

  let email = $state("");
  let password = $state("");
  let errors = $state<Record<string, string>>({});
  let showDemoModal = $state(false);
  let rememberMe = $state(false);

  onMount(() => {
    // Load remembered email from store
    if (loginStore.hasRememberedEmail) {
      email = loginStore.rememberedEmail;
      rememberMe = true;
    }
  });

  function handleSubmit() {
    // 1. Validate using extracted logic
    const result = validateLoginForm(email, password);

    if (!result.valid) {
      errors = result.errors;
      return;
    }

    errors = {};

    // 2. Persist rememberedEmail using loginStore
    if (rememberMe) {
      loginStore.setRememberedEmail(email);
    } else {
      loginStore.clearRememberedEmail();
    }

    onSubmit({ email, password });
  }

  function handleDemoClick() {
    showDemoModal = true;
  }

  function handleDemoUserSelect(userEmail: string) {
    showDemoModal = false;
    if (onDemoLogin) {
      onDemoLogin(userEmail);
    }
  }

  function closeDemoModal() {
    showDemoModal = false;
  }

  export async function reset() {
    // Reload from store
    await loginStore.reload();
    email = loginStore.rememberedEmail;
    password = "";
    errors = {};
    rememberMe = loginStore.hasRememberedEmail;
  }

  async function minimizeWindow() {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    await getCurrentWindow().minimize();
  }
</script>

<div class="w-full h-full flex flex-col justify-center p-8">
  <form onsubmit={preventDefault(handleSubmit)} class="flex flex-col gap-6">
    <div class="text-center">
      <h1 class="text-3xl font-bold text-primary">Bienvenido</h1>
      <p class="text-sm text-tertiary mt-2">Inicia sesión en Mega Brisas</p>
    </div>

    <!-- Email -->
    <div class="flex flex-col gap-1.5">
      <label for="email" class="text-sm font-medium text-secondary">
        Correo Electrónico
      </label>
      <input
        id="email"
        type="email"
        bind:value={email}
        placeholder="ejemplo@correo.com"
        disabled={loading}
        class="w-full rounded border bg-surface-1 px-3 py-2 text-primary focus:outline-none focus:ring-2 focus:ring-accent transition-all {errors.email
          ? 'border-red-500 focus:border-red-500 focus:ring-red-500/20'
          : 'border-emphasis focus:border-accent'}"
      />
      {#if errors.email}
        <span class="text-xs text-red-500 animate-fade-in">{errors.email}</span>
      {/if}
    </div>

    <!-- Password -->
    <div class="flex flex-col gap-1.5">
      <div class="flex justify-between items-center">
        <label for="password" class="text-sm font-medium text-secondary">
          Contraseña
        </label>
      </div>
      <input
        id="password"
        type="password"
        bind:value={password}
        placeholder="••••••••"
        disabled={loading}
        class="w-full rounded border bg-surface-1 px-3 py-2 text-primary focus:outline-none focus:ring-2 focus:ring-accent transition-all {errors.password
          ? 'border-red-500 focus:border-red-500 focus:ring-red-500/20'
          : 'border-emphasis focus:border-accent'}"
      />
      {#if errors.password}
        <span class="text-xs text-red-500 animate-fade-in"
          >{errors.password}</span
        >
      {/if}
    </div>

    <!-- Options -->
    <div class="flex items-center justify-center">
      <label
        class="flex items-center gap-2 cursor-pointer text-sm text-secondary hover:text-primary transition-colors select-none"
      >
        <input
          type="checkbox"
          bind:checked={rememberMe}
          disabled={loading}
          class="rounded border-surface-tertiary text-accent focus:ring-accent w-4 h-4 cursor-pointer"
        />
        Recordar usuario
      </label>
    </div>

    <!-- Acciones -->
    <div class="flex gap-3 mt-2">
      <button
        type="button"
        onclick={exitApp}
        class="flex-1 py-2.5 px-4 rounded-lg border border-surface-tertiary text-secondary font-medium hover:bg-surface-3 transition-colors text-sm"
      >
        Cancelar
      </button>
      <button
        type="submit"
        disabled={loading}
        class="flex-1 rounded-lg bg-accent px-4 py-2.5 font-medium text-white transition-all hover:bg-accent-hover hover:shadow-lg hover:shadow-accent/20 disabled:cursor-not-allowed disabled:opacity-60 active:scale-[0.98]"
      >
        {#if loading}
          <span class="flex items-center justify-center gap-2">
            <svg
              class="h-4 w-4 animate-spin text-white"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
            </svg>
            ...
          </span>
        {:else}
          Ingresar
        {/if}
      </button>
    </div>

    <!-- Link de Demo (solo visible si está habilitado) -->
    {#if showDemoLink}
      <div class="text-center">
        <button
          type="button"
          onclick={handleDemoClick}
          disabled={loading}
          class="text-sm text-amber-500 hover:text-amber-400 hover:underline transition-colors disabled:opacity-50"
        >
          ¿Modo Demo?
        </button>
      </div>
    {/if}
  </form>
</div>

<!-- Modal de Demo -->
{#if showDemoModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm animate-fade-in"
    onclick={closeDemoModal}
    onkeydown={(e) => e.key === "Escape" && closeDemoModal()}
    role="dialog"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="w-full max-w-sm mx-4 rounded-lg bg-surface-2 p-6 shadow-2xl border border-surface-tertiary animate-scale-in"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <!-- Header -->
      <div class="flex items-center gap-3 mb-4">
        <span class="text-2xl">🧪</span>
        <div>
          <h2 class="text-lg font-bold text-primary">Modo Demo</h2>
          <p class="text-xs text-tertiary">
            Selecciona un usuario para continuar
          </p>
        </div>
      </div>

      <!-- Users -->
      <div class="flex flex-col gap-2 mb-4">
        {#each DEMO_USERS as user}
          <button
            type="button"
            onclick={() => handleDemoUserSelect(user.email)}
            class="flex items-center gap-3 w-full rounded-lg bg-surface-1 px-4 py-3 text-left transition-all hover:bg-surface-3 hover:shadow-md border border-surface-tertiary hover:border-accent/50"
          >
            <span class="text-2xl">{user.icon}</span>
            <div class="flex-1">
              <div class="font-medium text-primary">{user.name}</div>
              <div class="text-xs text-tertiary">{user.role}</div>
            </div>
            <span class="text-accent">→</span>
          </button>
        {/each}
      </div>

      <!-- Footer -->
      <div
        class="flex justify-between items-center pt-3 border-t border-surface-tertiary"
      >
        <p class="text-xs text-tertiary">
          <span class="text-amber-500">⚡</span> Carga datos de prueba automáticamente
        </p>
        <button
          type="button"
          onclick={closeDemoModal}
          class="text-sm text-tertiary hover:text-primary transition-colors"
        >
          Cancelar
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes scale-in {
    from {
      transform: scale(0.95);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }
  .animate-scale-in {
    animation: scale-in 0.2s ease-out;
  }
</style>
