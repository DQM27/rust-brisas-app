<!-- src/lib/components/ContratistaView.svelte -->
<script lang="ts">
  import ContratistaForm from '$lib/components/ContratistaForm.svelte';
  import { submitRegisterContratista } from '$lib/logic/contratista/submitRegisterContratista';
  import { toast } from 'svelte-5-french-toast';
  import type { CreateContratistaInput } from '$lib/types/contratista';

  let loading = $state(false);

  // SIMPLE typed ref: solo necesitamos reset()
  let formRef: { reset: () => void } | null = null;

  async function handleRegister(data: CreateContratistaInput) {
    loading = true;

    const result = await submitRegisterContratista(data);

    if (result.ok) {
      formRef?.reset();
      toast.success('Contratista registrado correctamente', { icon: '✓', duration: 3000 });
    } else {
      toast.error(result.error, { icon: '✕', duration: 4000 });
    }

    loading = false;
  }
</script>

<div class="flex justify-center mt-10 px-4">
  <div class="w-full max-w-3xl">
    <h1 class="text-2xl font-bold text-gray-200 mb-6 text-center">Registrar Contratista</h1>

    <!-- Si quieres pasar un select de empresas, puedes hacerlo con slot="empresa" -->
    <ContratistaForm bind:this={formRef} {loading} onSubmit={handleRegister}>
      <select slot="empresa" bind:value={$state('empresaId')} class="px-3 py-2 rounded bg-gray-700 text-white border border-gray-600 w-full">
        <option value="">-- Seleccione empresa --</option>
        <!-- Ejemplo estático; reemplaza con tu lista dinámica -->
        <option value="empresa-1">Empresa 1</option>
        <option value="empresa-2">Empresa 2</option>
      </select>
    </ContratistaForm>
  </div>
</div>
