<script lang="ts">
  // ==========================================
  // ProveedorFormFields.svelte
  // ==========================================
  // Campos específicos para registro de PROVEEDORES

  import { onMount } from "svelte";
  import { fetchEmpresasActivas } from "$lib/api/empresa";
  import type { ProveedorFormData } from "$lib/logic/ingreso/proveedorService";
  import type { EmpresaResponse } from "$lib/types/empresa";

  export let formData: ProveedorFormData;
  export let errors: Record<string, string> = {};
  export let onChange: (field: keyof ProveedorFormData, value: any) => void;

  let empresas: EmpresaResponse[] = [];
  let loadingEmpresas = false;

  onMount(async () => {
    loadingEmpresas = true;
    try {
      empresas = await fetchEmpresasActivas();
    } catch (error) {
      console.error("Error cargando empresas:", error);
    }
    loadingEmpresas = false;
  });

  function handleInput(field: keyof ProveedorFormData) {
    return (e: Event) => {
      const target = e.target as HTMLInputElement | HTMLSelectElement;
      onChange(field, target.value);
    };
  }
</script>

<div class="proveedor-form-fields">
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
          placeholder="Nombre del proveedor"
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
          placeholder="Apellido del proveedor"
        />
        {#if errors.apellido}
          <span class="error-message">{errors.apellido}</span>
        {/if}
      </div>
    </div>
  </div>

  <!-- Información de empresa -->
  <div class="section">
    <h3 class="section-title">Información de Proveedor</h3>

    <div class="form-row">
      <div class="form-group">
        <label for="empresaId">Empresa Proveedora *</label>
        {#if loadingEmpresas}
          <select disabled>
            <option>Cargando empresas...</option>
          </select>
        {:else}
          <select
            id="empresaId"
            value={formData.empresaId}
            on:change={handleInput("empresaId")}
            class:error={errors.empresaId}
          >
            <option value="">Seleccione una empresa</option>
            {#each empresas as emp}
              <option value={emp.id}>{emp.nombre}</option>
            {/each}
          </select>
        {/if}
        {#if errors.empresaId}
          <span class="error-message">{errors.empresaId}</span>
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
          placeholder="Ej: Almacén, Producción"
        />
        {#if errors.areaVisitada}
          <span class="error-message">{errors.areaVisitada}</span>
        {/if}
      </div>
    </div>

    <div class="form-group">
      <label for="motivo">Motivo de Ingreso *</label>
      <textarea
        id="motivo"
        value={formData.motivo}
        on:input={handleInput("motivo")}
        class:error={errors.motivo}
        placeholder="Describa el motivo del ingreso (Ej: Entrega de materiales, Mantenimiento, etc.)"
        rows="3"
      ></textarea>
      {#if errors.motivo}
        <span class="error-message">{errors.motivo}</span>
      {/if}
    </div>
  </div>

  <!-- Observaciones -->
  <div class="form-group">
    <label for="observaciones">Observaciones</label>
    <textarea
      id="observaciones"
      value={formData.observaciones || ""}
      on:input={handleInput("observaciones")}
      placeholder="Observaciones adicionales (opcional)"
      rows="2"
    ></textarea>
  </div>
</div>

<style>
  .proveedor-form-fields {
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
  select,
  textarea {
    padding: 0.625rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 0.875rem;
    background: var(--bg-primary);
    color: var(--text-primary);
    transition: all 0.2s ease;
  }

  input:focus,
  select:focus,
  textarea:focus {
    outline: none;
    border-color: var(--primary);
    box-shadow: 0 0 0 3px var(--primary-alpha-10);
  }

  input.error,
  select.error,
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

  select {
    cursor: pointer;
  }

  select:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
