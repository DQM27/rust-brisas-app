<script lang="ts">
  import { Upload, FileSpreadsheet, X } from 'lucide-svelte';

  interface Props {
    loading?: boolean;
    selectedFile: File | null;
    onFileSelect: (file: File | null) => void;
    onSubmit: () => void;
    onOpenFileDialog: () => void;
  }

  let { loading = false, selectedFile, onFileSelect, onSubmit, onOpenFileDialog }: Props = $props();

  function handleClearFile() {
    onFileSelect(null);
  }

  const isFormValid = $derived(selectedFile !== null);
  const fileName = $derived(selectedFile?.name || '');
  const fileSize = $derived(selectedFile ? (selectedFile.size / 1024).toFixed(2) : '0');

  // Clases dinámicas
  const dropZoneClass = $derived(
    `relative border-2 border-dashed rounded-lg p-12 text-center bg-[#1e1e1e] transition-all duration-200 ${
      selectedFile 
        ? 'border-green-500/50 bg-green-500/5 cursor-default'
        : 'border-white/20 hover:border-blue-500/50 hover:bg-blue-500/5'
    }`
  );
</script>

<div class="w-full max-w-3xl mx-auto p-6 bg-[#252526] rounded-lg border border-white/10">
  <!-- Header -->
  <div class="mb-6 text-center">
    <h2 class="text-2xl font-semibold text-white mb-2">
      Importar Lista Negra desde Excel
    </h2>
    <p class="text-sm text-gray-400">
      Sube un archivo Excel con la información de personas para agregar a la lista negra
    </p>
  </div>

  <!-- Drop Zone -->
  <div class={dropZoneClass}>
    {#if selectedFile}
      <div class="flex items-center justify-center gap-4">
        <FileSpreadsheet size={48} class="text-blue-400" />
        <div class="text-left">
          <p class="text-sm font-medium text-white mb-1">{fileName}</p>
          <p class="text-xs text-gray-400">{fileSize} KB</p>
        </div>
        <button
          onclick={handleClearFile}
          disabled={loading}
          class="flex items-center justify-center w-8 h-8 bg-red-500/10 border border-red-500/20 rounded text-red-400 hover:bg-red-500/20 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
          title="Quitar archivo"
        >
          <X size={20} />
        </button>
      </div>
    {:else}
      <button
        onclick={onOpenFileDialog}
        disabled={loading}
        class="w-full h-full flex flex-col items-center justify-center bg-transparent border-none cursor-pointer transition-opacity hover:opacity-80 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <Upload size={48} class="text-gray-500" />
        <p class="text-base text-white mt-4 mb-1">Haz clic para seleccionar un archivo Excel</p>
        <p class="text-sm text-gray-400">.xlsx, .xls, .ods</p>
      </button>
    {/if}
  </div>

  <!-- Formato esperado -->
  <div class="my-6 p-4 bg-[#1e1e1e] rounded border border-white/10">
    <h3 class="text-sm font-semibold text-white mb-3">📋 Formato esperado del Excel:</h3>
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
      <div class="flex gap-2 text-sm">
        <span class="text-blue-400 font-medium">Columna A:</span>
        <span class="text-gray-300">Cédula</span>
      </div>
      <div class="flex gap-2 text-sm">
        <span class="text-blue-400 font-medium">Columna B:</span>
        <span class="text-gray-300">Nombre Completo</span>
      </div>
      <div class="flex gap-2 text-sm">
        <span class="text-blue-400 font-medium">Columna C:</span>
        <span class="text-gray-300">Empresa</span>
      </div>
      <div class="flex gap-2 text-sm opacity-70">
        <span class="text-blue-400 font-medium">Columna D:</span>
        <span class="text-gray-300">Motivo (opcional)</span>
      </div>
      <div class="flex gap-2 text-sm opacity-70">
        <span class="text-blue-400 font-medium">Columna E:</span>
        <span class="text-gray-300">Fecha Inicio (opcional)</span>
      </div>
      <div class="flex gap-2 text-sm opacity-70">
        <span class="text-blue-400 font-medium">Columna F:</span>
        <span class="text-gray-300">Observaciones (opcional)</span>
      </div>
    </div>
  </div>

  <!-- Botón de procesar -->
  <button
    onclick={onSubmit}
    disabled={loading || !isFormValid}
    class="flex items-center justify-center gap-2 w-full px-6 py-3 bg-[#007acc] rounded text-white text-sm font-medium transition-colors hover:bg-[#005a9e] disabled:opacity-50 disabled:cursor-not-allowed"
  >
    {#if loading}
      <span class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
      Procesando...
    {:else}
      <Upload size={18} />
      Procesar Archivo
    {/if}
  </button>
</div>