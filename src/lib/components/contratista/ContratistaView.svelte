<script lang="ts">
  import { onMount } from 'svelte';
  import { toast } from 'svelte-5-french-toast';

  import ContratistaForm from './ContratistaForm.svelte';
  import type ContratistaFormType from './ContratistaForm.svelte';

  import type { CreateContratistaInput, ContratistaResponse } from '$lib/types/contratista';
  import { submitRegisterContratista } from '$lib/logic/contratista/submitRegisterContratista';
  import { submitFetchActiveEmpresas } from '$lib/logic/empresa/empresaService';

  let loading = false;
  let formRef: ContratistaFormType;

  // Lista de empresas
  let empresas: { id: string; nombre: string }[] = [];

  // --- Cargar empresas activas al montar ---
  onMount(async () => {
    const res = await submitFetchActiveEmpresas();
    if (res.ok) empresas = res.empresas;
  });

  async function handleRegister(data: CreateContratistaInput) {
    loading = true;

    const result = await submitRegisterContratista(data);

    if (result.ok) {
      formRef?.reset();
      toast.success('Contratista registrado exitosamente', { icon: '✓', duration: 3000 });
    } else {
      toast.error(result.error, { icon: '✕', duration: 4000 });
    }

    loading = false;
  }
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <ContratistaForm
    bind:this={formRef}
    {loading}
    {empresas}
    onSubmit={handleRegister}
  />
</div>