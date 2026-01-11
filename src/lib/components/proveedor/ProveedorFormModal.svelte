<script lang="ts">
  import { fade, fly, scale } from "svelte/transition";
  import { X, Car } from "lucide-svelte";
  import { empresaStore } from "$lib/stores/empresaStore.svelte";
  import VehiculoManagerModal from "$lib/components/vehiculo/VehiculoManagerModal.svelte";
  import type {
    ProveedorResponse,
    CreateProveedorInput,
    UpdateProveedorInput,
  } from "$lib/types/proveedor";
  import { submitCreateEmpresa } from "$lib/logic/empresa/empresaService";
  import { toast } from "svelte-5-french-toast";
  import { superForm } from "sveltekit-superforms";
  import { zod4 } from "sveltekit-superforms/adapters";
  import {
    CreateProveedorSchema,
    UpdateProveedorSchema,
    type CreateProveedorForm,
    type UpdateProveedorForm,
  } from "$lib/schemas/proveedorSchema";
  import PersonaFields from "$lib/components/shared/form/PersonaFields.svelte";

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
  let showVehiculoModal = $state(false);

  // Combinar tipos para el formulario
  type CombinedForm = CreateProveedorForm & UpdateProveedorForm;

  const emptyFormData: CombinedForm = {
    cedula: "",
    nombre: "",
    segundoNombre: "",
    apellido: "",
    segundoApellido: "",
    empresaId: "",
    estado: "ACTIVO",
  };

  // Superforms setup
  const { form, errors, constraints, enhance, reset, validate, tainted } =
    superForm<CombinedForm>(emptyFormData, {
      SPA: true,
      validators: zod4(
        isEditMode ? UpdateProveedorSchema : CreateProveedorSchema,
      ),
      dataType: "json",
      validationMethod: "oninput",
      resetForm: false,
      async onUpdate({ form: f }) {
        if (f.valid) {
          const success = await onSave(f.data as any);
          if (success !== false) {
            onClose();
          }
        }
      },
    });

  // Sincronizar datos cuando cambia el proveedor
  $effect(() => {
    if (show) {
      empresaStore.init();
      const newData: CombinedForm = {
        cedula: proveedor?.cedula ?? "",
        nombre: proveedor?.nombre ?? "",
        segundoNombre: proveedor?.segundoNombre ?? "",
        apellido: proveedor?.apellido ?? "",
        segundoApellido: proveedor?.segundoApellido ?? "",
        empresaId: proveedor?.empresaId ?? "",
        estado: (proveedor?.estado as any) || "ACTIVO",
      };
      reset({ data: newData });
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
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 transition-all outline-none disabled:opacity-50";

  // Handler para Ctrl+S
  function handleKeydown(e: KeyboardEvent) {
    if (!show || readonly || loading) return;
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
      e.preventDefault();
      const f = document.getElementById("proveedorForm") as HTMLFormElement;
      if (f) f.requestSubmit();
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
      class="relative z-10 w-full max-w-[400px] max-h-[95vh] overflow-hidden rounded-xl bg-surface-2 shadow-2xl border border-surface flex flex-col"
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

      <form
        id="proveedorForm"
        method="POST"
        use:enhance
        class="flex-1 overflow-y-auto flex flex-col"
      >
        <div class="p-6 flex-1">
          <div class="bg-surface-1 rounded-lg border border-surface p-5">
            <PersonaFields
              {form}
              {errors}
              {constraints}
              {validate}
              empresas={empresaStore.empresas}
              {loading}
              {isEditMode}
              onCreateEmpresa={handleCreateEmpresa}
              tableName="proveedor"
              currentId={proveedor?.id || ""}
            />
          </div>

          <!-- Gestión de Vehículos (Solo en edición) -->
          {#if isEditMode && proveedor?.id}
            <div class="mt-6 border-t border-surface pt-6">
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="text-sm font-semibold text-primary">Vehículos</h3>
                  <p class="text-xs text-secondary mt-1">
                    Gestione la flotilla asociada a este proveedor
                  </p>
                </div>
                <button
                  type="button"
                  onclick={() => (showVehiculoModal = true)}
                  class="flex items-center gap-2 px-4 py-2 text-xs font-medium rounded-lg border border-surface bg-surface-3 text-secondary hover:text-primary hover:border-white/20 transition-all"
                >
                  <Car size={14} />
                  Gestionar Flotilla
                </button>
              </div>
            </div>
          {/if}
        </div>

        <!-- Sticky Footer -->
        <div
          class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1 sticky bottom-0 z-20 mt-auto"
        >
          <button
            type="button"
            onclick={onClose}
            disabled={loading}
            class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
          >
            Cancelar
          </button>
          {#if !readonly}
            <button
              type="submit"
              disabled={loading}
              class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50 flex items-center gap-2"
            >
              {#if loading}
                <span
                  class="w-4 h-4 rounded-full border-2 border-current border-t-transparent animate-spin"
                ></span>
              {/if}
              {isEditMode ? "Guardar Cambios" : "Crear Proveedor"}
            </button>
          {/if}
        </div>
      </form>
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
        class="flex justify-end gap-2 px-5 py-3 border-t border-surface bg-surface-1"
      >
        <button
          type="button"
          disabled={creatingEmpresa}
          onclick={() => (showEmpresaModal = false)}
          class="px-3 py-1.5 text-xs font-medium rounded-lg border-2 border-surface text-secondary transition-all duration-200 hover:border-white/60 hover:text-white/80"
        >
          Cancelar
        </button>
        <button
          type="button"
          disabled={creatingEmpresa || !nuevaEmpresaNombre.trim()}
          onclick={handleSaveEmpresa}
          class="px-3 py-1.5 text-xs font-medium rounded-lg border-2 border-surface text-secondary transition-all duration-200 hover:border-success hover:text-success disabled:opacity-50"
        >
          {creatingEmpresa ? "Guardando..." : "Guardar"}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Vehiculo Modal -->
{#if showVehiculoModal && proveedor}
  <VehiculoManagerModal
    show={showVehiculoModal}
    propietarioId={proveedor.id}
    propietarioNombre={proveedor.nombre + " " + proveedor.apellido}
    onClose={() => (showVehiculoModal = false)}
  />
{/if}

<style>
  input:focus {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
    outline: none !important;
  }

  /* Autofill Fix for Dark Theme */
  input:-webkit-autofill {
    -webkit-text-fill-color: white !important;
    -webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
    transition: background-color 5000s ease-in-out 0s;
  }
</style>
