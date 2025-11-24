<script lang="ts">
  import { X, AlertTriangle, Save } from 'lucide-svelte';
  import type { BlacklistImportEntry } from '$lib/types/blacklistImport.types';

  interface Props {
    entries: BlacklistImportEntry[];
    onSave: (correctedEntries: BlacklistImportEntry[]) => void;
    onClose: () => void;
  }

  let { entries, onSave, onClose }: Props = $props();

  // Estado local de las entradas siendo editadas
  let editedEntries = $state<BlacklistImportEntry[]>([]);
  let currentIndex = $state(0);

  // Inicializar con copia de las entradas
  $effect(() => {
    editedEntries = structuredClone(entries);
  });

  const currentEntry = $derived(editedEntries[currentIndex]);
  const hasNext = $derived(currentIndex < editedEntries.length - 1);
  const hasPrev = $derived(currentIndex > 0);
  const progress = $derived(`${currentIndex + 1} / ${editedEntries.length}`);

  function handleNext() {
    if (hasNext) {
      currentIndex++;
    }
  }

  function handlePrev() {
    if (hasPrev) {
      currentIndex--;
    }
  }

  function handleSave() {
    onSave(editedEntries);
  }

  function updateCurrentEntry(
    field: keyof BlacklistImportEntry,
    value: any
  ) {
    if (currentEntry) {
      editedEntries[currentIndex] = {
        ...currentEntry,
        [field]: value
      };
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-overlay" onclick={onClose}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-content" onclick={(e) => e.stopPropagation()}>
    <!-- Header -->
    <div class="modal-header">
      <div class="header-left">
        <AlertTriangle size={24} class="text-yellow-400" />
        <div>
          <h2 class="modal-title">Validación Manual Requerida</h2>
          <p class="modal-subtitle">
            Los siguientes nombres requieren corrección manual
          </p>
        </div>
      </div>
      <button onclick={onClose} class="close-button" title="Cerrar">
        <X size={20} />
      </button>
    </div>

    <!-- Progress -->
    <div class="progress-bar">
      <div class="progress-fill" style="width: {((currentIndex + 1) / editedEntries.length) * 100}%"></div>
    </div>
    <p class="progress-text">{progress}</p>

    {#if currentEntry}
      <!-- Current Entry Form -->
      <div class="form-container">
        <!-- Nombre Original -->
        <div class="original-name">
          <p class="original-label">Nombre original detectado:</p>
          <p class="original-value">{currentEntry.primerNombre} {currentEntry.primerApellido}</p>
          {#if currentEntry.validationMessage}
            <p class="validation-message">
              <AlertTriangle size={14} />
              {currentEntry.validationMessage}
            </p>
          {/if}
        </div>

        <!-- Datos básicos (solo lectura) -->
        <div class="readonly-section">
          <div class="readonly-field">
            <label>Cédula</label>
            <input
              type="text"
              value={currentEntry.cedula}
              readonly
              class="readonly-input"
            />
          </div>
          <div class="readonly-field">
            <label>Empresa</label>
            <input
              type="text"
              value={currentEntry.empresa}
              readonly
              class="readonly-input"
            />
          </div>
        </div>

        <!-- Nombres (editables) -->
        <div class="editable-section">
          <h3 class="section-title">Corrija los nombres:</h3>
          
          <div class="form-grid">
            <!-- Primer Nombre -->
            <div class="form-field">
              <label for="primerNombre" class="field-label">
                Primer Nombre <span class="required">*</span>
              </label>
              <input
                id="primerNombre"
                type="text"
                value={currentEntry.primerNombre}
                oninput={(e) => updateCurrentEntry('primerNombre', e.currentTarget.value)}
                placeholder="Juan"
                class="field-input"
              />
            </div>

            <!-- Segundo Nombre -->
            <div class="form-field">
              <label for="segundoNombre" class="field-label">
                Segundo Nombre
              </label>
              <input
                id="segundoNombre"
                type="text"
                value={currentEntry.segundoNombre || ''}
                oninput={(e) => updateCurrentEntry('segundoNombre', e.currentTarget.value || undefined)}
                placeholder="Carlos"
                class="field-input"
              />
            </div>

            <!-- Primer Apellido -->
            <div class="form-field">
              <label for="primerApellido" class="field-label">
                Primer Apellido <span class="required">*</span>
              </label>
              <input
                id="primerApellido"
                type="text"
                value={currentEntry.primerApellido}
                oninput={(e) => updateCurrentEntry('primerApellido', e.currentTarget.value)}
                placeholder="Pérez"
                class="field-input"
              />
            </div>

            <!-- Segundo Apellido -->
            <div class="form-field">
              <label for="segundoApellido" class="field-label">
                Segundo Apellido
              </label>
              <input
                id="segundoApellido"
                type="text"
                value={currentEntry.segundoApellido || ''}
                oninput={(e) => updateCurrentEntry('segundoApellido', e.currentTarget.value || undefined)}
                placeholder="Gómez"
                class="field-input"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Navigation & Actions -->
      <div class="modal-footer">
        <div class="nav-buttons">
          <button
            onclick={handlePrev}
            disabled={!hasPrev}
            class="nav-button"
          >
            ← Anterior
          </button>
          <button
            onclick={handleNext}
            disabled={!hasNext}
            class="nav-button"
          >
            Siguiente →
          </button>
        </div>

        <button onclick={handleSave} class="save-button">
          <Save size={18} />
          Guardar Correcciones
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s;
  }

  .modal-content {
    background-color: #252526;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    width: 90%;
    max-width: 700px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.7);
    animation: slideUp 0.3s;
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    padding: 20px 24px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .header-left {
    display: flex;
    gap: 12px;
  }

  .modal-title {
    margin: 0 0 4px 0;
    font-size: 18px;
    font-weight: 600;
    color: #ffffff;
  }

  .modal-subtitle {
    margin: 0;
    font-size: 13px;
    color: #a0a0a0;
  }

  .close-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background-color: transparent;
    border: none;
    border-radius: 4px;
    color: #a0a0a0;
    cursor: pointer;
    transition: all 0.2s;
  }

  .close-button:hover {
    background-color: rgba(255, 255, 255, 0.1);
    color: #ffffff;
  }

  .progress-bar {
    height: 3px;
    background-color: rgba(255, 255, 255, 0.1);
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background-color: #007acc;
    transition: width 0.3s ease;
  }

  .progress-text {
    padding: 8px 24px;
    margin: 0;
    font-size: 12px;
    color: #a0a0a0;
    text-align: center;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .form-container {
    padding: 20px 24px;
    overflow-y: auto;
    flex: 1;
  }

  .original-name {
    padding: 12px;
    background-color: rgba(234, 179, 8, 0.1);
    border: 1px solid rgba(234, 179, 8, 0.2);
    border-radius: 6px;
    margin-bottom: 20px;
  }

  .original-label {
    margin: 0 0 4px 0;
    font-size: 11px;
    color: #fde047;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .original-value {
    margin: 0 0 8px 0;
    font-size: 16px;
    font-weight: 600;
    color: #ffffff;
  }

  .validation-message {
    display: flex;
    align-items: center;
    gap: 6px;
    margin: 0;
    font-size: 12px;
    color: #fde047;
  }

  .readonly-section {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
    margin-bottom: 20px;
  }

  .readonly-field label {
    display: block;
    margin-bottom: 4px;
    font-size: 12px;
    color: #a0a0a0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .readonly-input {
    width: 100%;
    padding: 8px 12px;
    background-color: #1e1e1e;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    color: #a0a0a0;
    font-size: 13px;
  }

  .editable-section {
    padding: 16px;
    background-color: #1e1e1e;
    border-radius: 6px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .section-title {
    margin: 0 0 16px 0;
    font-size: 14px;
    font-weight: 600;
    color: #ffffff;
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
  }

  .form-field {
    display: flex;
    flex-direction: column;
  }

  .field-label {
    margin-bottom: 6px;
    font-size: 12px;
    font-weight: 500;
    color: #d4d4d4;
  }

  .required {
    color: #f87171;
  }

  .field-input {
    padding: 8px 12px;
    background-color: #252526;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    color: #ffffff;
    font-size: 13px;
    transition: all 0.2s;
  }

  .field-input:focus {
    outline: none;
    border-color: #007acc;
    box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.2);
  }

  .modal-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 24px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    background-color: #1e1e1e;
  }

  .nav-buttons {
    display: flex;
    gap: 8px;
  }

  .nav-button {
    padding: 8px 16px;
    background-color: transparent;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    color: #ffffff;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .nav-button:hover:not(:disabled) {
    background-color: rgba(255, 255, 255, 0.05);
  }

  .nav-button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .save-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 24px;
    background-color: #007acc;
    border: none;
    border-radius: 6px;
    color: #ffffff;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .save-button:hover {
    background-color: #005a9e;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }
</style>