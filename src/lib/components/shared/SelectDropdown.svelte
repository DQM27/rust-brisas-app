<script lang="ts" generics="T">
	import { ChevronDown, Check } from '@lucide/svelte';
	import { scale } from 'svelte/transition';

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
		class: className = '',
		placeholder = 'Seleccionar...',
		onSelect
	}: Props = $props();

	let isOpen = $state(false);

	function handleSelect(opt: Option<T>) {
		onSelect(opt.value);
		isOpen = false;
	}

	// Styles inspired by UserFormModal, enabling centralized theme control
	const buttonClass = 'form-select text-left flex items-center justify-between cursor-pointer';
	const labelStyle = 'form-label ml-0.5';
	// form-select handles active/focus states via theme, but we can keep overrides if passed via className
	const activeClass = 'border-accent ring-1 ring-accent-bg'; // Matching manual active state if needed, or rely on focus-within

	const selectedLabel = $derived(options.find((o) => o.value === value)?.label ?? placeholder);
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
		<span class="truncate pr-2 {value ? 'text-primary' : 'text-secondary'}">
			{selectedLabel}
		</span>
		<ChevronDown size={14} class="text-secondary flex-shrink-0" />
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
			class="form-dropdown absolute z-50 w-full mt-1 origin-top min-w-[150px]"
			transition:scale={{ duration: 150, start: 0.95 }}
		>
			<div class="max-h-[200px] overflow-y-auto custom-scrollbar">
				{#each options as opt}
					<button
						type="button"
						onclick={() => handleSelect(opt)}
						class="form-dropdown-item w-full flex items-center justify-between group"
					>
						<span class={value === opt.value ? 'text-primary font-medium' : ''}>{opt.label}</span>
						{#if value === opt.value}
							<Check size={14} class="text-accent" />
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
