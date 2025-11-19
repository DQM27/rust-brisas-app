<script lang="ts">
  import { Toaster } from 'svelte-5-french-toast';
</script>

<div class="toast-container">
  <Toaster 
    position="bottom-right"
    toastOptions={{
      duration: 4000,
      style: 'background: #2a2f32; color: #e9edef; border-radius: 12px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4); padding: 12px 16px; font-size: 14px; min-width: 280px; max-width: 400px;',
      className: 'whatsapp-toast'
    }}
    gutter={8}
  />
</div>

<style>
  .toast-container {
    /* Esto da scope al componente */
    display: contents;
  }

  /* Animación de entrada estilo WhatsApp */
  :global(.whatsapp-toast) {
    animation: slideInWhatsApp 0.25s cubic-bezier(0.25, 0.46, 0.45, 0.94);
    position: relative;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  @keyframes slideInWhatsApp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  /* Animación de salida */
  :global([data-exit]) {
    animation: slideOutWhatsApp 0.2s ease-out forwards;
  }

  @keyframes slideOutWhatsApp {
    from {
      transform: translateY(0);
      opacity: 1;
    }
    to {
      transform: translateY(20px);
      opacity: 0;
    }
  }

  /* Estilos por tipo con iconos */
  :global([data-type="success"]) {
    background: #2a2f32 !important;
  }

  :global([data-type="error"]) {
    background: #2a2f32 !important;
  }

  :global([data-type="loading"]) {
    background: #2a2f32 !important;
  }

  /* Agregar checkmark verde para success */
  :global([data-type="success"]::before) {
    content: '✓';
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    background: #25d366;
    color: white;
    border-radius: 50%;
    font-size: 12px;
    font-weight: bold;
    flex-shrink: 0;
  }

  /* Agregar X roja para error */
  :global([data-type="error"]::before) {
    content: '✕';
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    background: #dc4a3e;
    color: white;
    border-radius: 50%;
    font-size: 12px;
    font-weight: bold;
    flex-shrink: 0;
  }

  /* Agregar spinner para loading */
  :global([data-type="loading"]::before) {
    content: '';
    display: inline-block;
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: #25d366;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Hover sutil */
  :global(.whatsapp-toast:hover) {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    transition: box-shadow 0.2s ease;
  }

  /* Responsive */
  @media (max-width: 640px) {
    :global(.whatsapp-toast) {
      min-width: 0;
      max-width: calc(100vw - 32px);
      margin: 0 16px;
    }
  }
</style>