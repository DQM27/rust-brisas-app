<!-- src/routes/login/+page.svelte (NUEVO LOGIN LIMPIO) -->
<script lang="ts">
  import LoginForm from '$lib/components/auth/LoginForm.svelte';
  import Alert from '$lib/components/auth/Alert.svelte';
  import { tauri } from '$lib/tauri';
  import { login } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  let error = '';
  let loading = false;

  async function handleLogin(e: CustomEvent<{ email: string; password: string }>) {
    error = '';
    loading = true;

    try {
      const user = await tauri.login(e.detail.email, e.detail.password);
      login(user);
      goto('/');
    } catch {
      error = 'Credenciales inválidas';
    } finally {
      loading = false;
    }
  }
</script>

<div class="login-page">
  <div class="card">
    <h1>Iniciar Sesión</h1>
    <LoginForm {loading} on:submit={handleLogin} />
    <Alert type="error" message={error} />
    <div class="text-center">
      <button class="btn-link" disabled={loading}>
        ¿Olvidaste tu contraseña?
      </button>
    </div>
  </div>
</div>

<style>
  .login-page {
    width: 100%;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #1e1e1e;
  }

  .card {
    background: #252526;
    padding: 2rem;
    border-radius: 8px;
    width: 100%;
    max-width: 400px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
  }

  h1 {
    text-align: center;
    margin: 0 0 1.5rem;
    color: #ccc;
  }

  .btn-link {
    background: none;
    border: none;
    color: #007acc;
    font-size: 0.9rem;
    cursor: pointer;
    margin-top: 1rem;
    display: block;
  }
</style>