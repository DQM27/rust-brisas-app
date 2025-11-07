<script lang="ts">
  import { tauri } from '$lib/tauri';
  import type { User } from '$lib/types';
  import { onMount } from 'svelte';

  // === ESTADOS ===
  let mode: 'login' | 'register' = 'login';
  let email = '';
  let password = '';
  let nombre = '';
  let apellido = '';
  let role: string = 'user';
  let error = '';
  let success = '';
  let user: User | null = null;
  let users: User[] = [];
  let loading = false;

  // === CARGAR USUARIOS AL INICIAR ===
  onMount(async () => {
    try {
      users = await tauri.listUsers();
    } catch (e) {
      console.error('Error al cargar usuarios:', e);
    }
  });

  // === LOGIN ===
  async function handleLogin() {
    error = ''; success = ''; loading = true;
    if (!email || !password) {
      error = 'Email y contraseña son obligatorios';
      loading = false;
      return;
    }

    try {
      user = await tauri.login(email, password);
      success = `¡Bienvenido, ${user.nombre}!`;
      resetForm();
      users = await tauri.listUsers();
    } catch (e: any) {
      error = e === 'Usuario no encontrado' || e === 'Contraseña incorrecta'
        ? 'Credenciales inválidas'
        : e;
    } finally {
      loading = false;
    }
  }

  // === REGISTRO ===
  async function handleRegister() {
    error = ''; success = ''; loading = true;
    if (!email || !password || !nombre || !apellido) {
      error = 'Todos los campos son obligatorios';
      loading = false;
      return;
    }

    try {
      await tauri.createUser({ email, password, nombre, apellido, role });
      success = 'Cuenta creada. Ahora inicia sesión.';
      resetForm();
      mode = 'login';
      users = await tauri.listUsers();
    } catch (e: any) {
      error = e.includes('UNIQUE constraint failed')
        ? 'El email ya está registrado'
        : e;
    } finally {
      loading = false;
    }
  }

  // === RESETEAR FORMULARIO ===
  function resetForm() {
    email = password = nombre = apellido = '';
    role = 'user';
  }

  // === CAMBIAR MODO ===
  function switchMode() {
    mode = mode === 'login' ? 'register' : 'login';
    error = success = '';
    resetForm();
  }

  // === LOGOUT ===
  function handleLogout() {
    user = null;
    success = '';
    error = '';
  }
</script>

<div class="container">
  <div class="content">
    <!-- PANEL PRINCIPAL -->
    <div class="card">
      <div class="card-header">
        <h1>{mode === 'login' ? 'Iniciar Sesión' : 'Crear Cuenta'}</h1>
      </div>

      <div class="card-body">
        <!-- FORMULARIO -->
        <form on:submit|preventDefault={mode === 'login' ? handleLogin : handleRegister}>
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

          {#if mode === 'register'}
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
          {/if}

          <button type="submit" class="btn btn-primary" disabled={loading}>
            {loading ? 'Procesando...' : mode === 'login' ? 'Entrar' : 'Registrarse'}
          </button>
        </form>

        <!-- MENSAJES -->
        {#if error}
          <div class="alert alert-error">{error}</div>
        {/if}
        {#if success}
          <div class="alert alert-success">{success}</div>
        {/if}

        <!-- CAMBIAR MODO -->
        <div class="text-center">
          <button on:click={switchMode} class="btn-link" disabled={loading}>
            {mode === 'login' ? '¿No tienes cuenta? Regístrate' : '¿Ya tienes cuenta? Inicia sesión'}
          </button>
        </div>
      </div>
    </div>

    <!-- USUARIO LOGUEADO -->
    {#if user}
      <div class="card">
        <div class="user-info">
          <div class="user-avatar">
            {user.nombre.charAt(0).toUpperCase()}{user.apellido.charAt(0).toUpperCase()}
          </div>
          <div class="user-details">
            <h3>{user.nombre} {user.apellido}</h3>
            <p>{user.email}</p>
            <span class="badge {user.role === 'admin' ? 'badge-admin' : 'badge-user'}">
              {user.role}
            </span>
            {#if !user.isActive}
              <span class="badge badge-inactive">Inactivo</span>
            {/if}
          </div>
          <button on:click={handleLogout} class="btn btn-secondary btn-sm">
            Cerrar Sesión
          </button>
        </div>
      </div>
    {/if}

    <!-- LISTA DE USUARIOS -->
    {#if users.length > 0}
      <div class="card">
        <div class="card-header">
          <h2>Usuarios Registrados ({users.length})</h2>
        </div>
        <div class="user-list">
          {#each users as u}
            <div class="user-item">
              <div class="user-item-avatar">
                {u.nombre.charAt(0).toUpperCase()}
              </div>
              <div class="user-item-info">
                <div class="user-item-name">{u.nombre} {u.apellido}</div>
                <div class="user-item-email">{u.email}</div>
              </div>
              <div class="user-item-badges">
                <span class="badge {u.role === 'admin' ? 'badge-admin' : 'badge-user'}">
                  {u.role}
                </span>
                {#if !u.isActive}
                  <span class="badge badge-inactive">Inactivo</span>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  /* === RESET Y VARIABLES === */
  * {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    background: #f5f7fa;
    color: #2d3748;
    line-height: 1.6;
  }

  /* === CONTENEDOR PRINCIPAL === */
  .container {
    min-height: 100vh;
    padding: 2rem 1rem;
    display: flex;
    justify-content: center;
    align-items: flex-start;
  }

  .content {
    width: 100%;
    max-width: 480px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  /* === TARJETAS === */
  .card {
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .card-header {
    padding: 1.5rem;
    border-bottom: 1px solid #e2e8f0;
  }

  .card-header h1,
  .card-header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #1a202c;
  }

  .card-body {
    padding: 1.5rem;
  }

  /* === FORMULARIO === */
  form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #4a5568;
  }

  input,
  select {
    width: 100%;
    padding: 0.625rem 0.75rem;
    border: 1px solid #cbd5e0;
    border-radius: 6px;
    font-size: 0.9375rem;
    transition: all 0.15s ease;
    background: white;
  }

  input:focus,
  select:focus {
    outline: none;
    border-color: #4299e1;
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.1);
  }

  input:disabled,
  select:disabled {
    background: #f7fafc;
    cursor: not-allowed;
    opacity: 0.6;
  }

  input::placeholder {
    color: #a0aec0;
  }

  /* === BOTONES === */
  .btn {
    padding: 0.625rem 1rem;
    border: none;
    border-radius: 6px;
    font-size: 0.9375rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: center;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #3182ce;
    color: white;
    margin-top: 0.5rem;
  }

  .btn-primary:hover:not(:disabled) {
    background: #2c5282;
  }

  .btn-secondary {
    background: #718096;
    color: white;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #4a5568;
  }

  .btn-sm {
    padding: 0.375rem 0.75rem;
    font-size: 0.875rem;
  }

  .btn-link {
    background: none;
    border: none;
    color: #3182ce;
    font-size: 0.875rem;
    cursor: pointer;
    padding: 0.5rem;
    transition: color 0.15s ease;
  }

  .btn-link:hover:not(:disabled) {
    color: #2c5282;
    text-decoration: underline;
  }

  .text-center {
    text-align: center;
    margin-top: 1rem;
  }

  /* === ALERTAS === */
  .alert {
    padding: 0.75rem 1rem;
    border-radius: 6px;
    font-size: 0.875rem;
    margin-top: 1rem;
  }

  .alert-error {
    background: #fff5f5;
    color: #c53030;
    border: 1px solid #feb2b2;
  }

  .alert-success {
    background: #f0fff4;
    color: #2f855a;
    border: 1px solid #9ae6b4;
  }

  /* === USUARIO LOGUEADO === */
  .user-info {
    padding: 1.5rem;
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .user-avatar {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    font-weight: 600;
    flex-shrink: 0;
  }

  .user-details {
    flex: 1;
  }

  .user-details h3 {
    margin: 0 0 0.25rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: #1a202c;
  }

  .user-details p {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: #718096;
  }

  /* === BADGES === */
  .badge {
    display: inline-block;
    padding: 0.125rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 500;
    margin-right: 0.375rem;
  }

  .badge-admin {
    background: #fef5e7;
    color: #d69e2e;
  }

  .badge-user {
    background: #e6fffa;
    color: #319795;
  }

  .badge-inactive {
    background: #fff5f5;
    color: #e53e3e;
  }

  /* === LISTA DE USUARIOS === */
  .user-list {
    max-height: 400px;
    overflow-y: auto;
  }

  .user-item {
    padding: 1rem 1.5rem;
    display: flex;
    align-items: center;
    gap: 0.875rem;
    border-bottom: 1px solid #e2e8f0;
    transition: background 0.15s ease;
  }

  .user-item:last-child {
    border-bottom: none;
  }

  .user-item:hover {
    background: #f7fafc;
  }

  .user-item-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.875rem;
    font-weight: 600;
    flex-shrink: 0;
  }

  .user-item-info {
    flex: 1;
    min-width: 0;
  }

  .user-item-name {
    font-size: 0.9375rem;
    font-weight: 500;
    color: #1a202c;
    margin-bottom: 0.125rem;
  }

  .user-item-email {
    font-size: 0.875rem;
    color: #718096;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .user-item-badges {
    display: flex;
    gap: 0.25rem;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  /* === SCROLLBAR === */
  .user-list::-webkit-scrollbar {
    width: 6px;
  }

  .user-list::-webkit-scrollbar-track {
    background: #f1f1f1;
  }

  .user-list::-webkit-scrollbar-thumb {
    background: #cbd5e0;
    border-radius: 3px;
  }

  .user-list::-webkit-scrollbar-thumb:hover {
    background: #a0aec0;
  }

  /* === RESPONSIVE === */
  @media (max-width: 640px) {
    .container {
      padding: 1rem;
    }

    .content {
      gap: 1rem;
    }

    .card-header,
    .card-body,
    .user-info,
    .user-item {
      padding: 1rem;
    }

    .form-row {
      grid-template-columns: 1fr;
    }

    .user-info {
      flex-wrap: wrap;
    }

    .btn-sm {
      width: 100%;
      margin-top: 0.5rem;
    }
  }
</style>