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
  // Estilos
  const labelClass = "text-xs font-medium text-secondary mb-1";
  const inputClass =
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all";

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

  // Handler para Ctrl+S
  function handleKeydown(e: KeyboardEvent) {
    if (!show || readonly || loading) return;
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
      e.preventDefault();
      const form = document.querySelector("form") as HTMLFormElement;
      if (form) {
        form.requestSubmit();
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 150 }}
  >
    <!-- Backdrop -->
    <div
      class="absolute inset-0 bg-black/60 backdrop-blur-sm"
      onclick={onClose}
      role="presentation"
    ></div>

    <!-- Modal Content -->
    <div
      class="relative z-10 w-full max-w-[700px] max-h-[95vh] overflow-hidden rounded-xl bg-surface-2 shadow-2xl border border-surface flex flex-col"
      transition:fly={{ y: 20, duration: 200 }}
      role="dialog"
      aria-modal="true"
    >
      <!-- Header -->
      <div
        class="flex-none flex items-center justify-between px-3 py-3 bg-surface-2 border-b border-surface"
      >
        <h2 class="text-xl font-semibold text-primary">
          {modalTitle}
        </h2>
        <button
          onclick={onClose}
          class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
        >
          <X size={20} />
        </button>
      </div>

      <div class="flex-1 overflow-y-auto">
        <div class="p-6">
          <div class="bg-surface-1 rounded-lg border border-surface p-7">
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
      </div>
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
      class="relative w-full max-w-md rounded-xl bg-surface-2 shadow-2xl border border-surface"
      transition:scale={{ start: 0.95, duration: 200 }}
    >
      <div class="px-5 py-3 border-b border-surface bg-surface-2">
        <h3 class="text-base font-semibold text-primary">Nueva Empresa</h3>
      </div>
      <div class="p-5 space-y-3">
        {#if empresaError}
          <div
            class="rounded bg-red-900/20 border border-red-800 p-2 text-xs text-red-300"
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
        class="flex justify-end gap-3 px-5 py-3 bg-surface-1 border-t border-surface"
      >
        <button
          type="button"
          disabled={creatingEmpresa}
          onclick={() => (showEmpresaModal = false)}
          class="px-4 py-2 rounded-lg border-2 border-surface text-secondary font-medium hover:border-white/60 hover:text-white/80 transition-colors text-sm"
        >
          Cancelar
        </button>
        <button
          type="button"
          disabled={creatingEmpresa || !nuevaEmpresaNombre.trim()}
          onclick={handleSaveEmpresa}
          class="px-4 py-2 rounded-lg border-2 border-surface text-secondary font-medium hover:border-success hover:text-success transition-colors text-sm"
        >
          {creatingEmpresa ? "Guardando..." : "Guardar"}
        </button>
      </div>
    </div>
  </div>
{/if}
