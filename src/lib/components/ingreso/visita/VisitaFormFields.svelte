<script lang="ts">
  // ==========================================
  // VisitaFormFields.svelte
  // ==========================================
  // Campos específicos para registro de VISITAS

  import type { VisitaFormData } from "$lib/logic/ingreso/visitaService";

  export let formData: VisitaFormData;
  export let errors: Record<string, string> = {};
  export let onChange: (field: keyof VisitaFormData, value: any) => void;

  function handleInput(field: keyof VisitaFormData) {
    return (e: Event) => {
      const target = e.target as HTMLInputElement;
      onChange(field, target.value);
    };
  }
</script>

<div class="visita-form-fields">
  <!-- Información personal -->
  <div class="section">
    <h3 class="section-title">Información Personal</h3>

    <div class="form-row">
      <div class="form-group">
        <label for="cedula">Cédula *</label>
        <input
          id="cedula"
          type="text"
          value={formData.cedula}
          on:input={handleInput("cedula")}
          class:error={errors.cedula}
          placeholder="Ej: 1-1234-5678"
        />
        {#if errors.cedula}
          <span class="error-message">{errors.cedula}</span>
        {/if}
      </div>

      <div class="form-group">
        <label for="nombre">Nombre *</label>
        <input
          id="nombre"
          type="text"
          value={formData.nombre}
          on:input={handleInput("nombre")}
          class:error={errors.nombre}
          placeholder="Nombre del visitante"
        />
        {#if errors.nombre}
          <span class="error-message">{errors.nombre}</span>
        {/if}
      </div>

      <div class="form-group">
        <label for="apellido">Apellido *</label>
        <input
          id="apellido"
          type="text"
          value={formData.apellido}
          on:input={handleInput("apellido")}
          class:error={errors.apellido}
          placeholder="Apellido del visitante"
        />
        {#if errors.apellido}
          <span class="error-message">{errors.apellido}</span>
        {/if}
      </div>
    </div>
  </div>

  <!-- Información de visita -->
  <div class="section">
    <h3 class="section-title">Información de Visita</h3>

    <div class="form-row">
      <div class="form-group">
        <label for="anfitrion">Anfitrión *</label>
        <input
          id="anfitrion"
          type="text"
          value={formData.anfitrion}
          on:input={handleInput("anfitrion")}
          class:error={errors.anfitrion}
          placeholder="Nombre del anfitrión"
        />
        {#if errors.anfitrion}
          <span class="error-message">{errors.anfitrion}</span>
        {/if}
      </div>

      <div class="form-group">
        <label for="areaVisitada">Área a Visitar *</label>
        <input
          id="areaVisitada"
          type="text"
          value={formData.areaVisitada}
          on:input={handleInput("areaVisitada")}
          class:error={errors.areaVisitada}
          placeholder="Ej: Administración, Producción"
        />
        {#if errors.areaVisitada}
          <span class="error-message">{errors.areaVisitada}</span>
        {/if}
      </div>
    </div>

    <div class="form-group">
      <label for="motivoVisita">Motivo de Visita *</label>
      <textarea
        id="motivoVisita"
        value={formData.motivoVisita}
        on:input={handleInput("motivoVisita")}
        class:error={errors.motivoVisita}
        placeholder="Describa el motivo de la visita"
        rows="3"
      ></textarea>
      {#if errors.motivoVisita}
        <span class="error-message">{errors.motivoVisita}</span>
      {/if}
    </div>
  </div>
</div>

<style>
  .visita-form-fields {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .section-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-secondary);
    margin: 0;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border);
  }

  .form-row {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-secondary);
  }

  input,
  textarea {
    padding: 0.625rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 0.875rem;
    /* High contrast override */
    background-color: rgb(255 255 255);
    color: rgb(17 24 39);
  }

  :global(.dark) input,
  :global(.dark) textarea {
    background-color: #252526;
    color: #f3f4f6;
    border-color: rgba(255, 255, 255, 0.2);
  }

  input:focus,
  textarea:focus {
    outline: none;
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-alpha-10);
  }

  input.error,
  textarea.error {
    border-color: var(--error);
  }

  .error-message {
    font-size: 0.75rem;
    color: var(--error);
  }

  textarea {
    resize: vertical;
    min-height: 60px;
  }
</style>
