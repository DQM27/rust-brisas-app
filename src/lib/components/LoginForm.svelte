<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let loading = false;
  let email = '';
  let password = '';
  const dispatch = createEventDispatcher();

  function handleSubmit() {
    dispatch('submit', { email, password });
  }

  export function reset() {
    email = '';
    password = '';
  }
</script>

<div class="login-page">
  <form on:submit|preventDefault={handleSubmit} class="login-form">
    <h1>Iniciar Sesión</h1>
    <div class="form-group">
      <label for="email">Email</label>
      <input id="email" type="email" bind:value={email} placeholder="correo@ejemplo.com" disabled={loading} required />
    </div>

    <div class="form-group">
      <label for="password">Contraseña</label>
      <input id="password" type="password" bind:value={password} placeholder="••••••••" disabled={loading} required />
    </div>

    <button type="submit" class="btn" disabled={loading}>
      {loading ? 'Procesando...' : 'Entrar'}
    </button>
  </form>
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

.login-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 2rem;
  background: #252526;
  border-radius: 8px;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.3);
  color: #ccc;
}

.login-form h1 {
  text-align: center;
  margin: 0 0 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

input {
  padding: 8px;
  border-radius: 4px;
  border: 1px solid #3c3c3c;
  background: #1e1e1e;
  color: #ccc;
}

input:disabled {
  opacity: 0.6;
}

.btn {
  padding: 10px;
  border: none;
  border-radius: 4px;
  background: #007acc;
  color: white;
  cursor: pointer;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
