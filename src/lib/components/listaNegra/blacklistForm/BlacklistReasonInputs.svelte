<!-- BlacklistReasonInputs.svelte -->
<script lang="ts">
  interface Props {
    motivo: string;
    observaciones: string;
    isUnblock?: boolean;
    loading?: boolean;
    onMotivoChange: (value: string) => void;
    onObservacionesChange: (value: string) => void;
  }

  let {
    motivo,
    observaciones,
    isUnblock = false,
    loading = false,
    onMotivoChange,
    onObservacionesChange,
  }: Props = $props();

  const motivoLabel = isUnblock
    ? "Motivo del Desbloqueo"
    : "Motivo del Bloqueo";
  const motivoPlaceholder = isUnblock
    ? "Detalle la razón de la desactivación del bloqueo (ej: Cumplió sanción, revisión de caso, etc.)"
    : "Detalle la razón del bloqueo (ej: Agresión verbal a personal de seguridad, incumplimiento grave de normas, etc.)";
  const observacionesLabel = isUnblock
    ? "Observaciones de Desbloqueo (Opcional)"
    : "Observaciones Adicionales (Opcional)";
  const observacionesPlaceholder = isUnblock
    ? "Notas internas sobre el desbloqueo."
    : "Notas internas sobre el incidente.";
</script>

<div class="space-y-5">
  <div class="space-y-1.5">
    <label for="motivoBloqueo" class="text-sm font-medium text-primary">
      {motivoLabel} <span class="text-error">*</span>
    </label>
    <textarea
      id="motivoBloqueo"
      value={motivo}
      oninput={(e) => onMotivoChange(e.currentTarget.value)}
      rows="3"
      disabled={loading}
      placeholder={motivoPlaceholder}
      class="input-base w-full resize-y"
    ></textarea>
  </div>

  <div class="space-y-1.5">
    <label for="observaciones" class="text-sm font-medium text-primary">
      {observacionesLabel}
    </label>
    <textarea
      id="observaciones"
      value={observaciones}
      oninput={(e) => onObservacionesChange(e.currentTarget.value)}
      rows="2"
      disabled={loading}
      placeholder={observacionesPlaceholder}
      class="input-base w-full resize-y"
    ></textarea>
  </div>
</div>
