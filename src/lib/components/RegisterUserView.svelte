<script lang="ts">
  import RegisterUserForm from "$lib/components/UserRegisterPanel.svelte";
  import { submitRegisterUser } from "$lib/logic/user/submitRegisterUser";
  import { toast } from "svelte-5-french-toast";
  import type { UserResponse, CreateUserInput } from "$lib/types/user";

  // Definir la interfaz del componente para acceder a sus métodos exportados
  interface RegisterUserFormInstance {
    reset: () => void;
  }

  let loading = $state(false);
  let formRef = $state<any>();
  let createdUser = $state<UserResponse | null>(null);

  async function handleRegister(data: CreateUserInput) {
    loading = true;
    createdUser = null; // Reset previous

    const result = await submitRegisterUser(data);

    if (result.ok) {
      createdUser = result.user; // Corregido: .user en lugar de .val
      formRef?.reset();
      toast.success("Usuario creado exitosamente", {
        icon: "✓",
        duration: 3000,
      });
    } else {
      toast.error(result.error, { icon: "✕", duration: 4000 });
    }

    loading = false;
  }

  function handleReset() {
    createdUser = null;
    formRef?.reset();
  }
</script>

<RegisterUserForm
  bind:this={formRef}
  {loading}
  {createdUser}
  onSubmit={handleRegister}
  onReset={handleReset}
/>
