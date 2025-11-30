<script lang="ts">
  import { toast } from "svelte-5-french-toast";

  import ListaNegraForm from "$lib/components/listaNegra/Listanegraform.svelte";
  import type ListaNegraFormType from "$lib/components/listaNegra/Listanegraform.svelte";

  import type { AddToListaNegraInput } from "$lib/types/listaNegra";
  import * as listaNegraService from "$lib/logic/listaNegra/listaNegraService";

  let loading = false;
  let formRef: ListaNegraFormType;

  async function handleRegister(data: AddToListaNegraInput) {
    loading = true;

    const result = await listaNegraService.add(data);

    if (result.ok) {
      formRef?.reset();
      toast.success("Persona bloqueada exitosamente", {
        icon: "🚫",
        duration: 3000,
      });
    } else {
      toast.error(result.error, { icon: "✕", duration: 4000 });
    }

    loading = false;
  }

  async function handleUnblock(data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) {
    loading = true;

    const result = await listaNegraService.unblock(
      data.id,
      data.motivoDesbloqueo,
      data.observaciones,
    );

    if (result.ok) {
      formRef?.reset();
      toast.success("Persona desbloqueada exitosamente", {
        icon: "✅",
        duration: 3000,
      });
    } else {
      toast.error(result.error ?? "Error desconocido", {
        icon: "✕",
        duration: 4000,
      });
    }

    loading = false;
  }
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <ListaNegraForm
    bind:this={formRef}
    {loading}
    onSubmit={handleRegister}
    onUnblock={handleUnblock}
  />
</div>
