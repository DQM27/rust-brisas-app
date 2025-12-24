<!-- src/lib/components/contratista/ContratistaFormModal.svelte -->
<!-- Modal reutilizable para crear y editar contratistas -->
<script lang="ts">
  import { fade, fly, scale } from "svelte/transition";
  import { X } from "lucide-svelte";
  import { onMount } from "svelte";
  import type {
    ContratistaResponse,
    CreateContratistaInput,
    UpdateContratistaInput,
  } from "$lib/types/contratista";
  import type { TipoVehiculo } from "$lib/types/vehiculo";
  import {
    submitCreateEmpresa,
    submitFetchActiveEmpresas,
  } from "$lib/logic/empresa/empresaService";

  interface Props {
    show: boolean;
    contratista?: ContratistaResponse | null;
    loading?: boolean;
    onSave: (
      data: CreateContratistaInput | UpdateContratistaInput,
    ) => Promise<boolean | void>;
    onClose: () => void;
  }

  let {
    show,
    contratista = null,
    loading = false,
    onSave,
    onClose,
  }: Props = $props();

  // Modo derivado
  const isEditMode = $derived(!!contratista);
  const modalTitle = $derived(
    isEditMode
      ? `Editar: ${contratista?.nombre} ${contratista?.apellido}`
      : "Nuevo Contratista",
  );

  // Estado del formulario
  let formData = $state({
    cedula: "",
    nombre: "",
    segundoNombre: "",
    apellido: "",
    segundoApellido: "",
    empresaId: "",
    fechaVencimientoPraind: "",
    tieneVehiculo: false,
    tipoVehiculo: "" as TipoVehiculo | "",
    placa: "",
    marca: "",
    modelo: "",
    color: "",
  });

  // Empresas
  let empresas = $state<{ id: string; nombre: string }[]>([]);
  let loadingEmpresas = $state(false);
  let showEmpresaModal = $state(false);
  let nuevaEmpresaNombre = $state("");
  let creatingEmpresa = $state(false);
  let empresaError = $state("");

  // Validación
  const isFormValid = $derived(
    formData.cedula.trim() &&
      formData.nombre.trim() &&
      formData.apellido.trim() &&
      formData.empresaId.trim() &&
      formData.fechaVencimientoPraind.trim() &&
      (!formData.tieneVehiculo ||
        (formData.tipoVehiculo && formData.placa.trim())),
  );

  // Cargar empresas al montar
  onMount(async () => {
    await loadEmpresas();
  });

  async function loadEmpresas() {
    if (empresas.length > 0) return;
    loadingEmpresas = true;
    const resultado = await submitFetchActiveEmpresas();
    if (resultado.ok) {
      empresas = resultado.empresas;
    }
    loadingEmpresas = false;
  }

  // Helper: Convertir YYYY-MM-DD → DD-MM-YYYY
  function formatDateForDisplay(isoDate: string): string {
    if (!isoDate) return "";
    const [year, month, day] = isoDate.split("T")[0].split("-");
    return `${day}-${month}-${year}`;
  }

  // Helper: Convertir DD-MM-YYYY → YYYY-MM-DD
  function formatDateForBackend(displayDate: string): string {
    if (!displayDate || displayDate.length !== 10) return "";
    const [day, month, year] = displayDate.split("-");
    return `${year}-${month}-${day}`;
  }

  // Cargar datos del contratista cuando se abre en modo edición
  $effect(() => {
    if (show && contratista) {
      formData = {
        cedula: contratista.cedula || "",
        nombre: contratista.nombre || "",
        segundoNombre: "",
        apellido: contratista.apellido || "",
        segundoApellido: "",
        empresaId: contratista.empresaId || "",
        fechaVencimientoPraind: formatDateForDisplay(
          contratista.fechaVencimientoPraind || "",
        ),
        tieneVehiculo: !!(
          contratista.vehiculoTipo || contratista.vehiculoPlaca
        ),
        tipoVehiculo: (contratista.vehiculoTipo as TipoVehiculo) || "",
        placa: contratista.vehiculoPlaca || "",
        marca: "",
        modelo: "",
        color: "",
      };
    } else if (show && !contratista) {
      resetForm();
    }
  });

  function resetForm() {
    formData = {
      cedula: "",
      nombre: "",
      segundoNombre: "",
      apellido: "",
      segundoApellido: "",
      empresaId: "",
      fechaVencimientoPraind: "",
      tieneVehiculo: false,
      tipoVehiculo: "",
      placa: "",
      marca: "",
      modelo: "",
      color: "",
    };
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!isFormValid) return;

    const data = {
      cedula: formData.cedula,
      nombre: formData.nombre,
      apellido: formData.apellido,
      empresaId: formData.empresaId,
      fechaVencimientoPraind: formatDateForBackend(
        formData.fechaVencimientoPraind,
      ),
      tieneVehiculo: formData.tieneVehiculo,
      tipoVehiculo: formData.tipoVehiculo || undefined,
      placa: formData.placa,
      marca: formData.marca,
      modelo: formData.modelo,
      color: formData.color,
    };

    const success = await onSave(data as CreateContratistaInput);
    if (success) {
      onClose();
    }
  }

  async function handleCrearEmpresa() {
    if (!nuevaEmpresaNombre.trim()) return;
    creatingEmpresa = true;
    empresaError = "";
    const result = await submitCreateEmpresa(nuevaEmpresaNombre);
    if (result.ok) {
      empresas = [
        ...empresas,
        { id: result.empresa.id, nombre: result.empresa.nombre },
      ];
      formData.empresaId = result.empresa.id;
      nuevaEmpresaNombre = "";
      showEmpresaModal = false;
    } else {
      empresaError = result.error;
    }
    creatingEmpresa = false;
  }

  function handleClose() {
    if (!loading) {
      onClose();
    }
  }

  // Estilos compartidos
  const labelClass = "text-xs font-medium text-gray-700 dark:text-gray-300";
  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent focus:outline-none transition-all placeholder-gray-400 dark:placeholder-gray-500 disabled:opacity-50";
</script>

{#if show}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="absolute inset-0 bg-black/60" onclick={handleClose}></div>

    <!-- Modal -->
    <div
      class="relative z-10 w-full max-w-md max-h-[90vh] overflow-hidden rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22]"
      >
        <div>
          <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
            {modalTitle}
          </h2>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
            {isEditMode
              ? "Modifique los datos necesarios"
              : "Ingresa los datos del contratista"}
          </p>
        </div>
        <button
          type="button"
          onclick={handleClose}
          disabled={loading}
          class="p-1.5 rounded-md text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Form -->
      <form
        onsubmit={handleSubmit}
        class="overflow-y-auto max-h-[calc(90vh-180px)]"
      >
        <div class="p-6 space-y-4">
          <!-- Cédula -->
          <div class="space-y-1">
            <label for="cedula" class={labelClass}
              >Cédula <span class="text-red-500">*</span></label
            >
            <input
              id="cedula"
              type="text"
              bind:value={formData.cedula}
              placeholder="1-2345-6789"
              disabled={loading}
              class={inputClass}
            />
          </div>

          <!-- Nombre / Apellido -->
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-1">
              <label for="nombre" class={labelClass}
                >Nombre <span class="text-red-500">*</span></label
              >
              <input
                id="nombre"
                type="text"
                bind:value={formData.nombre}
                placeholder="Juan"
                disabled={loading}
                class={inputClass}
              />
            </div>
            <div class="space-y-1">
              <label for="apellido" class={labelClass}
                >Apellido <span class="text-red-500">*</span></label
              >
              <input
                id="apellido"
                type="text"
                bind:value={formData.apellido}
                placeholder="Pérez"
                disabled={loading}
                class={inputClass}
              />
            </div>
          </div>

          <!-- Empresa -->
          <div class="space-y-1">
            <label for="empresaId" class={labelClass}
              >Empresa <span class="text-red-500">*</span></label
            >
            <div class="flex gap-2">
              <div class="relative flex-1">
                <select
                  id="empresaId"
                  bind:value={formData.empresaId}
                  disabled={loading || loadingEmpresas}
                  class={inputClass}
                >
                  <option value="" disabled>
                    {loadingEmpresas ? "Cargando..." : "Seleccione empresa"}
                  </option>
                  {#each empresas as empresa}
                    <option value={empresa.id}>{empresa.nombre}</option>
                  {/each}
                </select>
              </div>
              <button
                type="button"
                onclick={() => (showEmpresaModal = true)}
                disabled={loading}
                class="px-3 py-2 rounded-md border border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-[#21262d] text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#30363d] transition-colors text-sm"
              >
                +
              </button>
            </div>
          </div>

          <!-- Fecha PRAIND -->
          <div class="space-y-1">
            <label for="fechaPraind" class={labelClass}
              >Vencimiento PRAIND <span class="text-red-500">*</span></label
            >
            <input
              id="fechaPraind"
              type="text"
              bind:value={formData.fechaVencimientoPraind}
              placeholder="DD-MM-YYYY"
              maxlength="10"
              disabled={loading}
              class={inputClass}
              oninput={(e) => {
                const input = e.target as HTMLInputElement;
                let value = input.value.replace(/[^\d-]/g, ""); // Solo números y guiones

                // Auto-formatear como DD-MM-YYYY
                if (value.length >= 3 && value[2] !== "-") {
                  value = value.slice(0, 2) + "-" + value.slice(2);
                }
                if (value.length >= 6 && value[5] !== "-") {
                  value = value.slice(0, 5) + "-" + value.slice(5);
                }

                // Limitar a 10 caracteres
                value = value.slice(0, 10);

                formData.fechaVencimientoPraind = value;
                input.value = value;
              }}
            />
          </div>

          <!-- Toggle Vehículo -->
          <div
            class="flex items-center justify-between p-3 rounded-md border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22]"
          >
            <span class={labelClass}>¿Agregar Vehículo?</span>
            <button
              type="button"
              role="switch"
              aria-checked={formData.tieneVehiculo}
              aria-label="Agregar vehículo"
              onclick={() => (formData.tieneVehiculo = !formData.tieneVehiculo)}
              class="relative inline-flex h-5 w-9 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-[#2da44e] {formData.tieneVehiculo
                ? 'bg-[#2da44e]'
                : 'bg-gray-300 dark:bg-gray-600'}"
            >
              <span
                class="pointer-events-none inline-block h-4 w-4 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {formData.tieneVehiculo
                  ? 'translate-x-4'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>

          <!-- Sección Vehículo -->
          {#if formData.tieneVehiculo}
            <div
              class="p-4 rounded-md border border-gray-200 dark:border-gray-700 bg-gray-50/50 dark:bg-[#161b22]/50 space-y-4"
              transition:fly={{ y: -10, duration: 200 }}
            >
              <h4 class="text-sm font-medium text-gray-900 dark:text-gray-100">
                Datos del Vehículo
              </h4>

              <!-- Tipo Vehículo -->
              <div class="grid grid-cols-2 gap-2">
                <button
                  type="button"
                  onclick={() => (formData.tipoVehiculo = "motocicleta")}
                  class="py-2 px-3 rounded-md border text-sm font-medium transition-all {formData.tipoVehiculo ===
                  'motocicleta'
                    ? 'border-[#2da44e] bg-[#2da44e]/10 text-[#2da44e]'
                    : 'border-gray-300 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:border-gray-400'}"
                >
                  🏍️ Moto
                </button>
                <button
                  type="button"
                  onclick={() => (formData.tipoVehiculo = "automovil")}
                  class="py-2 px-3 rounded-md border text-sm font-medium transition-all {formData.tipoVehiculo ===
                  'automovil'
                    ? 'border-[#2da44e] bg-[#2da44e]/10 text-[#2da44e]'
                    : 'border-gray-300 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:border-gray-400'}"
                >
                  🚗 Auto
                </button>
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div class="space-y-1">
                  <label for="placa" class={labelClass}
                    >Placa <span class="text-red-500">*</span></label
                  >
                  <input
                    id="placa"
                    type="text"
                    bind:value={formData.placa}
                    placeholder="ABC-123"
                    disabled={loading}
                    class="{inputClass} uppercase"
                  />
                </div>
                <div class="space-y-1">
                  <label for="marca" class={labelClass}>Marca</label>
                  <input
                    id="marca"
                    type="text"
                    bind:value={formData.marca}
                    placeholder="Toyota"
                    disabled={loading}
                    class={inputClass}
                  />
                </div>
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div class="space-y-1">
                  <label for="modelo" class={labelClass}>Modelo</label>
                  <input
                    id="modelo"
                    type="text"
                    bind:value={formData.modelo}
                    placeholder="Corolla"
                    disabled={loading}
                    class={inputClass}
                  />
                </div>
                <div class="space-y-1">
                  <label for="color" class={labelClass}>Color</label>
                  <input
                    id="color"
                    type="text"
                    bind:value={formData.color}
                    placeholder="Blanco"
                    disabled={loading}
                    class={inputClass}
                  />
                </div>
              </div>
            </div>
          {/if}
        </div>

        <!-- Footer -->
        <div
          class="flex items-center justify-end gap-3 px-6 py-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#161b22]"
        >
          <button
            type="button"
            onclick={handleClose}
            disabled={loading}
            class="px-4 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d] transition-colors disabled:opacity-50"
          >
            Cancelar
          </button>
          <button
            type="submit"
            disabled={loading || !isFormValid}
            class="px-4 py-2 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading
              ? "Guardando..."
              : isEditMode
                ? "Guardar Cambios"
                : "Registrar"}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Modal Nueva Empresa -->
{#if showEmpresaModal}
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
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
            onkeydown={(e) => e.key === "Enter" && handleCrearEmpresa()}
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
          onclick={handleCrearEmpresa}
          class="px-3 py-1.5 text-xs font-medium rounded-md bg-[#2da44e] text-white hover:bg-[#2c974b] disabled:opacity-50"
        >
          {creatingEmpresa ? "Guardando..." : "Guardar"}
        </button>
      </div>
    </div>
  </div>
{/if}
