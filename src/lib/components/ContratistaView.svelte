<script lang="ts">
  import ContratistaForm from '$lib/components/ContratistaForm.svelte';
  import { toast } from 'svelte-5-french-toast';
  import { submitRegisterContratista } from '$lib/logic/contratista/submitRegisterContratista';
  import { invoke } from "@tauri-apps/api/core"; // Para llamar comandos Tauri
  import type { EmpresaResponse } from '$lib/types/empresa';

  let loading = false;
  let formRef: any = null;

  // Lista de empresas activas
  let empresas: EmpresaResponse[] = [];

  // Cargar empresas activas desde la DB
  async function loadEmpresas() {
    try {
      empresas = await invoke<EmpresaResponse[]>('get_empresas_activas');
    } catch (error) {
      console.error('Error al cargar empresas:', error);
      toast.error('No se pudieron cargar las empresas', { icon: '✕', duration: 4000 });
    }
  }

  loadEmpresas(); // Cargar al iniciar

  // Manejar registro de contratista
  async function handleRegister(data) {
    loading = true;
    try {
      const result = await submitRegisterContratista(data);

      if (result.ok) {
        formRef?.reset();
        toast.success('Contratista registrado correctamente', { icon: '✓', duration: 3000 });
      } else {
        toast.error(result.error, { icon: '✕', duration: 4000 });
      }
    } finally {
      loading = false;
    }
  }
</script>

<ContratistaForm
  bind:this={formRef}
  {loading}
  {empresas}        
  onSubmit={handleRegister}
/>
