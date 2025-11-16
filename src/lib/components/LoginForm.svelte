<script lang="ts">
  interface Props {
    loading?: boolean;
    onSubmit: (data: { email: string; password: string }) => void;
  }

  let { loading = false, onSubmit }: Props = $props();

  let email = $state('');
  let password = $state('');

  function handleSubmit(event: Event) {
    event.preventDefault(); // ✅ Esto faltaba
    onSubmit({ email, password });
  }

  export function reset() {
    email = '';
    password = '';
  }
</script>

<div class="flex h-screen w-full items-center justify-center bg-[#1e1e1e]">
  <form 
    onsubmit={handleSubmit} 
    class="flex w-full max-w-md flex-col gap-4 rounded-lg bg-[#252526] p-8 shadow-xl"
  >
    <h1 class="mb-2 text-center text-2xl font-semibold text-gray-200">
      Iniciar Sesión
    </h1>

    <div class="flex flex-col gap-1">
      <label for="email" class="text-sm font-medium text-gray-300">
        Email
      </label>
      <input
        id="email"
        type="email"
        bind:value={email}
        placeholder="correo@ejemplo.com"
        disabled={loading}
        required
        class="rounded border border-[#3c3c3c] bg-[#1e1e1e] px-3 py-2 text-gray-200 
               placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none 
               focus:ring-1 focus:ring-[#007acc] disabled:opacity-60"
      />
    </div>

    <div class="flex flex-col gap-1">
      <label for="password" class="text-sm font-medium text-gray-300">
        Contraseña
      </label>
      <input
        id="password"
        type="password"
        bind:value={password}
        placeholder="••••••••"
        disabled={loading}
        required
        class="rounded border border-[#3c3c3c] bg-[#1e1e1e] px-3 py-2 text-gray-200 
               placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none 
               focus:ring-1 focus:ring-[#007acc] disabled:opacity-60"
      />
    </div>

    <button
      type="submit"
      disabled={loading}
      class="mt-2 rounded bg-[#007acc] px-4 py-2.5 font-medium text-white 
             transition-colors hover:bg-[#005a9e] disabled:cursor-not-allowed 
             disabled:opacity-60"
    >
      {loading ? 'Procesando...' : 'Entrar'}
    </button>
  </form>
</div>