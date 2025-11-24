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

  console.log('🎬 Container inicializado con userId:', userId);

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
    console.log('🔵 handleOpenFileDialog llamado');
    try {
      const result = await open({
        multiple: false,
        filters: [{
          name: 'Excel',
          extensions: ['xlsx', 'xls', 'ods']
        }]
      });

      console.log('📂 Resultado del diálogo:', result);

      if (result) {
        const path = result as string;
        console.log('📁 File path:', path);
        filePath = path;
        // Crear un File object mock con el nombre
        const fileName = path.split('/').pop() || path.split('\\').pop() || 'archivo.xlsx';
        console.log('📄 File name:', fileName);
        const mockFile = new File([], fileName, { type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' });
        selectedFile = mockFile;
        console.log('✅ File seleccionado correctamente');
      }
    } catch (err) {
      console.error('❌ Error selecting file:', err);
      toast.error('Error al seleccionar archivo');
    }
  }

  // Paso 1: Seleccionar archivo
  function handleFileSelect(file: File | null) {
    console.log('🔵 handleFileSelect llamado con:', file);
    selectedFile = file;
    if (!file) {
      filePath = null;
      console.log('🗑️ File limpiado');
    }
  }

  // Paso 2: Procesar/Preview del Excel
  async function handleSubmitPreview() {
    console.log('🔵 handleSubmitPreview llamado');
    console.log('📁 filePath actual:', filePath);
    
    if (!filePath) {
      console.error('❌ No hay filePath');
      toast.error('Debe seleccionar un archivo primero');
      return;
    }

    console.log('🔄 Procesando archivo:', filePath);
    loading = true;
    console.log('⏳ Loading activado');

    console.log('📞 Llamando submitPreview...');
    const result = await submitPreview(filePath, true);
    console.log('📥 Resultado de submitPreview:', result);
    console.log('📥 result.ok:', result.ok);
    
    if (result.ok) {
      console.log('✅ submitPreview exitoso');
      console.log('📊 result.result:', result.result);
      console.log('📊 result.result.entries.length:', result.result.entries.length);
      
      // Convertir entries a array normal
      parseResult = {
        ...result.result,
        entries: [...result.result.entries]
      };
      
      console.log('✅ parseResult asignado');
      step = 'summary';
      console.log('✅ step cambiado a summary');
      toast.success('Archivo procesado correctamente');
    } else {
      console.error('❌ Error en preview:', result.error);
      toast.error(result.error, { duration: 5000 });
    }

    loading = false;
    console.log('⏳ Loading desactivado');
  }

  // Paso 3: Desde Summary - Continuar
  async function handleContinueFromSummary() {
    console.log('🔵 handleContinueFromSummary llamado');
    console.log('⚠️ entriesNeedingReview.length:', entriesNeedingReview.length);
    console.log('✅ validEntries.length:', validEntries.length);

    if (entriesNeedingReview.length > 0) {
      // Hay entradas que necesitan revisión manual
      console.log('🔍 Abriendo modal de validación');
      showValidationModal = true;
      console.log('✅ showValidationModal =', showValidationModal);
    } else {
      // Todas las entradas son válidas, importar directamente
      console.log('✅ Importando solo válidas (sin modal)');
      await handleImportValidEntries();
    }
  }

  // Paso 4: Cancelar desde Summary
  function handleCancelFromSummary() {
    console.log('🔵 handleCancelFromSummary llamado');
    resetWizard();
  }

  // Paso 5: Guardar correcciones manuales
  async function handleSaveCorrections(correctedEntries: BlacklistImportEntry[]) {
    console.log('🔵 handleSaveCorrections llamado');
    console.log('💾 Guardando correcciones:', correctedEntries);
    console.log('💾 Cantidad de entradas corregidas:', correctedEntries.length);
    
    showValidationModal = false;
    loading = true;

    try {
      let totalImported = 0;

      // 1. Importar las entradas corregidas manualmente
      if (correctedEntries.length > 0) {
        console.log('📝 Preparando entradas corregidas para importar...');
        console.log('📝 Cantidad:', correctedEntries.length);
        
        const inputs: CreateBlacklistImportInput[] = correctedEntries.map((entry, idx) => {
          const input = {
            cedula: entry.cedula,
            primerNombre: entry.primerNombre,
            segundoNombre: entry.segundoNombre,
            primerApellido: entry.primerApellido,
            segundoApellido: entry.segundoApellido,
            empresa: entry.empresa,
            motivoBloqueo: entry.motivoBloqueo,
            fechaInicioBloqueo: entry.fechaInicioBloqueo,
            observaciones: entry.observaciones
          };
          console.log(`📝 Input ${idx}:`, JSON.stringify(input, null, 2));
          return input;
        });

        console.log('📞 Llamando importReviewedEntries con:', {
          entriesCount: inputs.length,
          userId,
          firstEntry: inputs[0]
        });
        
        const reviewedResult = await importReviewedEntries(inputs, userId);
        console.log('📥 Resultado de entradas corregidas:', reviewedResult);
        console.log('📥 reviewedResult.successful:', reviewedResult.successful);
        console.log('📥 reviewedResult.failed:', reviewedResult.failed);
        console.log('📥 reviewedResult.errors:', reviewedResult.errors);
        
        totalImported += reviewedResult.successful;
        console.log('➕ Total importado después de corregidas:', totalImported);

        if (reviewedResult.failed > 0) {
          console.error('❌ Algunas entradas corregidas fallaron:', reviewedResult.failed);
          toast.error(`${reviewedResult.failed} entradas corregidas fallaron al importar`, { duration: 4000 });
        }
      }

      // 2. Importar las que ya eran válidas desde el Excel
      if (filePath && validEntries.length > 0) {
        console.log('✅ Importando entradas válidas del Excel...');
        console.log('✅ Cantidad de válidas:', validEntries.length);
        console.log('✅ filePath:', filePath);
        console.log('✅ userId:', userId);
        
        console.log('📞 Llamando submitImport...');
        const importResult = await submitImport(filePath, userId, true);
        console.log('📥 Resultado de entradas válidas:', importResult);
        console.log('📥 importResult.ok:', importResult.ok);
        
        if (importResult.ok) {
          console.log('✅ submitImport exitoso');
          console.log('📥 importResult.result:', importResult.result);
          console.log('📥 importResult.result.successful:', importResult.result.successful);
          totalImported += importResult.result.successful;
          console.log('➕ Total importado después de válidas:', totalImported);
        } else {
          console.error('❌ submitImport falló:', importResult.error);
        }
      } else {
        console.log('⚠️ No hay entradas válidas para importar');
        console.log('⚠️ filePath:', filePath);
        console.log('⚠️ validEntries.length:', validEntries.length);
      }

      console.log('✅ Total final importado:', totalImported);
      toast.success(
        `✓ Importación completada: ${totalImported} registros importados`,
        { duration: 4000 }
      );

      console.log('📞 Llamando onSuccess callback');
      onSuccess?.();
      
      console.log('🔄 Reseteando wizard');
      resetWizard();
    } catch (err: any) {
      console.error('❌ Error en handleSaveCorrections:', err);
      console.error('❌ Error stack:', err.stack);
      const errorMessage = parseImportError(err);
      console.error('❌ Error parseado:', errorMessage);
      toast.error(errorMessage, { duration: 5000 });
    }

    loading = false;
    console.log('⏳ Loading desactivado');
  }

  // Importar solo entradas válidas (sin correcciones)
  async function handleImportValidEntries() {
    console.log('🔵 handleImportValidEntries llamado');
    console.log('📁 filePath:', filePath);
    console.log('👤 userId:', userId);

    if (!filePath) {
      console.error('❌ No hay filePath');
      return;
    }

    loading = true;
    console.log('⏳ Loading activado');

    console.log('🔄 Llamando submitImport...');
    console.log('📞 Parámetros:', { filePath, userId, skipHeader: true });
    
    const result = await submitImport(filePath, userId, true);
    console.log('📥 Resultado de submitImport:', result);
    console.log('📥 result.ok:', result.ok);

    if (result.ok) {
      console.log('✅ Importación exitosa');
      console.log('📊 result.result:', result.result);
      console.log('📊 result.result.successful:', result.result.successful);
      console.log('📊 result.result.failed:', result.result.failed);
      console.log('📊 result.result.errors:', result.result.errors);
      console.log('📊 result.result.successful:', result.result.successful);
console.log('📊 result.result.failed:', result.result.failed);
console.log('📊 result.result.errors:', result.result.errors);
console.log('🔍 Primer error:', JSON.stringify(result.result.errors[0], null, 2)); // ⬅️ AGREGAR ESTO
      
      toast.success(
        `✓ Importación completada: ${result.result.successful} registros importados`,
        { duration: 4000 }
      );
      
      console.log('📞 Llamando onSuccess callback');
      onSuccess?.();
      
      console.log('🔄 Reseteando wizard');
      resetWizard();
    } else {
      console.error('❌ Error en importación:', result.error);
      toast.error(result.error, { duration: 5000 });
    }

    loading = false;
    console.log('⏳ Loading desactivado');
  }

  // Reset wizard
  function resetWizard() {
    console.log('🔵 resetWizard llamado');
    step = 'upload';
    selectedFile = null;
    filePath = null;
    parseResult = null;
    showValidationModal = false;
    console.log('✅ Wizard reseteado completamente');
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