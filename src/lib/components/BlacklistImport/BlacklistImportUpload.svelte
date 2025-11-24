<script lang="ts">
  import { Upload, FileSpreadsheet, X } from 'lucide-svelte';

  interface Props {
    loading?: boolean;
    selectedFile: File | null;
    onFileSelect: (file: File | null) => void;
    onSubmit: () => void;
  }

  let { loading = false, selectedFile, onFileSelect, onSubmit }: Props = $props();

  let fileInput: HTMLInputElement;

  function handleFileChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0] || null;
    onFileSelect(file);
  }

  function handleClearFile() {
    onFileSelect(null);
    if (fileInput) {
      fileInput.value = '';
    }
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    const file = event.dataTransfer?.files[0];
    if (file && isValidExcelFile(file)) {
      onFileSelect(file);
    }
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
  }

  function isValidExcelFile(file: File): boolean {
    const validExtensions = ['.xlsx', '.xls', '.ods'];
    return validExtensions.some(ext => file.name.toLowerCase().endsWith(ext));
  }

  const isFormValid = $derived(selectedFile !== null);
</script>

<div class="upload-container">
  <div class="upload-header">
    <h2 class="upload-title">Importar Lista Negra desde Excel</h2>
    <p class="upload-description">
      Sube un archivo Excel con la información de personas para agregar a la lista negra
    </p>
  </div>

  <!-- Drop Zone -->
  <div
    class="drop-zone"
    class:has-file={selectedFile}
    ondrop={handleDrop}
    ondragover={handleDragOver}
  >
    {#if selectedFile}
      <div class="file-info">
        <FileSpreadsheet size={48} class="text-blue-400" />
        <div class="file-details">
          <p class="file-name">{selectedFile.name}</p>
          <p class="file-size">{(selectedFile.size / 1024).toFixed(2)} KB</p>
        </div>
        <button
          onclick={handleClearFile}
          class="clear-button"
          disabled={loading}
          title="Quitar archivo"
        >
          <X size={20} />
        </button>
      </div>
    {:else}
      <Upload size={48} class="text-gray-500" />
      <p class="drop-text">Arrastra tu archivo Excel aquí</p>
      <p class="drop-subtext">o haz clic para seleccionar</p>
      <input
        bind:this={fileInput}
        type="file"
        accept=".xlsx,.xls,.ods"
        onchange={handleFileChange}
        disabled={loading}
        class="file-input"
      />
    {/if}
  </div>

  <!-- Formato esperado -->
  <div class="format-info">
    <h3 class="format-title">📋 Formato esperado del Excel:</h3>
    <div class="format-grid">
      <div class="format-item">
        <span class="format-label">Columna A:</span>
        <span class="format-value">Cédula</span>
      </div>
      <div class="format-item">
        <span class="format-label">Columna B:</span>
        <span class="format-value">Nombre Completo</span>
      </div>
      <div class="format-item">
        <span class="format-label">Columna C:</span>
        <span class="format-value">Empresa</span>
      </div>
      <div class="format-item optional">
        <span class="format-label">Columna D:</span>
        <span class="format-value">Motivo (opcional)</span>
      </div>
      <div class="format-item optional">
        <span class="format-label">Columna E:</span>
        <span class="format-value">Fecha Inicio (opcional)</span>
      </div>
      <div class="format-item optional">
        <span class="format-label">Columna F:</span>
        <span class="format-value">Observaciones (opcional)</span>
      </div>
    </div>
  </div>

  <!-- Botón de procesar -->
  <button
    onclick={onSubmit}
    disabled={loading || !isFormValid}
    class="submit-button"
  >
    {#if loading}
      <span class="loading-spinner"></span>
      Procesando...
    {:else}
      <Upload size={18} />
      Procesar Archivo
    {/if}
  </button>
</div>

<style>
  .upload-container {
    width: 100%;
    max-width: 800px;
    margin: 0 auto;
    padding: 24px;
    background-color: #252526;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .upload-header {
    margin-bottom: 24px;
    text-align: center;
  }

  .upload-title {
    font-size: 24px;
    font-weight: 600;
    color: #ffffff;
    margin: 0 0 8px 0;
  }

  .upload-description {
    font-size: 14px;
    color: #a0a0a0;
    margin: 0;
  }

  .drop-zone {
    position: relative;
    border: 2px dashed rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    padding: 48px 24px;
    text-align: center;
    background-color: #1e1e1e;
    transition: all 0.2s;
    cursor: pointer;
  }

  .drop-zone:hover {
    border-color: rgba(59, 130, 246, 0.5);
    background-color: rgba(59, 130, 246, 0.05);
  }

  .drop-zone.has-file {
    border-color: rgba(34, 197, 94, 0.5);
    background-color: rgba(34, 197, 94, 0.05);
    cursor: default;
  }

  .file-input {
    position: absolute;
    inset: 0;
    opacity: 0;
    cursor: pointer;
  }

  .drop-text {
    font-size: 16px;
    color: #ffffff;
    margin: 16px 0 4px 0;
  }

  .drop-subtext {
    font-size: 13px;
    color: #a0a0a0;
    margin: 0;
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 16px;
    justify-content: center;
  }

  .file-details {
    text-align: left;
  }

  .file-name {
    font-size: 14px;
    font-weight: 500;
    color: #ffffff;
    margin: 0 0 4px 0;
  }

  .file-size {
    font-size: 12px;
    color: #a0a0a0;
    margin: 0;
  }

  .clear-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background-color: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    border-radius: 6px;
    color: #f87171;
    cursor: pointer;
    transition: all 0.2s;
  }

  .clear-button:hover:not(:disabled) {
    background-color: rgba(239, 68, 68, 0.2);
  }

  .clear-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .format-info {
    margin: 24px 0;
    padding: 16px;
    background-color: #1e1e1e;
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .format-title {
    font-size: 14px;
    font-weight: 600;
    color: #ffffff;
    margin: 0 0 12px 0;
  }

  .format-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 8px;
  }

  .format-item {
    display: flex;
    gap: 8px;
    font-size: 13px;
  }

  .format-item.optional {
    opacity: 0.7;
  }

  .format-label {
    color: #60a5fa;
    font-weight: 500;
  }

  .format-value {
    color: #d4d4d4;
  }

  .submit-button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    width: 100%;
    padding: 12px 24px;
    background-color: #007acc;
    border: none;
    border-radius: 6px;
    color: #ffffff;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .submit-button:hover:not(:disabled) {
    background-color: #005a9e;
  }

  .submit-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .loading-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: #ffffff;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>