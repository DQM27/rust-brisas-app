<script lang="ts">
  import { onMount } from "svelte";
  import { toast } from "svelte-5-french-toast";

  import ContratistaForm from "./ContratistaForm.svelte";
  import type ContratistaFormType from "./ContratistaForm.svelte";

  import type {
    CreateContratistaInput,
    ContratistaResponse,
    UpdateContratistaInput,
  } from "$lib/types/contratista";
  import * as contratistaService from "$lib/logic/contratista/contratistaService";
  import { submitFetchActiveEmpresas } from "$lib/logic/empresa/empresaService";
  import { closeTab, activeTabId } from "$lib/stores/tabs";

  interface Props {
    data?: {
      contratistaId?: string;
    };
  }

  let { data }: Props = $props();

  let loading = $state(false);
  let formRef: ContratistaFormType;
  let mode = $derived(
    data?.contratistaId ? ("edit" as const) : ("create" as const),
  );
  let contratistaData = $state<Partial<ContratistaResponse>>({});

  // Lista de empresas
  let empresas = $state<{ id: string; nombre: string }[]>([]);

  // --- Cargar datos al montar ---
  onMount(async () => {
    // 1. Cargar empresas
    const res = await submitFetchActiveEmpresas();
    if (res.ok) empresas = res.empresas;

    // 2. Si es edición, cargar datos del contratista
    if (data?.contratistaId) {
      loading = true;
      const result = await contratistaService.fetchContratistaById(
        data.contratistaId,
      );
      if (result.ok) {
        contratistaData = result.data;
      } else {
        toast.error("Error al cargar datos del contratista: " + result.error);
      }
      loading = false;
    }
  });

  async function handleRegister(formData: any) {
    loading = true;
    let result;

    if (mode === "edit" && data?.contratistaId) {
      // MODO EDICIÓN
      const updateInput: UpdateContratistaInput = {
        id: data.contratistaId,
        ...formData,
      };
      result = await contratistaService.update(data.contratistaId, updateInput);
    } else {
      // MODO CREACIÓN
      const createInput: CreateContratistaInput = formData;
      result = await contratistaService.register(createInput);
    }

    if (result.ok) {
      if (mode === "create") formRef?.reset();

      toast.success(
        `Contratista ${mode === "edit" ? "actualizado" : "registrado"} exitosamente`,
        {
          icon: "✓",
          duration: 3000,
        },
      );
      // Cerrar la pestaña actual al finalizar
      closeTab($activeTabId);
    } else {
      toast.error(result.error, { icon: "✕", duration: 4000 });
    }

    loading = false;
  }
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <ContratistaForm
    bind:this={formRef}
    {loading}
    {empresas}
    {mode}
    initialData={contratistaData}
    onSubmit={handleRegister}
  />
</div>
