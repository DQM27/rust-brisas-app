<script lang="ts">
  import { fade, fly, scale } from "svelte/transition";
  import { X } from "lucide-svelte";
  import type {
    ProveedorResponse,
    CreateProveedorInput,
    UpdateProveedorInput,
  } from "$lib/types/proveedor";
  import { submitCreateEmpresa } from "$lib/logic/empresa/empresaService";
  import { toast } from "svelte-5-french-toast";
  import ProveedorForm from "./ProveedorForm.svelte";
  import { empresaStore } from "$lib/stores/empresaStore.svelte";

  interface Props {
    show: boolean;
    proveedor?: ProveedorResponse | null;
    loading?: boolean;
    onSave: (
      data: CreateProveedorInput | UpdateProveedorInput,
    ) => Promise<boolean | void>;
    onClose: () => void;
    readonly?: boolean;
  }

  let {
    show,
    proveedor = null,
    loading = false,
    onSave,
    onClose,
    readonly = false,
  }: Props = $props();

  const isEditMode = $derived(!!proveedor);
  const modalTitle = $derived(
    readonly
      ? `Detalle Proveedor: ${proveedor?.nombre}`
      : isEditMode
        ? `Editar Proveedor: ${proveedor?.nombre}`
        : "Nuevo Proveedor",
  );

  // Empresas state (Global Store)
  // let empresas = ... (Reemplazado por empresaStore.empresas)

  // Estado para modal de nueva empresa (inline)
  let showEmpresaModal = $state(false);
  let creatingEmpresa = $state(false);
  let nuevaEmpresaNombre = $state("");
  let empresaError = $state("");

  // Cargar empresas
  $effect(() => {
    if (show) {
      empresaStore.init();
    }
  });

  // async function loadEmpresas() ... (Eliminado, usa store)

  // Handlers para crear nueva empresa
  function handleCreateEmpresa() {
    nuevaEmpresaNombre = "";
    empresaError = "";
    showEmpresaModal = true;
  }

  async function handleSaveEmpresa() {
    if (!nuevaEmpresaNombre.trim()) {
      empresaError = "El nombre es requerido";
      return;
    }

    creatingEmpresa = true;
    empresaError = "";

    try {
      const result = await submitCreateEmpresa(nuevaEmpresaNombre);
      if (result.ok) {
        toast.success(`Empresa "${result.empresa.nombre}" creada exitosamente`);
        // Agregar la nueva empresa a la lista y cerrar modal
        // Agregar la nueva empresa a la lista y cerrar modal
        empresaStore.add(result.empresa);
        showEmpresaModal = false;
        nuevaEmpresaNombre = "";
      } else {
        empresaError =
          (result as { ok: false; error: string }).error ||
          "Error al crear la empresa";
      }
    } catch (err: any) {
      console.error("Error creating empresa:", err);
      empresaError = "Error inesperado al crear empresa";
    } finally {
      creatingEmpresa = false;
    }
  }

  // Estilos (mismos que Contratista para consistencia)
  const labelClass = "text-xs font-medium text-gray-700 dark:text-gray-300";
  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent focus:outline-none transition-all placeholder-gray-400 dark:placeholder-gray-500 disabled:opacity-50";

  // Prepara los datos iniciales para el formulario
  const initialData = $derived.by(() => {
    if (proveedor) {
      return {
        cedula: proveedor.cedula,
        nombre: proveedor.nombre,
        apellido: proveedor.apellido,
        segundoNombre: proveedor.segundoNombre || "",
        segundoApellido: proveedor.segundoApellido || "",
        empresaId: proveedor.empresaId,
        estado: (proveedor.estado as any) || "ACTIVO",

        tieneVehiculo: !!proveedor.vehiculoPlaca,
        tipoVehiculo: proveedor.vehiculoTipo || "",
        placa: proveedor.vehiculoPlaca || "",
        marca: proveedor.vehiculoMarca || "",
        modelo: proveedor.vehiculoModelo || "",
        color: proveedor.vehiculoColor || "",
      };
    }
    return {
      // Valores por defecto para creación
      cedula: "",
      nombre: "",
      apellido: "",
      segundoNombre: "",
      segundoApellido: "",
      empresaId: "",
      estado: "ACTIVO",
      tieneVehiculo: false,
      tipoVehiculo: "",
      placa: "",
      marca: "",
      modelo: "",
      color: "",
    };
  });
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 150 }}
  >
    <!-- Backdrop -->
    <button
      class="absolute inset-0 bg-black/60 backdrop-blur-sm border-0 cursor-default"
      onclick={onClose}
      aria-label="Cerrar"
    ></button>

    <!-- Modal Content -->
    <div
      class="relative z-10 w-full max-w-2xl max-h-[90vh] overflow-auto rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:fly={{ y: 20, duration: 200 }}
    >
      <!-- Header -->
      <div
        class="sticky top-0 z-20 flex items-center justify-between px-6 py-4 bg-white dark:bg-[#0d1117] border-b border-gray-200 dark:border-gray-700"
      >
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          {modalTitle}
        </h2>
        <button
          onclick={onClose}
          class="p-1 rounded-full text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
        >
          <X size={20} />
        </button>
      </div>

      <!-- Form Component with Superforms -->
      <ProveedorForm
        data={initialData}
        {isEditMode}
        {loading}
        empresas={empresaStore.empresas}
        {onSave}
        {onClose}
        currentId={proveedor?.id || ""}
        onCreateEmpresa={handleCreateEmpresa}
      />
    </div>
  </div>
{/if}

<!-- Modal para crear nueva empresa (Inline para igualar comportamiento de Contratista) -->
{#if showEmpresaModal}
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="absolute inset-0 bg-black/50"
      onclick={() => !creatingEmpresa && (showEmpresaModal = false)}
    ></div>
    <div
      class="relative w-full max-w-md rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:scale={{ start: 0.95, duration: 200 }}
    >
      <div
        class="px-5 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22]"
      >
        <h3 class="text-base font-semibold text-gray-900 dark:text-gray-100">
          Nueva Empresa
        </h3>
      </div>
      <div class="p-5 space-y-3">
        {#if empresaError}
          <div
            class="rounded bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 p-2 text-xs text-red-700 dark:text-red-300"
          >
            {empresaError}
          </div>
        {/if}
        <div class="space-y-1">
          <label for="newEmpresa" class={labelClass}>Nombre de la Empresa</label
          >
          <input
            id="newEmpresa"
            type="text"
            bind:value={nuevaEmpresaNombre}
            placeholder="Servicios Generales S.A."
            disabled={creatingEmpresa}
            class={inputClass}
            onkeydown={(e) => e.key === "Enter" && handleSaveEmpresa()}
          />
        </div>
      </div>
      <div
        class="flex justify-end gap-2 px-5 py-3 bg-gray-50 dark:bg-[#161b22] border-t border-gray-200 dark:border-gray-700"
      >
        <button
          type="button"
          disabled={creatingEmpresa}
          onclick={() => (showEmpresaModal = false)}
          class="px-3 py-1.5 text-xs font-medium rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#21262d]"
        >
          Cancelar
        </button>
        <button
          type="button"
          disabled={creatingEmpresa || !nuevaEmpresaNombre.trim()}
          onclick={handleSaveEmpresa}
          class="px-3 py-1.5 text-xs font-medium rounded-md bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 transition-colors"
        >
          {creatingEmpresa ? "Guardando..." : "Guardar"}
        </button>
      </div>
    </div>
  </div>
{/if}
