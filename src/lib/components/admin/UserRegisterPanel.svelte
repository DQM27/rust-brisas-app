<!-- src/lib/components/admin/UserRegisterPanel.svelte -->
<script lang="ts">
  import RegisterForm from '$lib/components/auth/RegisterForm.svelte';
  import Alert from '$lib/components/auth/Alert.svelte';
  import { tauri } from '$lib/tauri';
  import { toast } from 'svelte-5-french-toast';
  import type { CreateUserInput } from '$lib/types';

  let loading = false;
  let success = '';
  let error = '';

  async function handleRegister(e: CustomEvent<CreateUserInput>) {
    error = success = '';
    loading = true;

    try {
      await tauri.createUser(e.detail);
      success = 'Usuario creado exitosamente';
      toast.success(success);
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
  <RegisterForm {loading} on:submit={handleRegister} />
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
</style>