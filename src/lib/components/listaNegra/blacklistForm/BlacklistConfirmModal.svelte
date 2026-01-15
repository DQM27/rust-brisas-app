<!-- BlacklistConfirmModal.svelte -->
<script lang="ts">
	import { fade, fly } from 'svelte/transition';

	interface Props {
		show: boolean;
		contratistaName: string;
		motivo: string;
		onConfirm: () => void;
		onCancel: () => void;
		onMotivoChange: (value: string) => void;
	}

	let { show, contratistaName, motivo, onConfirm, onCancel, onMotivoChange }: Props = $props();
</script>

{#if show}
	<div class="modal-overlay fixed inset-0 z-50 flex items-center justify-center" transition:fade>
		<div class="card-base max-w-sm w-full p-6" transition:fly={{ y: 20, duration: 300 }}>
			<h3 class="text-lg font-semibold text-primary mb-4">Confirmar Desbloqueo</h3>
			<p class="text-sm text-secondary mb-6">
				Está a punto de desactivar el bloqueo para <strong>{contratistaName}</strong>. El motivo
				será registrado en el historial.
			</p>

			<div class="space-y-4">
				<div class="space-y-1.5">
					<label for="unblockMotivo" class="text-sm font-medium text-primary">
						Motivo del Desbloqueo
					</label>
					<textarea
						id="unblockMotivo"
						value={motivo}
						oninput={(e) => onMotivoChange(e.currentTarget.value)}
						rows="2"
						placeholder="Motivo de la desactivación del bloqueo (opcional)"
						class="input-base w-full resize-y"
					></textarea>
				</div>
			</div>

			<div class="mt-6 flex justify-end gap-3">
				<button type="button" onclick={onCancel} class="btn-ghost btn-base"> Cancelar </button>
				<button
					type="button"
					onclick={onConfirm}
					class="btn-base bg-success text-white hover:bg-success-hover"
				>
					Confirmar Desbloqueo
				</button>
			</div>
		</div>
	</div>
{/if}
