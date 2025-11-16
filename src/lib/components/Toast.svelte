<script lang="ts">
  import { Toaster } from 'svelte-5-french-toast';
</script>

<Toaster 
  position="bottom-right"
  toastOptions={{
    duration: 4000,
    style: 'background: #252526; color: #e5e5e5; border: 1px solid #3c3c3c; border-radius: 8px; box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5); padding: 16px; font-size: 14px;',
    className: 'custom-toast'
  }}
  gutter={12}
/>

<style>
  /* Animaciones de entrada */
  :global(.custom-toast) {
    animation: slideIn 0.3s cubic-bezier(0.21, 1.02, 0.73, 1) forwards,
               pulse 0.3s ease-in-out 0.2s;
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  @keyframes pulse {
    0%, 100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.05);
    }
  }

  /* Hover effect */
  :global(.custom-toast:hover) {
    transform: translateY(-2px);
    box-shadow: 0 15px 30px rgba(0, 0, 0, 0.6) !important;
    transition: all 0.2s ease;
  }

  /* Animación de salida */
  :global([data-exit]) {
    animation: slideOut 0.2s ease-in forwards;
  }

  @keyframes slideOut {
    from {
      transform: translateX(0) scale(1);
      opacity: 1;
    }
    to {
      transform: translateX(100%) scale(0.9);
      opacity: 0;
    }
  }

  /* Estilos específicos por tipo */
  :global([data-type="success"]) {
    border-color: #4caf50 !important;
    box-shadow: 0 10px 25px rgba(76, 175, 80, 0.2) !important;
  }

  :global([data-type="error"]) {
    border-color: #f44336 !important;
    box-shadow: 0 10px 25px rgba(244, 67, 54, 0.2) !important;
  }

  :global([data-type="loading"]) {
    border-color: #007acc !important;
    box-shadow: 0 10px 25px rgba(0, 122, 204, 0.2) !important;
  }

  /* Barra lateral de acento por tipo */
  :global([data-type="success"]::before) {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 4px;
    height: 100%;
    background: #4caf50;
    border-radius: 8px 0 0 8px;
  }

  :global([data-type="error"]::before) {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 4px;
    height: 100%;
    background: #f44336;
    border-radius: 8px 0 0 8px;
  }

  :global([data-type="loading"]::before) {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 4px;
    height: 100%;
    background: linear-gradient(to bottom, #007acc, #0098ff);
    border-radius: 8px 0 0 8px;
  }

  /* Progress bar animado */
  :global(.custom-toast::after) {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    height: 2px;
    width: 100%;
    background: linear-gradient(to right, #007acc, #0098ff);
    transform-origin: left;
    animation: shrink 4s linear forwards;
  }

  @keyframes shrink {
    from {
      transform: scaleX(1);
    }
    to {
      transform: scaleX(0);
    }
  }

  /* Colores del progress bar por tipo */
  :global([data-type="success"]::after) {
    background: linear-gradient(to right, #4caf50, #66bb6a);
  }

  :global([data-type="error"]::after) {
    background: linear-gradient(to right, #f44336, #ef5350);
  }

  /* Responsive */
  @media (max-width: 640px) {
    :global(.custom-toast) {
      max-width: calc(100vw - 32px);
      margin: 0 16px;
    }
  }
</style>