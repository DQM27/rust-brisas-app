<script lang="ts">
  import { preventDefault } from "svelte/legacy";
  import { LoginSchema } from "$lib/schemas/userSchema";
  import type { ZodIssue } from "zod";

  interface Props {
    loading?: boolean;
    isDemoMode?: boolean;
    onSubmit: (data: { email: string; password: string }) => void;
  }

  let { loading = false, isDemoMode = false, onSubmit }: Props = $props();

  let email = $state("");
  let password = $state("");
  let errors = $state<Record<string, string>>({});

  // Usuarios demo con físicos famosos
  const demoUsers = [
    { email: "marie.curie@demo.com", password: "demo123", role: "Supervisora", name: "Marie Curie", icon: "👩‍🔬" },
    { email: "albert.einstein@demo.com", password: "demo123", role: "Guardia", name: "Albert Einstein", icon: "👨‍🔬" },
    { email: "richard.feynman@demo.com", password: "demo123", role: "Guardia", name: "Richard Feynman", icon: "🧑‍🔬" },
  ];

  function handleSubmit() {
    errors = {};

    // 1. Validar localmente con Zod
    const result = LoginSchema.safeParse({ email, password });

    if (!result.success) {
      const newErrors: Record<string, string> = {};
      result.error.issues.forEach((issue: ZodIssue) => {
        if (issue.path[0]) {
          newErrors[String(issue.path[0])] = issue.message;
        }
      });
      errors = newErrors;
      return;
    }

    // 2. Enviar si es válido
    onSubmit({ email, password });
  }

  function quickLogin(userEmail: string, userPassword: string) {
    email = userEmail;
    password = userPassword;
    onSubmit({ email: userEmail, password: userPassword });
  }

  export function reset() {
    email = "";
    password = "";
    errors = {};
  }
</script>

<div class="w-full max-w-md">
  <form
    onsubmit={preventDefault(handleSubmit)}
    class="flex flex-col gap-6 rounded-lg bg-surface-2 p-8 shadow-xl border border-surface-tertiary"
  >
    <div class="text-center">
      <h1 class="text-2xl font-bold text-primary">Bienvenido</h1>
      <p class="text-sm text-tertiary mt-1">Inicia sesión en Brisas App</p>
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
        <!-- Opcional: Link a recuperar contraseña -->
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

    <!-- Submit -->
    <button
      type="submit"
      disabled={loading}
      class="mt-2 w-full rounded bg-accent px-4 py-2.5 font-medium text-white transition-all hover:bg-accent-hover hover:shadow-lg hover:shadow-accent/20 disabled:cursor-not-allowed disabled:opacity-60 active:scale-[0.98]"
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
          Procesando...
        </span>
      {:else}
        Ingresar
      {/if}
    </button>
  </form>

  <!-- Demo Quick Login Buttons (solo visible en modo demo) -->
  {#if isDemoMode}
    <div class="mt-6 rounded-lg bg-amber-500/10 border border-amber-500/30 p-4">
      <div class="flex items-center gap-2 mb-3">
        <span class="text-amber-500 text-lg">⚡</span>
        <span class="text-sm font-medium text-amber-400">Acceso Rápido Demo</span>
      </div>
      <div class="flex flex-col gap-2">
        {#each demoUsers as user}
          <button
            type="button"
            onclick={() => quickLogin(user.email, user.password)}
            disabled={loading}
            class="flex items-center gap-3 w-full rounded bg-surface-1 px-3 py-2 text-left text-sm transition-all hover:bg-surface-2 hover:shadow-md disabled:opacity-60 border border-surface-tertiary"
          >
            <span class="text-xl">{user.icon}</span>
            <div class="flex-1">
              <div class="font-medium text-primary">{user.name}</div>
              <div class="text-xs text-tertiary">{user.role}</div>
            </div>
            <span class="text-xs text-accent">→</span>
          </button>
        {/each}
      </div>
      <p class="text-xs text-tertiary mt-3 text-center">Contraseña: demo123</p>
    </div>
  {/if}
</div>
