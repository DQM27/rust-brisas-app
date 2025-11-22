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

  // Clases base compartidas
  const baseClasses = `
    inline-flex items-center border-none bg-transparent font-inherit 
    whitespace-nowrap select-none cursor-pointer
    transition-all duration-150 ease-in-out
    animate-in fade-in slide-in-from-top-1 duration-200
    focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[#007acc] focus-visible:ring-offset-2
  `;

  // Clases de variantes
  const variantClasses = {
    default: 'gap-1.5 px-2 py-0.5 text-xs',
    compact: 'gap-1 px-1.5 py-0.5 text-xs',
    'icon-only': 'gap-0 p-1'
  };

  // Clases de estado
  const stateClasses = `
    hover:bg-white/[0.08] active:scale-[0.98]
    disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-transparent disabled:active:scale-100
  `;

  // Clases de estado activo
  const activeClasses = visible
    ? 'bg-[#007acc]/40 text-white hover:bg-[#007acc]/50'
    : '';

  // Clases responsivas para la variante default
  const responsiveClasses = variant === 'default' 
    ? 'md:gap-1.5 md:px-2 md:text-xs' 
    : '';
</script>

<button
  class="{baseClasses} {variantClasses[variant]} {stateClasses} {activeClasses} {responsiveClasses} rounded-sm"
  class:disabled
  on:click={handleClick}
  on:keydown={handleKeyPress}
  type="button"
  title={title}
  aria-label={ariaLabel}
  aria-pressed={visible}
  {disabled}
>
  <span 
    class="flex items-center justify-center transition-transform duration-150 ease-in-out
           group-hover:scale-110"
    class:group-hover={!disabled}
    aria-hidden="true"
  >
    {#if visible}
      <EyeOff {size} class="transition-transform duration-150 {!disabled ? 'hover:scale-110' : ''}" />
    {:else}
      <Eye {size} class="transition-transform duration-150 {!disabled ? 'hover:scale-110' : ''}" />
    {/if}
  </span>
  
  {#if showLabel && variant !== 'icon-only'}
    <span class="font-medium leading-none {variant === 'default' ? 'max-md:hidden' : ''}">
      {#if variant === 'compact'}
        Inspección
      {:else}
        {visible ? 'Ocultar' : 'Mostrar'} Inspección
      {/if}
    </span>
  {/if}
</button>