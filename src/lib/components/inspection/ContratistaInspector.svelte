<!-- src/lib/components/inspection/inspectors/ContratistaInspector.svelte -->
<script lang="ts">
  import { User, Building2, Calendar, Shield, AlertCircle } from 'lucide-svelte';
  
  // Props
  export let data: any;

  // Computed
  $: praindVencido = data?.fecha_vencimiento_praind 
    ? new Date(data.fecha_vencimiento_praind) < new Date()
    : false;

  $: diasRestantes = data?.fecha_vencimiento_praind
    ? Math.floor((new Date(data.fecha_vencimiento_praind).getTime() - new Date().getTime()) / (1000 * 60 * 60 * 24))
    : 0;
</script>

<div class="inspector">
  <!-- Header con nombre -->
  <div class="inspector-header">
    <div class="avatar">
      <User size={32} />
    </div>
    <div class="header-info">
      <h2>{data?.nombre || 'Sin nombre'} {data?.apellido || ''}</h2>
      <p class="subtitle">Contratista</p>
    </div>
  </div>

  <!-- Secciones de información -->
  <div class="inspector-content">
    <!-- Información básica -->
    <section class="info-section">
      <h3>Información General</h3>
      <div class="info-grid">
        <div class="info-item">
          <span class="label">Cédula:</span>
          <span class="value">{data?.cedula || 'N/A'}</span>
        </div>
        <div class="info-item">
          <span class="label">Estado:</span>
          <span class="value status-badge {data?.estado || 'inactivo'}">
            {data?.estado || 'desconocido'}
          </span>
        </div>
      </div>
    </section>

    <!-- Empresa -->
    <section class="info-section">
      <h3><Building2 size={16} /> Empresa</h3>
      <div class="info-item">
        <span class="value">{data?.empresa_nombre || 'Sin empresa asignada'}</span>
      </div>
    </section>

    <!-- PRAIND -->
    <section class="info-section">
      <h3><Shield size={16} /> PRAIND</h3>
      <div class="info-item">
        <span class="label">Vencimiento:</span>
        <span class="value">{data?.fecha_vencimiento_praind || 'N/A'}</span>
      </div>
      
      {#if praindVencido}
        <div class="alert alert-danger">
          <AlertCircle size={16} />
          <span>PRAIND vencido</span>
        </div>
      {:else if diasRestantes <= 30 && diasRestantes > 0}
        <div class="alert alert-warning">
          <AlertCircle size={16} />
          <span>Vence en {diasRestantes} días</span>
        </div>
      {:else if diasRestantes > 0}
        <div class="alert alert-success">
          <Calendar size={16} />
          <span>Vigente ({diasRestantes} días restantes)</span>
        </div>
      {/if}
    </section>

    <!-- Placeholder para futuras secciones -->
    <section class="info-section placeholder">
      <h3>Historial de Ingresos</h3>
      <p class="coming-soon">Próximamente en Fase 2</p>
    </section>

    <section class="info-section placeholder">
      <h3>Vehículos Registrados</h3>
      <p class="coming-soon">Próximamente en Fase 2</p>
    </section>
  </div>
</div>

<style>
  .inspector {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow-y: auto;
  }

  .inspector-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1.5rem;
    background: #2d2d2d;
    border-bottom: 1px solid #3c3c3c;
  }

  .avatar {
    width: 56px;
    height: 56px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
  }

  .header-info h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: #fff;
  }

  .subtitle {
    margin: 0.25rem 0 0 0;
    font-size: 12px;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .inspector-content {
    flex: 1;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .info-section {
    background: #2d2d2d;
    border-radius: 6px;
    padding: 1rem;
    border: 1px solid #3c3c3c;
  }

  .info-section h3 {
    margin: 0 0 0.75rem 0;
    font-size: 13px;
    font-weight: 600;
    color: #aaa;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .info-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .info-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .label {
    font-size: 11px;
    color: #888;
    text-transform: uppercase;
    letter-spacing: 0.3px;
  }

  .value {
    font-size: 13px;
    color: #ccc;
    font-weight: 500;
  }

  .status-badge {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status-badge.activo {
    background: rgba(76, 175, 80, 0.2);
    color: #4caf50;
  }

  .status-badge.inactivo {
    background: rgba(244, 67, 54, 0.2);
    color: #f44336;
  }

  .alert {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 12px;
    margin-top: 0.5rem;
  }

  .alert-success {
    background: rgba(76, 175, 80, 0.1);
    color: #4caf50;
    border: 1px solid rgba(76, 175, 80, 0.3);
  }

  .alert-warning {
    background: rgba(255, 152, 0, 0.1);
    color: #ff9800;
    border: 1px solid rgba(255, 152, 0, 0.3);
  }

  .alert-danger {
    background: rgba(244, 67, 54, 0.1);
    color: #f44336;
    border: 1px solid rgba(244, 67, 54, 0.3);
  }

  .placeholder {
    opacity: 0.6;
  }

  .coming-soon {
    margin: 0;
    font-size: 12px;
    color: #666;
    font-style: italic;
  }
</style>