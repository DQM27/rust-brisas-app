export interface ConfirmOptions {
	title: string;
	message: string;
	type?: 'warning' | 'danger' | 'info';
	confirmText?: string;
	cancelText?: string;
	onConfirm: () => void | Promise<void>;
}

export const confirmState = $state<{
	isOpen: boolean;
	options: ConfirmOptions;
}>({
	isOpen: false,
	options: {
		title: '',
		message: '',
		type: 'warning',
		confirmText: 'Confirmar',
		cancelText: 'Cancelar',
		onConfirm: () => {}
	}
});

export function openConfirm(options: ConfirmOptions) {
	confirmState.options = {
		type: 'warning',
		confirmText: 'Confirmar',
		cancelText: 'Cancelar',
		...options
	};
	confirmState.isOpen = true;
}

export function closeConfirm() {
	confirmState.isOpen = false;
}
