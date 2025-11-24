<script lang="ts">
  import { toast } from 'svelte-5-french-toast';
  import { open } from '@tauri-apps/plugin-dialog';
  import BlacklistImportUpload from './BlacklistImportUpload.svelte';
  import BlacklistImportSummary from './BlacklistImportSummary.svelte';
  import BlacklistImportValidationModal from './BlacklistImportValidationModal.svelte';
  import { submitPreview, submitImport } from '$lib/logic/blacklistImport/submitImport';
  import { importReviewedEntries } from '$lib/logic/blacklistImport/processExcelFile';
  import { parseImportError } from '$lib/logic/blacklistImport/parseImportErrors';
  import type { ImportResultResponse, BlacklistImportEntry, CreateBlacklistImportInput } from '$lib/types/blacklistImport.types';

  // Props
  interface Props {
    userId: string;
    onSuccess?: () => void;
  }

  let { userId, onSuccess }: Props = $props();

  // Estado
  let step = $state<'upload' | 'summary' | 'validation'>('upload');
  let loading = $state(false);
  let selectedFile = $state<File | null>(null);
  let filePath = $state<string | null>(null);
  let parseResult = $state<ImportResultResponse | null>(null);
  let showValidationModal = $state(false);

  // Entradas que requieren validación
  const entriesNeedingReview = $derived(
    parseResult?.entries.filter(e => e.validationStatus === 'needs_review') || []
  );

  // Paso 1: Seleccionar archivo
  async function handleFileSelect(file: File | null) {
    selectedFile = file;
    filePath = null;

    if (file) {
      // Guardar archivo temporalmente para Tauri
      try {
        const result = await open({
          multiple: false,
          filters: [{
            name: 'Excel',
            extensions: ['xlsx', 'xls', 'ods']
          }]
        });

        if (result) {
          filePath = result as string;
        }
      } catch (err) {
        console.error('Error selecting file:', err);
      }
    }
  }

  // Paso 2: Procesar/Preview del Excel
  async function handleSubmitPreview() {
    if (!filePath) {
      toast.error('Debe seleccionar un archivo primero');
      return;
    }

    loading = true;

    const result = await submitPreview(filePath, true);

    if (result.ok) {
      parseResult = result.result;
      step = 'summary';
      toast.success('Archivo procesado correctamente');
    } else {
      toast.error(result.error, { duration: 5000 });
    }

    loading = false;
  }

  // Paso 3: Desde Summary - Continuar
  function handleContinueFromSummary() {
    if (entriesNeedingReview.length > 0) {
      // Hay entradas que necesitan revisión manual
      showValidationModal = true;
    } else {
      // Todas las entradas son válidas, importar directamente
      handleFinalImport();
    }
  }

  // Paso 4: Cancelar desde Summary
  function handleCancelFromSummary() {
    resetWizard();
  }

  // Paso 5: Guardar correcciones manuales
  async function handleSaveCorrections(correctedEntries: BlacklistImportEntry[]) {
    showValidationModal = false;
    loading = true;

    try {
      // Convertir entradas corregidas a formato de input
      const inputs: CreateBlacklistImportInput[] = correctedEntries.map(entry => ({
        cedula: entry.cedula,
        primerNombre: entry.primerNombre,
        segundoNombre: entry.segundoNombre,
        primerApellido: entry.primerApellido,
        segundoApellido: entry.segundoApellido,
        empresa: entry.empresa,
        motivoBloqueo: entry.motivoBloqueo,
        fechaInicioBloqueo: entry.fechaInicioBloqueo,
        observaciones: entry.observaciones
      }));

      // Importar entradas corregidas
      const reviewedResult = await importReviewedEntries(inputs, userId);

      // También importar las que ya eran válidas
      if (filePath && parseResult) {
        await submitImport(filePath, userId, true);
      }

      toast.success(
        `✓ Importación completada: ${reviewedResult.successful + (parseResult?.successful || 0)} registros importados`,
        { duration: 4000 }
      );

      onSuccess?.();
      resetWizard();
    } catch (err: any) {
      const errorMessage = parseImportError(err);
      toast.error(errorMessage, { duration: 5000 });
    }

    loading = false;
  }

  // Importación final (cuando no hay correcciones)
  async function handleFinalImport() {
    if (!filePath) return;

    loading = true;

    const result = await submitImport(filePath, userId, true);

    if (result.ok) {
      toast.success(
        `✓ Importación completada: ${result.result.successful} registros importados`,
        { duration: 4000 }
      );
      onSuccess?.();
      resetWizard();
    } else {
      toast.error(result.error, { duration: 5000 });
    }

    loading = false;
  }

  // Reset wizard
  function resetWizard() {
    step = 'upload';
    selectedFile = null;
    filePath = null;
    parseResult = null;
    showValidationModal = false;
  }
</script>

<div class="container">
  {#if step === 'upload'}
    <BlacklistImportUpload
      {loading}
      {selectedFile}
      onFileSelect={handleFileSelect}
      onSubmit={handleSubmitPreview}
    />
  {:else if step === 'summary' && parseResult}
    <BlacklistImportSummary
      result={parseResult}
      onContinue={handleContinueFromSummary}
      onCancel={handleCancelFromSummary}
    />
  {/if}

  {#if showValidationModal && entriesNeedingReview.length > 0}
    <BlacklistImportValidationModal
      entries={entriesNeedingReview}
      onSave={handleSaveCorrections}
      onClose={() => (showValidationModal = false)}
    />
  {/if}
</div>

<style>
  .container {
    width: 100%;
    min-height: 100vh;
    padding: 40px 20px;
    background-color: #1e1e1e;
  }
</style>