<script lang="ts">
  import { tauri } from '$lib/tauri';
  import type { User } from '$lib/types';
  import LoginForm from './LoginForm.svelte';
  import RegisterForm from './RegisterForm.svelte';
  import Alert from './Alert.svelte';
  import { onMount } from 'svelte';
  import { login } from '$lib/stores/auth';

  let mode: 'login' | 'register' = 'login';
  let error = '';
  let success = '';
  let loading = false;
  
  let loginForm: any;
  let registerForm: any;

  async function handleLogin(event: CustomEvent) {
    error = '';
    success = '';
    loading = true;
    
    const { email, password } = event.detail;

    try {
      const user = await tauri.login(email, password);
      success = `¡Bienvenido, ${user.nombre}!`;
      loginForm?.reset();
      
      // Usar helper de auth que maneja tabs automáticamente
      login(user);
      
    } catch (e: any) {
      error = 'Credenciales inválidas';
    } finally {
      loading = false;
    }
  }

  async function handleRegister(event: CustomEvent) {
    error = '';
    success = '';
    loading = true;

    try {
      await tauri.createUser(event.detail);
      success = 'Cuenta creada. Ahora inicia sesión.';
      registerForm?.reset();
      mode = 'login';
    } catch (e: any) {
      error = e.includes('UNIQUE') ? 'El email ya está registrado' : String(e);
    } finally {
      loading = false;
    }
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
  </div>
</div>

<style>
  .auth-panel {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary);
    overflow-y: auto;
    padding: 2rem;
  }

  .auth-container {
    width: 100%;
    max-width: 420px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  /* Los estilos de .card, .card-header, .card-body, .text-center y .btn-link 
     ya están definidos en app.css global */
</style>