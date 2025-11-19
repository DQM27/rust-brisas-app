<script lang="ts">
  console.log('RegisterUserView montado');
  import RegisterUserForm from '$lib/components/UserRegisterPanel.svelte';
  import { submitRegisterUser } from '$lib/logic/user/submitRegisterUser';
  import { toast } from 'svelte-5-french-toast';
  import type { UserRole } from '$lib/types/user';

  import type RegisterUserFormType from '$lib/components/UserRegisterPanel.svelte';

  let loading = $state(false);
  let formRef = $state<RegisterUserFormType>();

  async function handleRegister(data: {
    email: string;
    password: string;
    nombre: string;
    apellido: string;
    role: UserRole;
  }) {
    loading = true;

    const result = await submitRegisterUser(
      data.email,
      data.password,
      data.nombre,
      data.apellido,
      data.role
    );

    if (result.ok) {
      formRef?.reset();
      toast.success('Usuario creado exitosamente', { icon: '✓', duration: 3000 });
    } else {
      toast.error(result.error, { icon: '✕', duration: 4000 });
    }

    loading = false;
  }
</script>

<RegisterUserForm bind:this={formRef} {loading} onSubmit={handleRegister} />