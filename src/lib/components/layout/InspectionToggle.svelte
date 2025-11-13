<!-- src/lib/components/common/InspectionToggle.svelte -->
<script lang="ts">
  import { Eye, EyeOff } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';

  // Props del componente
  export let visible: boolean = false;
  export let size: number = 14;
  export let showLabel: boolean = true;
  export let variant: 'default' | 'compact' | 'icon-only' = 'default';
  export let disabled: boolean = false;

  // Event dispatcher
  const dispatch = createEventDispatcher<{
    toggle: { visible: boolean };
  }>();

  // Computed properties
  $: title = visible 
    ? 'Ocultar panel de inspección' 
    : 'Mostrar panel de inspección';
  
  $: ariaLabel = visible 
    ? 'Ocultar panel de inspección' 
    : 'Mostrar panel de inspección';

  // Handlers
  function handleClick(): void {
    if (disabled) return;
    
    const newVisible = !visible;
    console.log(`Toggle panel de inspección: ${newVisible ? 'visible' : 'oculto'}`);
    
    dispatch('toggle', { visible: newVisible });
  }

  function handleKeyPress(event: KeyboardEvent): void {
    if (disabled) return;
    
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleClick();
    }
  }
</script>

<button
  class="inspection-toggle {variant} {visible ? 'active' : ''}"
  class:disabled
  on:click={handleClick}
  on:keydown={handleKeyPress}
  type="button"
  title={title}
  aria-label={ariaLabel}
  aria-pressed={visible}
  {disabled}
>
  <span class="icon-wrapper" aria-hidden="true">
    {#if visible}
      <EyeOff {size} />
    {:else}
      <Eye {size} />
    {/if}
  </span>
  
  {#if showLabel && variant !== 'icon-only'}
    <span class="label">
      {#if variant === 'compact'}
        Inspección
      {:else}
        {visible ? 'Ocultar' : 'Mostrar'} Inspección
      {/if}
    </span>
  {/if}
</button>

<style>
  .inspection-toggle {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 2px 8px;
    border-radius: 3px;
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
    border: none;
    background: transparent;
    color: var(--inspection-toggle-color, inherit);
    font-size: var(--inspection-toggle-font-size, 12px);
    font-family: inherit;
    user-select: none;
  }

  /* Variantes del componente */
  .inspection-toggle.compact {
    padding: 2px 6px;
    gap: 4px;
  }

  .inspection-toggle.icon-only {
    padding: 4px;
    gap: 0;
  }

  /* Estados interactivos */
  .inspection-toggle:hover:not(.disabled) {
    background: var(--inspection-toggle-hover, rgba(255, 255, 255, 0.08));
  }

  .inspection-toggle:active:not(.disabled) {
    transform: scale(0.98);
  }

  /* Estado activo (panel visible) */
  .inspection-toggle.active {
    background: var(--inspection-toggle-active-bg, rgba(0, 122, 204, 0.4));
    color: var(--inspection-toggle-active-color, #ffffff);
  }

  .inspection-toggle.active:hover:not(.disabled) {
    background: var(--inspection-toggle-active-hover, rgba(0, 122, 204, 0.5));
  }

  /* Estado deshabilitado */
  .inspection-toggle.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Focus para accesibilidad */
  .inspection-toggle:focus-visible {
    outline: 2px solid var(--inspection-toggle-focus, #007acc);
    outline-offset: 2px;
  }

  /* Elementos internos */
  .icon-wrapper {
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.15s ease;
  }

  .inspection-toggle:hover:not(.disabled) .icon-wrapper {
    transform: scale(1.1);
  }

  .label {
    font-weight: 500;
    line-height: 1;
  }

  /* Animación de entrada */
  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(-2px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .inspection-toggle {
    animation: fadeIn 0.2s ease;
  }

  /* Responsive */
  @media (max-width: 768px) {
    .inspection-toggle.default {
      padding: 4px 6px;
      font-size: 11px;
    }

    .inspection-toggle.default .label {
      display: none;
    }
  }

  /* Tema oscuro (opcional) */
  @media (prefers-color-scheme: dark) {
    .inspection-toggle {
      --inspection-toggle-hover: rgba(255, 255, 255, 0.1);
    }
  }

  /* Tema claro (opcional) */
  @media (prefers-color-scheme: light) {
    .inspection-toggle {
      --inspection-toggle-hover: rgba(0, 0, 0, 0.05);
      --inspection-toggle-active-bg: rgba(0, 122, 204, 0.15);
      --inspection-toggle-active-color: #007acc;
    }
  }
</style>