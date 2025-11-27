<script lang="ts">
  import type { ButtonVariant, ButtonState } from '$lib/types/agGrid';
  import { Loader2 } from 'lucide-svelte';

  interface Props {
    button: {
      id: string;
      label: string;
      icon?: any;
      variant?: ButtonVariant;
      tooltip?: string;
      state?: ButtonState;
    };
    disabled?: boolean;
    onclick: () => void;
  }

  let {
    button,
    disabled = false,
    onclick
  }: Props = $props();

  const variant = $derived(button.variant || 'default');
  const state = $derived(button.state || 'normal');
  const isLoading = $derived(state === 'loading');
  const isDisabled = $derived(disabled || state === 'disabled');

  // Clases de variante
  const variantClasses = $derived.by(() => {
    const base = "flex items-center gap-1.5 px-3 py-2 rounded-md text-xs font-medium cursor-pointer transition-all duration-200 whitespace-nowrap";
    
    if (isDisabled) {
      return `${base} bg-[#1e1e1e] border border-white/10 text-white/30 cursor-not-allowed opacity-50`;
    }

    switch (state) {
      case 'success':
        return `${base} bg-green-500/20 border border-green-500/40 text-green-400`;
      case 'error':
        return `${base} bg-red-500/20 border border-red-500/40 text-red-400`;
      case 'loading':
        return `${base} bg-blue-500/20 border border-blue-500/40 text-blue-400`;
      default:
        break;
    }

    switch (variant) {
      case 'primary':
        return `${base} bg-blue-500/10 border border-blue-500/20 text-blue-400 hover:bg-blue-500/20 hover:border-blue-500/30`;
      case 'danger':
        return `${base} bg-red-500/10 border border-red-500/20 text-red-400 hover:bg-red-500/20 hover:border-red-500/30`;
      case 'success':
        return `${base} bg-green-500/10 border border-green-500/20 text-green-400 hover:bg-green-500/20 hover:border-green-500/30`;
      default:
        return `${base} bg-[#1e1e1e] border border-white/10 text-white hover:bg-white/5 hover:border-white/20`;
    }
  });

  function handleClick() {
    if (!isDisabled && !isLoading) {
      onclick();
    }
  }
</script>

<button
  onclick={handleClick}
  class={variantClasses}
  disabled={isDisabled}
  title={button.tooltip || button.label}
>
  {#if isLoading}
    <Loader2 size={16} class="animate-spin" />
  {:else if button.icon}
    {@const Icon = button.icon}
    <Icon size={16} />
  {/if}
  {button.label}
</button>