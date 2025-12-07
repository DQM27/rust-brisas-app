<script lang="ts">
  import { onMount } from "svelte";
  import { currentUser } from "$lib/stores/auth";
  import UserProfilePanel from "$lib/components/UserProfilePanel.svelte";
  import { users } from "$lib/api/users";
  import type { UserResponse, UpdateUserInput } from "$lib/types/user";
  import { toast } from "svelte-5-french-toast";

  let loading = $state(true);
  let user = $state<UserResponse | null>(null);

  onMount(async () => {
    if ($currentUser?.id) {
      try {
        user = await users.getById($currentUser.id);
      } catch (err) {
        console.error(err);
        toast.error("Error al cargar perfil");
      } finally {
        loading = false;
      }
    } else {
      loading = false;
    }
  });

  async function handleUpdate(data: UpdateUserInput) {
    if (!user) return;

    try {
      const updated = await users.update(user.id, data);
      user = updated; // Update local view
      toast.success("Perfil actualizado");
    } catch (err: any) {
      console.error("Error updating user:", err);
      throw typeof err === "string" ? err : "Error al actualizar perfil";
    }
  }
</script>

<div class="h-full w-full overflow-y-auto bg-surface-1">
  {#if loading}
    <div class="flex h-full items-center justify-center">
      <div
        class="h-8 w-8 animate-spin rounded-full border-4 border-accent border-t-transparent"
      ></div>
    </div>
  {:else if user}
    <UserProfilePanel {user} onUpdate={handleUpdate} />
  {:else}
    <div
      class="flex h-full items-center justify-center flex-col gap-4 text-tertiary"
    >
      <p>No se encontró información del usuario.</p>
    </div>
  {/if}
</div>
