<script lang="ts">
import UserRegisterForm from '$lib/components/UserRegisterPanel.svelte';
import { users } from '$lib/api/users';
import { toast } from 'svelte-5-french-toast';

let loading = false;

async function handleUserRegister(data: {
  email: string;
  password: string;
  nombre: string;
  apellido: string;
  role: string;
}) {
  loading = true;

  try {
    await users.create(data);
    toast.success('Usuario creado exitosamente', { icon: '✓', duration: 3000 });
  } catch (err: any) {
    const message = err.includes('UNIQUE') ? 'El email ya está registrado' : 'Error al crear usuario';
    toast.error(message, { icon: '✕', duration: 4000 });
  } finally {
    loading = false;
  }
}
</script>

<UserRegisterForm {loading} onSubmit={handleUserRegister} />
