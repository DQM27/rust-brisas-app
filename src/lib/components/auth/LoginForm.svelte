
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

  <button type="submit" class="btn btn-primary" disabled={loading}>
    {loading ? 'Procesando...' : 'Entrar'}
  </button>
</form>