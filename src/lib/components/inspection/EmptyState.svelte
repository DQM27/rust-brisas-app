<!-- src/lib/components/inspection/EmptyState.svelte -->
<script lang="ts">
  import { Eye } from 'lucide-svelte';
</script>

<div class="empty-state">
  <div class="empty-icon">
    <Eye size={48} />
  </div>
  <h3>Panel de Inspección</h3>
  <p>Selecciona un elemento de la lista para ver sus detalles aquí.</p>
  <div class="tips">
    <p class="tip">💡 Haz clic en cualquier registro para inspeccionar</p>
  </div>
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 2rem;
    text-align: center;
    color: #888;
  }

  .empty-icon {
    margin-bottom: 1rem;
    opacity: 0.3;
  }

  h3 {
    margin: 0 0 0.5rem 0;
    font-size: 16px;
    font-weight: 600;
    color: #aaa;
  }

  p {
    margin: 0;
    font-size: 13px;
    color: #777;
    max-width: 300px;
  }

  .tips {
    margin-top: 2rem;
    padding: 1rem;
    background: rgba(0, 122, 204, 0.1);
    border-radius: 6px;
    border-left: 3px solid #007acc;
  }

  .tip {
    font-size: 12px;
    color: #007acc;
    margin: 0;
  }
</style>