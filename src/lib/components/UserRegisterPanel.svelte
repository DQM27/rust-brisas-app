<script lang="ts">
  import type { UserRole } from '$lib/types/user';

  interface Props {
    loading?: boolean;
    onSubmit: (data: {
      email: string;
      password: string;
      nombre: string;
      apellido: string;
      role: UserRole;
    }) => void;
  }

  let { loading = false, onSubmit }: Props = $props();

  let email = $state('');
  let password = $state('');
  let nombre = $state('');
  let apellido = $state('');
  let role = $state<UserRole>('guardia');

  function handleSubmit(event: Event) {
    event.preventDefault();
    onSubmit({ email, password, nombre, apellido, role });
  }

  export function reset() {
    email = '';
    password = '';
    nombre = '';
    apellido = '';
    role = 'guardia';
  }

  const isFormValid = $derived(
    email.trim() !== '' &&
    password.trim() !== '' &&
    nombre.trim() !== '' &&
    apellido.trim() !== ''
  );
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <div class="w-full max-w-2xl rounded-lg bg-[#252526] p-8 shadow-xl">
    <h2 class="border-b border-[#007acc] pb-3 text-2xl font-semibold text-gray-200">
      Registrar Nuevo Usuario
    </h2>

    <form onsubmit={handleSubmit} class="mt-6 space-y-5">
      <!-- Email -->
      <div class="space-y-2">
        <label for="email" class="block text-sm font-medium text-gray-300">Email</label>
        <input
          id="email"
          type="email"
          bind:value={email}
          placeholder="correo@ejemplo.com"
          disabled={loading}
          required
          class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white 
                 placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
        />
      </div>

      <!-- Contraseña -->
      <div class="space-y-2">
        <label for="password" class="block text-sm font-medium text-gray-300">Contraseña</label>
        <input
          id="password"
          type="password"
          bind:value={password}
          placeholder="••••••••"
          disabled={loading}
          required
          class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white 
                 placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
        />
      </div>

      <!-- Nombre y Apellido -->
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div class="space-y-2">
          <label for="nombre" class="block text-sm font-medium text-gray-300">Nombre</label>
          <input
            id="nombre"
            type="text"
            bind:value={nombre}
            placeholder="Juan"
            disabled={loading}
            required
            class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white 
                   placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
          />
        </div>

        <div class="space-y-2">
          <label for="apellido" class="block text-sm font-medium text-gray-300">Apellido</label>
          <input
            id="apellido"
            type="text"
            bind:value={apellido}
            placeholder="Pérez"
            disabled={loading}
            required
            class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white 
                   placeholder:text-gray-500 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
          />
        </div>
      </div>

      <!-- Rol -->
      <div class="space-y-2">
        <label for="role" class="block text-sm font-medium text-gray-300">Rol</label>
        <select
          id="role"
          bind:value={role}
          disabled={loading}
          class="w-full rounded border border-[#444] bg-[#1e1e1e] px-3 py-2 text-sm text-white 
                 focus:border-[#007acc] focus:outline-none focus:ring-2 focus:ring-[#007acc] disabled:opacity-60"
        >
          <option value="guardia">Guardia</option>
          <option value="supervisor">Supervisor</option>
          <option value="admin">Administrador</option>
        </select>
      </div>

      <!-- Botón -->
      <button
        type="submit"
        disabled={loading || !isFormValid}
        class="mt-6 w-full rounded bg-[#007acc] px-4 py-2.5 font-medium text-white 
               transition-colors hover:bg-[#005a9e] disabled:cursor-not-allowed disabled:opacity-60"
      >
        {loading ? 'Procesando...' : 'Registrar Usuario'}
      </button>
    </form>
  </div>
</div>
