<!-- ListaNegraForm.svelte - REFACTORIZADO -->
<script lang="ts">
  import { currentUser } from "$lib/stores/auth";
  import type { SearchResult } from "$lib/types/search.types";
  import type { BlockCheckResponse } from "$lib/types/listaNegra";
  import { listaNegra } from "$lib/api/listaNegra";

  // Componentes
  import BlacklistFormModeSelector from "./blacklistForm/BlacklistFormModeSelector.svelte";
  import BlacklistContratistaSearch from "./blacklistForm/BlacklistContratistaSearch.svelte";
  import BlacklistContratistaInfo from "./blacklistForm/BlacklistContratistaInfo.svelte";
  import BlacklistManualInputs from "./blacklistForm/BlacklistManualInputs.svelte";
  import BlacklistReasonInputs from "./blacklistForm/BlacklistReasonInputs.svelte";
  import BlacklistConfirmModal from "./blacklistForm/BlacklistConfirmModal.svelte";

  interface Props {
    loading?: boolean;
    onSubmit: (data: any) => void;
    onUnblock?: (data: {
      id: string;
      motivoDesbloqueo: string;
      observaciones?: string;
    }) => void;
  }

  let { loading = false, onSubmit, onUnblock }: Props = $props();

  // Estado del formulario
  let modoRegistro: "existente" | "manual" = $state("existente");

  // Modo existente
  let contratistaId = $state("");
  let selectedContratista: {
    id: string;
    nombreCompleto: string;
    cedula: string;
    empresaNombre?: string;
  } | null = $state(null);

  // Verificación de bloqueo
  let checkingBlock = $state(false);
  let blockInfo: BlockCheckResponse | null = $state(null);
  let bloqueadoId: string | null = $state(null);

  // Modo manual
  let cedula = $state("");
  let nombre = $state("");
  let segundoNombre = $state("");
  let apellido = $state("");
  let segundoApellido = $state("");

  // Datos del bloqueo/desbloqueo
  let motivoBloqueo = $state("");
  let observaciones = $state("");

  // Modal de confirmación
  let showConfirmModal = $state(false);

  // Función de reset exportada
  export function reset() {
    modoRegistro = "existente";
    contratistaId = "";
    selectedContratista = null;
    blockInfo = null;
    bloqueadoId = null;
    checkingBlock = false;
    cedula = "";
    nombre = "";
    segundoNombre = "";
    apellido = "";
    segundoApellido = "";
    motivoBloqueo = "";
    observaciones = "";
    showConfirmModal = false;
  }

  // Manejadores de modo
  function handleModeChange(mode: "existente" | "manual") {
    modoRegistro = mode;
    clearSelection();
  }

  function clearSelection() {
    selectedContratista = null;
    contratistaId = "";
    blockInfo = null;
    bloqueadoId = null;
    motivoBloqueo = "";
    observaciones = "";
  }

  // Manejadores de búsqueda
  async function handleContratistaSelect(result: SearchResult) {
    selectedContratista = {
      id: result.id,
      nombreCompleto: result.nombreCompleto || `ID: ${result.id}`,
      cedula: result.cedula || "",
      empresaNombre: result.empresaNombre ?? undefined,
    };
    contratistaId = result.id;
    motivoBloqueo = "";
    observaciones = "";

    // Verificar si está bloqueado
    if (result.cedula) {
      await checkIfBlocked(result.cedula);
    }
  }

  async function checkIfBlocked(cedulaToCheck: string) {
    checkingBlock = true;
    blockInfo = null;
    bloqueadoId = null;

    try {
      const check = await listaNegra.checkIsBlocked(cedulaToCheck);
      blockInfo = check;

      if (check.isBlocked) {
        const bloqueado = await listaNegra.getByCedula(cedulaToCheck);
        if (bloqueado) {
          bloqueadoId = bloqueado.id;
        }
      }
    } catch (error) {
      console.error("Error al verificar bloqueo:", error);
    }

    checkingBlock = false;
  }

  // Manejadores de modal
  function handleUnblock() {
    showConfirmModal = true;
  }

  function confirmUnblock() {
    if (bloqueadoId && onUnblock) {
      onUnblock({
        id: bloqueadoId,
        motivoDesbloqueo: motivoBloqueo,
        observaciones: observaciones.trim() || undefined,
      });
    }
    showConfirmModal = false;
  }

  // Submit
  function handleSubmit(e: Event) {
    e.preventDefault();

    const usuario = $currentUser;
    if (!usuario) {
      console.error("No hay usuario autenticado");
      return;
    }

    const bloqueadoPor = `${usuario.nombre} ${usuario.apellido}`;

    const data: any = {
      motivoBloqueo,
      bloqueadoPor,
      observaciones: observaciones.trim() || undefined,
    };

    if (modoRegistro === "existente") {
      data.contratistaId = contratistaId;
    } else {
      data.cedula = cedula;
      data.nombre = nombre;
      data.segundoNombre = segundoNombre.trim() || undefined;
      data.apellido = apellido;
      data.segundoApellido = segundoApellido.trim() || undefined;
    }

    onSubmit(data);
  }

  // Validación
  let isManualValid = $derived(
    modoRegistro === "manual" &&
      cedula.trim() &&
      nombre.trim() &&
      apellido.trim(),
  );

  let isExistenteValid = $derived(
    modoRegistro === "existente" && contratistaId.trim() && !checkingBlock,
  );

  let isFormValid = $derived(
    motivoBloqueo.trim() && (isExistenteValid || isManualValid),
  );
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <div class="card-base bg-surface-1 p-0 w-full max-w-2xl">
    <!-- Header -->
    <div
      class="border-b border-border-subtle bg-surface-2 px-6 py-4 rounded-t-lg"
    >
      <div class="flex items-center gap-3">
        <div
          class="flex h-8 w-8 items-center justify-center rounded-md bg-surface-1 border border-border-subtle"
        >
          <svg
            class="h-5 w-5 text-error"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
            />
          </svg>
        </div>
        <div>
          <h2 class="text-base font-semibold text-primary">
            Agregar a Lista Negra
          </h2>
          <p class="text-xs text-tertiary">
            Bloquear acceso a las instalaciones.
          </p>
        </div>
      </div>
    </div>

    <!-- Form -->
    <form onsubmit={handleSubmit}>
      <div class="p-6">
        <BlacklistFormModeSelector
          {modoRegistro}
          {loading}
          onModeChange={handleModeChange}
        />

        <div class="mt-6">
          {#if modoRegistro === "existente"}
            <div class="space-y-5">
              <BlacklistContratistaSearch
                {loading}
                onSelect={handleContratistaSelect}
                onClear={clearSelection}
              />

              {#if selectedContratista}
                <BlacklistContratistaInfo
                  contratista={selectedContratista}
                  {blockInfo}
                  {checkingBlock}
                  onClear={clearSelection}
                />

                <BlacklistReasonInputs
                  motivo={motivoBloqueo}
                  {observaciones}
                  isUnblock={blockInfo?.isBlocked || false}
                  {loading}
                  onMotivoChange={(v) => (motivoBloqueo = v)}
                  onObservacionesChange={(v) => (observaciones = v)}
                />
              {/if}
            </div>
          {:else}
            <div class="space-y-5">
              <BlacklistManualInputs
                {cedula}
                {nombre}
                {segundoNombre}
                {apellido}
                {segundoApellido}
                {loading}
                onCedulaChange={(v) => (cedula = v)}
                onNombreChange={(v) => (nombre = v)}
                onSegundoNombreChange={(v) => (segundoNombre = v)}
                onApellidoChange={(v) => (apellido = v)}
                onSegundoApellidoChange={(v) => (segundoApellido = v)}
              />

              <BlacklistReasonInputs
                motivo={motivoBloqueo}
                {observaciones}
                isUnblock={false}
                {loading}
                onMotivoChange={(v) => (motivoBloqueo = v)}
                onObservacionesChange={(v) => (observaciones = v)}
              />
            </div>
          {/if}
        </div>
      </div>

      <!-- Actions -->
      <div
        class="bg-surface-2 border-t border-border-subtle px-4 py-3 flex justify-end gap-3 rounded-b-lg"
      >
        {#if selectedContratista && blockInfo?.isBlocked && !checkingBlock}
          <button
            type="button"
            onclick={handleUnblock}
            disabled={loading || !motivoBloqueo.trim()}
            class="btn-success"
          >
            Desbloquear
          </button>
        {/if}

        {#if (modoRegistro === "manual" && !loading) || (selectedContratista && !blockInfo?.isBlocked && !checkingBlock)}
          <button
            type="submit"
            disabled={loading || !isFormValid}
            class="inline-flex justify-center items-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {#if loading}
              <svg
                class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  class="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  stroke-width="4"
                ></circle>
                <path
                  class="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
              </svg>
              Bloqueando...
            {:else}
              Bloquear Persona
            {/if}
          </button>
        {/if}
      </div>
    </form>
  </div>
</div>

<BlacklistConfirmModal
  show={showConfirmModal}
  contratistaName={selectedContratista?.nombreCompleto || ""}
  motivo={motivoBloqueo}
  {observaciones}
  onConfirm={confirmUnblock}
  onCancel={() => (showConfirmModal = false)}
  onMotivoChange={(v) => (motivoBloqueo = v)}
  onObservacionesChange={(v) => (observaciones = v)}
/>
