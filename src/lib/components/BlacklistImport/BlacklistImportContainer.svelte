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

  // Entradas válidas (para importar directamente)
  const validEntries = $derived(
    parseResult?.entries.filter(e => e.validationStatus === 'valid') || []
  );

  // DEBUG: Ver qué se está recibiendo
  $effect(() => {
    if (parseResult) {
      console.log('📊 Parse Result completo:', parseResult);
      console.log('📈 Total rows:', parseResult.totalRows);
      console.log('✅ Successful:', parseResult.successful);
      console.log('⚠️ Needs review:', parseResult.needsReview);
      console.log('❌ Failed:', parseResult.failed);
      console.log('📋 Entries array length:', parseResult.entries.length);
      console.log('📋 Entries:', parseResult.entries);
      
      if (parseResult.entries.length > 0) {
        console.log('🔍 Primera entrada completa:', JSON.parse(JSON.stringify(parseResult.entries[0])));
        console.log('🔍 validationStatus de primera entrada:', parseResult.entries[0].validationStatus);
        console.log('🔍 Tipo de validationStatus:', typeof parseResult.entries[0].validationStatus);
        
        // Ver todas las validationStatus
        const statuses = parseResult.entries.map(e => e.validationStatus);
        console.log('🔍 Todos los validationStatus:', statuses);
        console.log('🔍 Únicos:', [...new Set(statuses)]);
      }
      
      console.log('✅ Valid entries count:', validEntries.length);
      console.log('✅ Valid entries:', validEntries);
      console.log('⚠️ Needs review count:', entriesNeedingReview.length);
      console.log('⚠️ Needs review entries:', entriesNeedingReview);
    }
  });

  // Abrir diálogo de archivo
  async function handleOpenFileDialog() {
    try {
      const result = await open({
        multiple: false,
        filters: [{
          name: 'Excel',
          extensions: ['xlsx', 'xls', 'ods']
        }]
      });

      if (result) {
        const path = result as string;
        console.log('📁 File path:', path);
        filePath = path;
        // Crear un File object mock con el nombre
        const fileName = path.split('/').pop() || path.split('\\').pop() || 'archivo.xlsx';
        const mockFile = new File([], fileName, { type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' });
        selectedFile = mockFile;
      }
    } catch (err) {
      console.error('Error selecting file:', err);
      toast.error('Error al seleccionar archivo');
    }
  }

  // Paso 1: Seleccionar archivo
  function handleFileSelect(file: File | null) {
    selectedFile = file;
    if (!file) {
      filePath = null;
    }
  }

  // Paso 2: Procesar/Preview del Excel
  async function handleSubmitPreview() {
    if (!filePath) {
      toast.error('Debe seleccionar un archivo primero');
      return;
    }

    console.log('🔄 Procesando archivo:', filePath);
    loading = true;

    const result = await submitPreview(filePath, true);
    console.log('📥 Resultado de submitPreview:', result);

    if (result.ok) {
      // ⬇️⬇️⬇️ FIX: Convertir entries a array normal ⬇️⬇️⬇️
      parseResult = {
        ...result.result,
        entries: [...result.result.entries]
      };
      step = 'summary';
      toast.success('Archivo procesado correctamente');
    } else {
      console.error('❌ Error en preview:', result.error);
      toast.error(result.error, { duration: 5000 });
    }

    loading = false;
  }

  // Paso 3: Desde Summary - Continuar
  async function handleContinueFromSummary() {
    console.log('▶️ handleContinueFromSummary called');
    console.log('⚠️ entriesNeedingReview.length:', entriesNeedingReview.length);
    console.log('✅ validEntries.length:', validEntries.length);

    if (entriesNeedingReview.length > 0) {
      // Hay entradas que necesitan revisión manual
      console.log('🔍 Abriendo modal de validación');
      showValidationModal = true;
    } else {
      // Todas las entradas son válidas, importar directamente
      console.log('✅ Importando solo válidas');
      await handleImportValidEntries();
    }
  }

  // Paso 4: Cancelar desde Summary
  function handleCancelFromSummary() {
    resetWizard();
  }

  // Paso 5: Guardar correcciones manuales
  async function handleSaveCorrections(correctedEntries: BlacklistImportEntry[]) {
    console.log('💾 Guardando correcciones:', correctedEntries);
    showValidationModal = false;
    loading = true;

    try {
      let totalImported = 0;

      // 1. Importar las entradas corregidas manualmente
      if (correctedEntries.length > 0) {
        console.log('📝 Importando entradas corregidas:', correctedEntries.length);
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

        const reviewedResult = await importReviewedEntries(inputs, userId);
        console.log('📥 Resultado de entradas corregidas:', reviewedResult);
        totalImported += reviewedResult.successful;

        if (reviewedResult.failed > 0) {
          toast.error(`${reviewedResult.failed} entradas corregidas fallaron al importar`, { duration: 4000 });
        }
      }

      // 2. Importar las que ya eran válidas desde el Excel
      if (filePath && validEntries.length > 0) {
        console.log('✅ Importando entradas válidas:', validEntries.length);
        const importResult = await submitImport(filePath, userId, true);
        console.log('📥 Resultado de entradas válidas:', importResult);
        if (importResult.ok) {
          totalImported += importResult.result.successful;
        }
      }

      console.log('✅ Total importado:', totalImported);
      toast.success(
        `✓ Importación completada: ${totalImported} registros importados`,
        { duration: 4000 }
      );

      onSuccess?.();
      resetWizard();
    } catch (err: any) {
      console.error('❌ Error en handleSaveCorrections:', err);
      const errorMessage = parseImportError(err);
      toast.error(errorMessage, { duration: 5000 });
    }

    loading = false;
  }

  // Importar solo entradas válidas (sin correcciones)
  async function handleImportValidEntries() {
    console.log('📤 handleImportValidEntries called');
    console.log('📁 filePath:', filePath);
    console.log('👤 userId:', userId);

    if (!filePath) {
      console.error('❌ No hay filePath');
      return;
    }

    loading = true;

    console.log('🔄 Llamando submitImport...');
    const result = await submitImport(filePath, userId, true);
    console.log('📥 Resultado de submitImport:', result);

    if (result.ok) {
      console.log('✅ Importación exitosa:', result.result.successful);
      toast.success(
        `✓ Importación completada: ${result.result.successful} registros importados`,
        { duration: 4000 }
      );
      onSuccess?.();
      resetWizard();
    } else {
      console.error('❌ Error en importación:', result.error);
      toast.error(result.error, { duration: 5000 });
    }

    loading = false;
  }

  // Reset wizard
  function resetWizard() {
    console.log('🔄 Reseteando wizard');
    step = 'upload';
    selectedFile = null;
    filePath = null;
    parseResult = null;
    showValidationModal = false;
  }
</script>

<div class="w-full min-h-screen p-10 bg-[#1e1e1e]">
  {#if step === 'upload'}
    <BlacklistImportUpload
      {loading}
      {selectedFile}
      onFileSelect={handleFileSelect}
      onSubmit={handleSubmitPreview}
      onOpenFileDialog={handleOpenFileDialog}
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