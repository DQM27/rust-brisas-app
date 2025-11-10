<script lang="ts">
  import { useTabState } from '$lib/stores/tabs';
  import { Save, X, Upload, User } from 'lucide-svelte';
  
  export let tabId: string;
  export let data: any = {};

  const tabState = useTabState(tabId);

  // Datos del formulario
  let form = {
    // Información Personal
    cedula: data?.cedula || '',
    nombre: data?.nombre || '',
    primerApellido: data?.primerApellido || '',
    segundoApellido: data?.segundoApellido || '',
    fechaNacimiento: data?.fechaNacimiento || '',
    genero: data?.genero || '',
    
    // Contacto
    telefono: data?.telefono || '',
    celular: data?.celular || '',
    email: data?.email || '',
    direccion: data?.direccion || '',
    provincia: data?.provincia || '',
    canton: data?.canton || '',
    distrito: data?.distrito || '',
    
    // Información Laboral
    especialidad: data?.especialidad || '',
    experienciaAnios: data?.experienciaAnios || '',
    tipoContrato: data?.tipoContrato || 'temporal',
    disponibilidad: data?.disponibilidad || 'inmediata',
    
    // Documentos
    licenciaConducir: data?.licenciaConducir || false,
    vehiculoPropio: data?.vehiculoPropio || false,
    
    // Estado
    estado: data?.estado || 'activo'
  };

  let originalForm = JSON.stringify(form);
  let loading = false;
  let error = '';
  let success = '';

  // Detectar cambios
  $: isDirty = JSON.stringify(form) !== originalForm;
  $: if (isDirty) tabState.markDirty();

  const especialidades = [
    'Electricista',
    'Plomero',
    'Albañil',
    'Pintor',
    'Carpintero',
    'Soldador',
    'Mecánico',
    'Jardinero',
    'Limpieza',
    'Seguridad',
    'Otro'
  ];

  const provincias = [
    'San José',
    'Alajuela',
    'Cartago',
    'Heredia',
    'Guanacaste',
    'Puntarenas',
    'Limón'
  ];

  async function handleSave() {
    loading = true;
    error = '';
    success = '';

    try {
      // Validaciones
      if (!form.cedula || !form.nombre || !form.primerApellido) {
        throw new Error('Campos requeridos: Cédula, Nombre y Primer Apellido');
      }

      if (!form.telefono && !form.celular) {
        throw new Error('Debe proporcionar al menos un número de contacto');
      }

      // TODO: Llamada a API/Tauri para guardar
      // await tauri.saveContractor(form);

      // Simular guardado
      await new Promise(resolve => setTimeout(resolve, 1000));

      originalForm = JSON.stringify(form);
      tabState.markClean();
      tabState.updateData(form);
      
      // Actualizar título del tab con el nombre
      const nombreCompleto = `${form.nombre} ${form.primerApellido}`;
      tabState.updateTitle(`Contratista: ${nombreCompleto}`);
      
      success = 'Contratista guardado exitosamente';
      
    } catch (e: any) {
      error = e.message || 'Error al guardar el contratista';
    } finally {
      loading = false;
    }
  }

  function handleCancel() {
    if (isDirty) {
      const confirmed = confirm('Hay cambios sin guardar. ¿Deseas descartarlos?');
      if (!confirmed) return;
    }
    tabState.close();
  }

  function handleReset() {
    if (confirm('¿Deseas restablecer el formulario?')) {
      form = JSON.parse(originalForm);
    }
  }
</script>

<div class="contractor-register">
  <div class="header">
    <div class="header-content">
      <div class="header-icon">
        <User size={24} />
      </div>
      <div>
        <h2>Registro de Contratista</h2>
        <p class="subtitle">Complete la información del contratista</p>
      </div>
    </div>
    
    <div class="header-actions">
      <button class="btn btn-secondary btn-sm" on:click={handleReset} disabled={loading || !isDirty}>
        Restablecer
      </button>
      <button class="btn btn-secondary btn-sm" on:click={handleCancel} disabled={loading}>
        <X size={16} />
        Cancelar
      </button>
      <button class="btn btn-primary btn-sm" on:click={handleSave} disabled={loading}>
        <Save size={16} />
        {loading ? 'Guardando...' : 'Guardar'}
      </button>
    </div>
  </div>

  <div class="content">
    <form on:submit|preventDefault={handleSave}>
      <!-- Información Personal -->
      <section class="form-section">
        <h3>Información Personal</h3>
        
        <div class="form-row">
          <div class="form-group">
            <label>Cédula <span class="required">*</span></label>
            <input 
              type="text" 
              bind:value={form.cedula}
              placeholder="0-0000-0000"
              required
              disabled={loading}
            />
          </div>
          
          <div class="form-group">
            <label>Fecha de Nacimiento</label>
            <input 
              type="date" 
              bind:value={form.fechaNacimiento}
              disabled={loading}
            />
          </div>
        </div>

        <div class="form-group">
          <label>Nombre <span class="required">*</span></label>
          <input 
            type="text" 
            bind:value={form.nombre}
            placeholder="Nombre"
            required
            disabled={loading}
          />
        </div>

        <div class="form-row">
          <div class="form-group">
            <label>Primer Apellido <span class="required">*</span></label>
            <input 
              type="text" 
              bind:value={form.primerApellido}
              placeholder="Primer Apellido"
              required
              disabled={loading}
            />
          </div>
          
          <div class="form-group">
            <label>Segundo Apellido</label>
            <input 
              type="text" 
              bind:value={form.segundoApellido}
              placeholder="Segundo Apellido"
              disabled={loading}
            />
          </div>
        </div>

        <div class="form-group">
          <label>Género</label>
          <select bind:value={form.genero} disabled={loading}>
            <option value="">Seleccionar...</option>
            <option value="masculino">Masculino</option>
            <option value="femenino">Femenino</option>
            <option value="otro">Otro</option>
          </select>
        </div>
      </section>

      <!-- Información de Contacto -->
      <section class="form-section">
        <h3>Información de Contacto</h3>
        
        <div class="form-row">
          <div class="form-group">
            <label>Teléfono</label>
            <input 
              type="tel" 
              bind:value={form.telefono}
              placeholder="2222-2222"
              disabled={loading}
            />
          </div>
          
          <div class="form-group">
            <label>Celular <span class="required">*</span></label>
            <input 
              type="tel" 
              bind:value={form.celular}
              placeholder="8888-8888"
              disabled={loading}
            />
          </div>
        </div>

        <div class="form-group">
          <label>Email</label>
          <input 
            type="email" 
            bind:value={form.email}
            placeholder="correo@ejemplo.com"
            disabled={loading}
          />
        </div>

        <div class="form-group">
          <label>Dirección</label>
          <textarea 
            bind:value={form.direccion}
            placeholder="Dirección exacta"
            rows="2"
            disabled={loading}
          ></textarea>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label>Provincia</label>
            <select bind:value={form.provincia} disabled={loading}>
              <option value="">Seleccionar...</option>
              {#each provincias as provincia}
                <option value={provincia}>{provincia}</option>
              {/each}
            </select>
          </div>
          
          <div class="form-group">
            <label>Cantón</label>
            <input 
              type="text" 
              bind:value={form.canton}
              placeholder="Cantón"
              disabled={loading}
            />
          </div>
        </div>

        <div class="form-group">
          <label>Distrito</label>
          <input 
            type="text" 
            bind:value={form.distrito}
            placeholder="Distrito"
            disabled={loading}
          />
        </div>
      </section>

      <!-- Información Laboral -->
      <section class="form-section">
        <h3>Información Laboral</h3>
        
        <div class="form-row">
          <div class="form-group">
            <label>Especialidad <span class="required">*</span></label>
            <select bind:value={form.especialidad} required disabled={loading}>
              <option value="">Seleccionar...</option>
              {#each especialidades as esp}
                <option value={esp}>{esp}</option>
              {/each}
            </select>
          </div>
          
          <div class="form-group">
            <label>Años de Experiencia</label>
            <input 
              type="number" 
              bind:value={form.experienciaAnios}
              placeholder="0"
              min="0"
              disabled={loading}
            />
          </div>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label>Tipo de Contrato</label>
            <select bind:value={form.tipoContrato} disabled={loading}>
              <option value="temporal">Temporal</option>
              <option value="permanente">Permanente</option>
              <option value="proyecto">Por Proyecto</option>
            </select>
          </div>
          
          <div class="form-group">
            <label>Disponibilidad</label>
            <select bind:value={form.disponibilidad} disabled={loading}>
              <option value="inmediata">Inmediata</option>
              <option value="1-semana">1 Semana</option>
              <option value="2-semanas">2 Semanas</option>
              <option value="1-mes">1 Mes</option>
            </select>
          </div>
        </div>

        <div class="checkbox-group">
          <label class="checkbox-label">
            <input 
              type="checkbox" 
              bind:checked={form.licenciaConducir}
              disabled={loading}
            />
            <span>Licencia de conducir</span>
          </label>
          
          <label class="checkbox-label">
            <input 
              type="checkbox" 
              bind:checked={form.vehiculoPropio}
              disabled={loading}
            />
            <span>Vehículo propio</span>
          </label>
        </div>
      </section>

      <!-- Estado -->
      <section class="form-section">
        <h3>Estado</h3>
        
        <div class="form-group">
          <label>Estado del Contratista</label>
          <select bind:value={form.estado} disabled={loading}>
            <option value="activo">Activo</option>
            <option value="inactivo">Inactivo</option>
            <option value="suspendido">Suspendido</option>
          </select>
        </div>
      </section>

      {#if error}
        <div class="alert alert-error">{error}</div>
      {/if}

      {#if success}
        <div class="alert alert-success">{success}</div>
      {/if}
    </form>
  </div>
</div>

<style>
  .contractor-register {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg-primary);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    flex-shrink: 0;
  }

  .header-content {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .header-icon {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background: linear-gradient(135deg, var(--accent-primary) 0%, var(--accent-active) 100%);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .subtitle {
    margin: 0;
    font-size: 0.875rem;
    color: var(--text-muted);
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }

  form {
    max-width: 900px;
    margin: 0 auto;
  }

  .form-section {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .form-section h3 {
    margin: 0 0 1.25rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--border-color);
  }

  .required {
    color: var(--error);
  }

  textarea {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 0.9375rem;
    background: var(--bg-primary);
    color: var(--text-primary);
    transition: all 0.15s ease;
    font-family: inherit;
    resize: vertical;
    min-height: 60px;
  }

  textarea:focus {
    outline: none;
    border-color: var(--border-focus);
    box-shadow: 0 0 0 1px var(--border-focus);
  }

  .checkbox-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 0.9375rem;
    color: var(--text-secondary);
  }

  .checkbox-label input[type="checkbox"] {
    width: auto;
    cursor: pointer;
  }

  .checkbox-label:hover {
    color: var(--text-primary);
  }
</style>