<!-- src/lib/components/proveedor/ProveedorView.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { toast } from "svelte-5-french-toast";

  import ProveedorForm from "./ProveedorForm.svelte";
  import type ProveedorFormType from "./ProveedorForm.svelte";

  import {
    proveedorService,
    type CreateProveedorInput,
    type ProveedorResponse,
  } from "$lib/services/proveedorService";
  import { submitFetchActiveEmpresas } from "$lib/logic/empresa/empresaService";
  import { closeTab, activeTabId } from "$lib/stores/tabs";

  interface Props {
    data?: {
      proveedorId?: string;
      initialData?: ProveedorResponse;
    };
  }

  let { data }: Props = $props();

  let loading = $state(false);
  let formRef: ProveedorFormType;
  let mode = $derived(
    data?.proveedorId ? ("edit" as const) : ("create" as const),
  );
  let proveedorData = $state<Partial<ProveedorResponse>>({});

  // Lista de empresas
  let empresas = $state<{ id: string; nombre: string }[]>([]);

  // --- Cargar datos al montar ---
  onMount(async () => {
    // 1. Cargar empresas
    const res = await submitFetchActiveEmpresas();
    if (res.ok) empresas = res.empresas;

    // 2. Si es edición, usar datos pasados o cargar
    if (data?.initialData) {
      proveedorData = data.initialData;
    } else if (data?.proveedorId) {
      loading = true;
      try {
        const result = await proveedorService.getByCedula(data.proveedorId);
        if (result) {
          proveedorData = result;
        } else {
          toast.error("Proveedor no encontrado");
        }
      } catch (err: any) {
        toast.error("Error al cargar datos del proveedor: " + err.message);
      }
      loading = false;
    }
  });

  async function handleSubmit(formData: any) {
    loading = true;

    try {
      if (mode === "edit" && data?.proveedorId) {
        // MODO EDICIÓN - Not implemented yet in backend
        toast.error("La edición de proveedores aún no está implementada");
      } else {
        // MODO CREACIÓN
        const input: CreateProveedorInput = {
          cedula: formData.cedula,
          nombre: formData.nombre,
          segundoNombre: formData.segundoNombre || undefined,
          apellido: formData.apellido,
          segundoApellido: formData.segundoApellido || undefined,
          empresaId: formData.empresaId,
          tieneVehiculo: formData.tieneVehiculo,
          tipoVehiculo: formData.tipoVehiculo || undefined,
          placa: formData.placa || undefined,
          marca: formData.marca || undefined,
          modelo: formData.modelo || undefined,
          color: formData.color || undefined,
        };

        await proveedorService.create(input);

        formRef?.reset();
        toast.success("Proveedor registrado exitosamente", {
          icon: "✓",
          duration: 3000,
        });
        // Cerrar la pestaña actual al finalizar
        closeTab($activeTabId);
      }
    } catch (err: any) {
      toast.error(err.message || "Error al guardar proveedor", {
        icon: "✕",
        duration: 4000,
      });
    }

    loading = false;
  }
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <ProveedorForm
    bind:this={formRef}
    {loading}
    {empresas}
    {mode}
    initialData={proveedorData}
    onSubmit={handleSubmit}
  />
</div>
