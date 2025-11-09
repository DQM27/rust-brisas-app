
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { CreateUserInput } from '$lib/types';
  
  export let loading = false;
  
  let email = '';
  let password = '';
  let nombre = '';
  let apellido = '';
  let role: string = 'user';
  
  const dispatch = createEventDispatcher();
  
  function handleSubmit() {
    const data: CreateUserInput = {
      email,
      password,
      nombre,
      apellido,
      role
    };
    dispatch('submit', data);
  }
  
  export function reset() {
    email = '';
    password = '';
    nombre = '';
    apellido = '';
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

  <button type="submit" class="btn btn-primary" disabled={loading}>
    {loading ? 'Procesando...' : 'Registrarse'}
  </button>
</form>
