<script lang="ts" generics="T">
  import { ChevronDown, Check } from "lucide-svelte";
  import { scale } from "svelte/transition";

  interface Option<T> {
    value: T;
    label: string;
  }

  interface Props {
    label?: string;
    value: T;
    options: Option<T>[];
    disabled?: boolean;
    class?: string;
    placeholder?: string;
    onSelect: (value: T) => void;
  }

  let {
    label,
    value,
    options,
    disabled = false,
    class: className = "",
    placeholder = "Seleccionar...",
    onSelect,
  }: Props = $props();

  let isOpen = $state(false);

  function handleSelect(opt: Option<T>) {
    onSelect(opt.value);
    isOpen = false;
  }

  // Styles inspired by UserFormModal
  const buttonClass =
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white focus:outline-none disabled:opacity-50 transition-all cursor-pointer flex items-center justify-between text-left hover:border-white/20";
  const labelStyle = "block text-xs font-medium text-zinc-400 mb-1.5 ml-0.5";
  const activeClass = "!border-blue-500/50 !ring-1 !ring-blue-500/20";

  const selectedLabel = $derived(
    options.find((o) => o.value === value)?.label ?? placeholder,
  );
  const labelId = `dropdown-label-${Math.random().toString(36).slice(2, 9)}`;
</script>

<div class="relative {className}">
  {#if label}
    <label class={labelStyle} for={labelId}>{label}</label>
  {/if}

  <button
    id={labelId}
    type="button"
    {disabled}
    onclick={() => (isOpen = !isOpen)}
    class="{buttonClass} {isOpen ? activeClass : ''}"
  >
    <span class="truncate pr-2 {value ? 'text-white' : 'text-zinc-500'}">
      {selectedLabel}
    </span>
    <ChevronDown size={14} class="text-zinc-500 flex-shrink-0" />
  </button>

  {#if isOpen && !disabled}
    <!-- Backdrop -->
    <div
      class="fixed inset-0 z-40"
      onclick={() => (isOpen = false)}
      role="presentation"
      aria-hidden="true"
    ></div>

    <div
      class="absolute z-50 w-full mt-1 bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-top min-w-[150px]"
      transition:scale={{ duration: 150, start: 0.95 }}
    >
      <div class="max-h-[200px] overflow-y-auto custom-scrollbar">
        {#each options as opt}
          <button
            type="button"
            onclick={() => handleSelect(opt)}
            class="w-full text-left px-3 py-1.5 text-xs text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
          >
            <span class={value === opt.value ? "text-white font-medium" : ""}
              >{opt.label}</span
            >
            {#if value === opt.value}
              <Check size={14} class="text-blue-500" />
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
  }
</style>
