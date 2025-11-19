<script lang="ts">
  import { login as setAuth } from '$lib/stores/auth';
  import LoginForm from '$lib/components/LoginForm.svelte';
  import { submitLogin } from '$lib/logic/auth/submitLogin';
  import { toast } from 'svelte-5-french-toast';

  import type LoginFormType from '$lib/components/LoginForm.svelte';

  let loading = $state(false);
  let formRef = $state<LoginFormType>();

  async function handleLogin({ email, password }: { email: string; password: string }) {
    loading = true;

    const result = await submitLogin(email, password);

    if (result.ok) {
      setAuth(result.user);
      formRef?.reset();
      toast.success("Sesión iniciada correctamente", { icon: "✓" });
    } else {
      toast.error(result.error, { icon: "✕" });
    }

    loading = false;
  }
</script>

<div class="flex h-screen w-full items-center justify-center bg-[#1e1e1e]">
  <LoginForm bind:this={formRef} {loading} onSubmit={handleLogin} />
</div>