<!-- src/lib/components/admin/UserRegisterPanel.svelte -->
<script lang="ts">
  import Alert from '$lib/components/Alert.svelte';
  import { users } from '$lib/api/users';
  import { toast } from 'svelte-5-french-toast';
  import type { UserRole } from '$lib/types/user';

  let loading = false;
  let success = '';
  let error = '';

  let email = '';
  let password = '';
  let nombre = '';
  let apellido = '';
  let role: UserRole = 'guardia';

  async function handleSubmit() {
    error = success = '';
    loading = true;

    try {
      await users.create({ email, password, nombre, apellido, role });
      success = 'Usuario creado exitosamente';
      toast.success(success);
      
      // Limpiar formulario
      email = password = nombre = apellido = '';
      role = 'guardia';
    } catch (err: any) {
      error = err.includes('UNIQUE') ? 'El email ya está registrado' : 'Error al crear usuario';
      toast.error(error);
    } finally {
      loading = false;
    }
  }
</script>

<div class="user-register-panel">
  <h2>Registrar Nuevo Usuario</h2>
  
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
        <option value="guardia">Guardia</option>
        <option value="supervisor">Supervisor</option>
        <option value="admin">Administrador</option>
      </select>
    </div>

    <button
      type="submit"
      class="btn btn-primary"
      disabled={loading || !email || !password || !nombre || !apellido}
    >
      {loading ? 'Procesando...' : 'Registrar Usuario'}
    </button>
  </form>

  <Alert type="success" message={success} />
  <Alert type="error" message={error} />
</div>

<style>
  .user-register-panel {
    padding: 1.5rem;
    background: #252526;
    border-radius: 8px;
    margin: 1rem;
  }

  h2 {
    margin-top: 0;
    color: #ccc;
    border-bottom: 1px solid #007acc;
    padding-bottom: 0.5rem;
  }

  .form-group { 
    margin-bottom: 1rem; 
  }
  
  .form-row { 
    display: flex; 
    gap: 1rem; 
  }
  
  .form-row .form-group { 
    flex: 1; 
  }
  
  label { 
    display: block; 
    margin-bottom: 0.5rem; 
    font-weight: 500; 
    color: #ccc; 
  }
  
  input, select {
    width: 100%; 
    padding: 0.5rem; 
    border: 1px solid #444; 
    border-radius: 4px;
    background: #1e1e1e; 
    color: #fff; 
    font-size: 14px;
  }
  
  input:focus, select:focus { 
    outline: 2px solid #007acc; 
    border-color: #007acc; 
  }
  
  .btn { 
    padding: 0.6rem 1.2rem; 
    border: none; 
    border-radius: 4px; 
    cursor: pointer; 
    font-weight: 500; 
    width: 100%;
  }
  
  .btn-primary { 
    background: #007acc; 
    color: white; 
  }
  
  .btn:disabled { 
    opacity: 0.6; 
    cursor: not-allowed; 
  }
</style>