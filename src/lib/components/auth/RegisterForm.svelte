<!-- src/lib/components/auth/RegisterForm.svelte -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { CreateUserInput } from '$lib/types';

  export let loading = false;

  let email = '';
  let password = '';
  let nombre = '';
  let apellido = '';
  let role: 'user' | 'admin' = 'user';

  const dispatch = createEventDispatcher<{ submit: CreateUserInput }>();

  function handleSubmit() {
    if (!email || !password || !nombre || !apellido) return;
    dispatch('submit', { email, password, nombre, apellido, role });
  }

  export function reset() {
    email = password = nombre = apellido = '';
    role = 'user';
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  <div class="form-group">
    <label for="email">Email</label>
    <input
      id="email"
      bind:value={email}
      type="email"
      placeholder="correo@ejemplo.com"
      disabled={loading}
      required
    />
  </div>

  <div class="form-group">
    <label for="password">Contraseña</label>
    <input
      id="password"
      bind:value={password}
      type="password"
      placeholder="••••••••"
      disabled={loading}
      required
    />
  </div>

  <div class="form-row">
    <div class="form-group">
      <label for="nombre">Nombre</label>
      <input
        id="nombre"
        bind:value={nombre}
        type="text"
        placeholder="Juan"
        disabled={loading}
        required
      />
    </div>

    <div class="form-group">
      <label for="apellido">Apellido</label>
      <input
        id="apellido"
        bind:value={apellido}
        type="text"
        placeholder="Pérez"
        disabled={loading}
        required
      />
    </div>
  </div>

  <div class="form-group">
    <label for="role">Rol</label>
    <select id="role" bind:value={role} disabled={loading}>
      <option value="user">Usuario</option>
      <option value="admin">Administrador</option>
    </select>
  </div>

  <button
    type="submit"
    class="btn btn-primary"
    disabled={loading || !email || !password || !nombre || !apellido}
  >
    {loading ? 'Procesando...' : 'Registrarse'}
  </button>
</form>

<style>
  .form-group { margin-bottom: 1rem; }
  .form-row { display: flex; gap: 1rem; }
  .form-row .form-group { flex: 1; }
  label { display: block; margin-bottom: 0.5rem; font-weight: 500; color: #ccc; }
  input, select {
    width: 100%; padding: 0.5rem; border: 1px solid #444; border-radius: 4px;
    background: #1e1e1e; color: #fff; font-size: 14px;
  }
  input:focus, select:focus { outline: 2px solid #007acc; border-color: #007acc; }
  .btn { padding: 0.6rem 1.2rem; border: none; border-radius: 4px; cursor: pointer; font-weight: 500; }
  .btn-primary { background: #007acc; color: white; }
  .btn:disabled { opacity: 0.6; cursor: not-allowed; }
</style>