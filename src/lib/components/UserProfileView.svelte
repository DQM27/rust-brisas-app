<script lang="ts">
  import { onMount } from "svelte";
  import { currentUser } from "$lib/stores/auth";
  import UserProfilePanel from "$lib/components/UserProfilePanel.svelte";
  import { users } from "$lib/api/users";
  import * as userService from "$lib/logic/user/userService";
  import type { UserResponse, UpdateUserInput } from "$lib/types/user";
  import { toast } from "svelte-5-french-toast";
  import {
    getPermissionsForUser,
    type UserPermissions,
  } from "$lib/logic/permissions";

  interface Props {
    data?: { userId?: string };
  }

  let { data = {} }: Props = $props();

  let loading = $state(true);
  let user = $state<UserResponse | null>(null);

  // Permisos derivados automáticamente del estado
  let permissions = $derived.by(() => {
    if (user && $currentUser) {
      return getPermissionsForUser($currentUser, user);
    }
    return {
      canEditBasic: false,
      canEditSensitive: false,
      canChangePassword: false,
      canResetPassword: false,
      canDelete: false,
    };
  });

  // Efecto para cargar datos cuando cambia el ID o currentUser
  $effect(() => {
    // Leemos las dependencias para asegurar reactividad
    const currentId = $currentUser?.id;
    const targetId = data.userId;
    loadUser();
  });

  async function loadUser() {
    loading = true;
    const targetId = data.userId || $currentUser?.id;

    if (!targetId) {
      loading = false;
      return;
    }

    try {
      // Optimización: si es el mismo currentUser, usar store, sino fetch
      if ($currentUser && $currentUser.id === targetId && !data.userId) {
        user = $currentUser;
      } else {
        const res = await users.getById(targetId);
        user = res;
      }

      // La actualización de permisos ahora es automática por $derived
    } catch (err) {
      console.error(err);
      toast.error("Error al cargar perfil");
    } finally {
      loading = false;
    }
  }

  async function handleUpdate(data: UpdateUserInput) {
    if (!user) return;

    try {
      const updated = await users.update(user.id, data);

      // Si estamos editando el usuario actual, actualizar store (opcional, pero buena práctica)
      // Nota: auth store se actualiza usualmente al recargar sesión, pero aquí es visual
      user = updated;
      toast.success("Perfil actualizado");
    } catch (err: any) {
      console.error("Error updating user:", err);
      throw typeof err === "string" ? err : "Error al actualizar perfil";
    }
  }

  async function handleStatusChange(isActive: boolean) {
    if (!user) return;
    try {
      const result = await userService.changeStatus(user.id, isActive);
      if (result.ok) {
        user = result.data;
        toast.success(`Usuario ${isActive ? "activado" : "desactivado"}`);
      } else {
        toast.error(result.error);
      }
    } catch (err) {
      toast.error("Error al cambiar estado");
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
    <UserProfilePanel
      {user}
      {permissions}
      isSelf={$currentUser?.id === user.id}
      onUpdate={handleUpdate}
      onStatusChange={handleStatusChange}
    />
  {:else}
    <div
      class="flex h-full items-center justify-center flex-col gap-4 text-tertiary"
    >
      <p>No se encontró información del usuario.</p>
    </div>
  {/if}
</div>
