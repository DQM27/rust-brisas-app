<script lang="ts">
  import { CheckCircle, AlertTriangle, XCircle, FileText } from 'lucide-svelte';
  import type { ImportResultResponse } from '$lib/types/blacklistImport.types';

  interface Props {
    result: ImportResultResponse;
    onContinue: () => void;
    onCancel: () => void;
  }

  let { result, onContinue, onCancel }: Props = $props();

  const hasErrors = $derived(result.failed > 0);
  const hasReviews = $derived(result.needsReview > 0);
  const canContinue = $derived(result.successful > 0 || result.needsReview > 0);
</script>

<div class="summary-container">
  <div class="summary-header">
    <FileText size={32} class="text-blue-400" />
    <h2 class="summary-title">Resultado del Análisis</h2>
  </div>

  <!-- Stats Cards -->
  <div class="stats-grid">
    <!-- Total -->
    <div class="stat-card">
      <div class="stat-icon total">
        <FileText size={24} />
      </div>
      <div class="stat-content">
        <p class="stat-label">Total de Filas</p>
        <p class="stat-value">{result.totalRows}</p>
      </div>
    </div>

    <!-- Válidas -->
    <div class="stat-card">
      <div class="stat-icon success">
        <CheckCircle size={24} />
      </div>
      <div class="stat-content">
        <p class="stat-label">Válidas</p>
        <p class="stat-value">{result.successful}</p>
      </div>
    </div>

    <!-- Requieren Revisión -->
    {#if hasReviews}
      <div class="stat-card">
        <div class="stat-icon warning">
          <AlertTriangle size={24} />
        </div>
        <div class="stat-content">
          <p class="stat-label">Requieren Revisión</p>
          <p class="stat-value">{result.needsReview}</p>
        </div>
      </div>
    {/if}

    <!-- Con Errores -->
    {#if hasErrors}
      <div class="stat-card">
        <div class="stat-icon error">
          <XCircle size={24} />
        </div>
        <div class="stat-content">
          <p class="stat-label">Con Errores</p>
          <p class="stat-value">{result.failed}</p>
        </div>
      </div>
    {/if}
  </div>

  <!-- Mensajes informativos -->
  <div class="info-messages">
    {#if result.successful > 0}
      <div class="info-message success">
        <CheckCircle size={16} />
        <p>
          {result.successful} {result.successful === 1 ? 'fila' : 'filas'} se procesaron correctamente
          y están listas para importar.
        </p>
      </div>
    {/if}

    {#if hasReviews}
      <div class="info-message warning">
        <AlertTriangle size={16} />
        <p>
          {result.needsReview} {result.needsReview === 1 ? 'fila requiere' : 'filas requieren'} revisión
          manual debido a nombres compuestos o múltiples palabras.
        </p>
      </div>
    {/if}

    {#if hasErrors}
      <div class="info-message error">
        <XCircle size={16} />
        <p>
          {result.failed} {result.failed === 1 ? 'fila tiene errores' : 'filas tienen errores'} y no se
          {result.failed === 1 ? 'importará' : 'importarán'}.
        </p>
      </div>
    {/if}
  </div>

  <!-- Errores detallados -->
  {#if hasErrors && result.errors.length > 0}
    <div class="error-list">
      <h3 class="error-list-title">Errores Encontrados:</h3>
      <div class="error-items">
        {#each result.errors.slice(0, 5) as error}
          <div class="error-item">
            <span class="error-row">Fila {error.rowNumber}</span>
            {#if error.cedula}
              <span class="error-cedula">{error.cedula}</span>
            {/if}
            <span class="error-message">{error.message}</span>
          </div>
        {/each}
        {#if result.errors.length > 5}
          <p class="error-more">
            ... y {result.errors.length - 5} error(es) más
          </p>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Acciones -->
  <div class="action-buttons">
    <button onclick={onCancel} class="cancel-button">
      Cancelar
    </button>
    <button
      onclick={onContinue}
      disabled={!canContinue}
      class="continue-button"
    >
      {hasReviews ? 'Revisar y Continuar' : 'Continuar con Importación'}
    </button>
  </div>
</div>

<style>
  .summary-container {
    width: 100%;
    max-width: 900px;
    margin: 0 auto;
    padding: 24px;
    background-color: #252526;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .summary-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .summary-title {
    font-size: 20px;
    font-weight: 600;
    color: #ffffff;
    margin: 0;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 16px;
    margin-bottom: 24px;
  }

  .stat-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px;
    background-color: #1e1e1e;
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .stat-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border-radius: 8px;
  }

  .stat-icon.total {
    background-color: rgba(59, 130, 246, 0.1);
    color: #60a5fa;
  }

  .stat-icon.success {
    background-color: rgba(34, 197, 94, 0.1);
    color: #4ade80;
  }

  .stat-icon.warning {
    background-color: rgba(234, 179, 8, 0.1);
    color: #fde047;
  }

  .stat-icon.error {
    background-color: rgba(239, 68, 68, 0.1);
    color: #f87171;
  }

  .stat-content {
    flex: 1;
  }

  .stat-label {
    font-size: 12px;
    color: #a0a0a0;
    margin: 0 0 4px 0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .stat-value {
    font-size: 24px;
    font-weight: 600;
    color: #ffffff;
    margin: 0;
  }

  .info-messages {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-bottom: 24px;
  }

  .info-message {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 12px;
    border-radius: 6px;
    font-size: 13px;
  }

  .info-message p {
    margin: 0;
    line-height: 1.5;
  }

  .info-message.success {
    background-color: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.2);
    color: #4ade80;
  }

  .info-message.warning {
    background-color: rgba(234, 179, 8, 0.1);
    border: 1px solid rgba(234, 179, 8, 0.2);
    color: #fde047;
  }

  .info-message.error {
    background-color: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    color: #f87171;
  }

  .error-list {
    margin-bottom: 24px;
    padding: 16px;
    background-color: #1e1e1e;
    border-radius: 6px;
    border: 1px solid rgba(239, 68, 68, 0.2);
  }

  .error-list-title {
    font-size: 14px;
    font-weight: 600;
    color: #f87171;
    margin: 0 0 12px 0;
  }

  .error-items {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .error-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px;
    background-color: rgba(239, 68, 68, 0.05);
    border-radius: 4px;
    font-size: 12px;
  }

  .error-row {
    color: #f87171;
    font-weight: 600;
    min-width: 60px;
  }

  .error-cedula {
    color: #a0a0a0;
    font-family: monospace;
    min-width: 100px;
  }

  .error-message {
    color: #d4d4d4;
    flex: 1;
  }

  .error-more {
    margin: 8px 0 0 0;
    font-size: 12px;
    color: #a0a0a0;
    font-style: italic;
  }

  .action-buttons {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
  }

  .cancel-button,
  .continue-button {
    padding: 10px 24px;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .cancel-button {
    background-color: transparent;
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: #ffffff;
  }

  .cancel-button:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }

  .continue-button {
    background-color: #007acc;
    border: none;
    color: #ffffff;
  }

  .continue-button:hover:not(:disabled) {
    background-color: #005a9e;
  }

  .continue-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>