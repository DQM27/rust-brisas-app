
<script lang="ts">
  import { tauri } from '$lib/tauri';
  import type { User } from '$lib/types';
  import LoginForm from './LoginForm.svelte';
  import RegisterForm from './RegisterForm.svelte';
  import UserCard from './UserCard.svelte';
  import UserList from './UserList.svelte';
  import Alert from './Alert.svelte';
  import { onMount } from 'svelte';
  import { isAuthenticated } from '$lib/stores/auth';

  let mode: 'login' | 'register' = 'login';
  let error = '';
  let success = '';
  let user: User | null = null;
  let users: User[] = [];
  let loading = false;
  
  let loginForm: any;
  let registerForm: any;

  onMount(async () => {
    await loadUsers();
  });

  async function loadUsers() {
    try {
      users = await tauri.listUsers();
    } catch (e) {
      console.error('Error al cargar usuarios:', e);
    }
  }

  async function handleLogin(event: CustomEvent) {
    error = ''; success = ''; loading = true;
    const { email, password } = event.detail;

    try {
      user = await tauri.login(email, password);
      success = `¡Bienvenido, ${user.nombre}!`;
      loginForm?.reset();
      await loadUsers();
    } catch (e: any) {
      error = 'Credenciales inválidas';
    } finally {
      loading = false;
    }
    isAuthenticated.set(true);
  }

  async function handleRegister(event: CustomEvent) {
    error = ''; success = ''; loading = true;

    try {
      await tauri.createUser(event.detail);
      success = 'Cuenta creada. Ahora inicia sesión.';
      registerForm?.reset();
      mode = 'login';
      await loadUsers();
    } catch (e: any) {
      error = e.includes('UNIQUE') ? 'El email ya está registrado' : String(e);
    } finally {
      loading = false;
    }
  }

  function handleLogout() {
    user = null;
    isAuthenticated.set(false);
    success = '';
    error = '';
  }

  function switchMode() {
    mode = mode === 'login' ? 'register' : 'login';
    error = success = '';
    loginForm?.reset();
    registerForm?.reset();
  }
</script>

<div class="auth-panel">
  <div class="auth-container">
    <div class="card">
      <div class="card-header">
        <h1>{mode === 'login' ? 'Iniciar Sesión' : 'Crear Cuenta'}</h1>
      </div>

      <div class="card-body">
        {#if mode === 'login'}
          <LoginForm bind:this={loginForm} {loading} on:submit={handleLogin} />
        {:else}
          <RegisterForm bind:this={registerForm} {loading} on:submit={handleRegister} />
        {/if}

        <Alert type="error" message={error} />
        <Alert type="success" message={success} />

        <div class="text-center">
          <button on:click={switchMode} class="btn-link" disabled={loading}>
            {mode === 'login' ? '¿No tienes cuenta? Regístrate' : '¿Ya tienes cuenta? Inicia sesión'}
          </button>
        </div>
      </div>
    </div>

    {#if user}
      <UserCard {user} on:logout={handleLogout} />
    {/if}

    <UserList {users} />
  </div>
</div>

<style>
  .auth-panel {
    height: 100%;
    overflow-y: auto;
    background: #f5f7fa;
  }

  .auth-container {
    width: 100%;
    max-width: 480px;
    margin: 0 auto;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
</style>
