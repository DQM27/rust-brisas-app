<!-- src/lib/components/settings/RolesSettingsPanel.svelte -->
<script lang="ts">
  import {
    Shield,
    Users,
    Plus,
    RefreshCw,
    Check,
    AlertCircle,
    Edit2,
    Trash2,
    Lock,
  } from "lucide-svelte";
  import { scale } from "svelte/transition";
  import { onMount } from "svelte";
  import * as roleService from "$lib/logic/role/roleService";
  import type { RoleResponse, Permission } from "$lib/types/role";
  import {
    ROLE_ADMIN_ID,
    ROLE_SUPERVISOR_ID,
    ROLE_GUARDIA_ID,
  } from "$lib/types/role";

  // Estado
  let roles = $state<RoleResponse[]>([]);
  let permissions = $state<Permission[]>([]);
  let loading = $state(true);
  let error = $state("");
  let successMessage = $state("");

  // Formulario
  let showCreateForm = $state(false);
  let editingRoleId = $state<string | null>(null);
  let formData = $state({
    name: "",
    description: "",
    permissions: [] as string[],
  });
  let saving = $state(false);

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    loading = true;
    error = "";
    try {
      const [rolesResult, permsResult] = await Promise.all([
        roleService.fetchAllRoles(),
        roleService.fetchAllPermissions(),
      ]);

      if (rolesResult.ok) {
        roles = rolesResult.data.roles;
      } else {
        error = rolesResult.error;
      }

      if (permsResult.ok) {
        permissions = permsResult.data;
      }
    } catch (e) {
      error = `Error cargando datos: ${e}`;
    } finally {
      loading = false;
    }
  }

  function showSuccess(msg: string) {
    successMessage = msg;
    setTimeout(() => (successMessage = ""), 3000);
  }

  function isSystemRole(roleId: string): boolean {
    return [ROLE_ADMIN_ID, ROLE_SUPERVISOR_ID, ROLE_GUARDIA_ID].includes(
      roleId,
    );
  }

  function startEdit(role: RoleResponse) {
    editingRoleId = role.id;
    formData = {
      name: role.name,
      description: role.description || "",
      permissions: [...role.permissions],
    };
    showCreateForm = false;
  }

  function startCreate() {
    showCreateForm = true;
    editingRoleId = null;
    formData = {
      name: "",
      description: "",
      permissions: [],
    };
  }

  function cancelEdit() {
    editingRoleId = null;
    showCreateForm = false;
    formData = { name: "", description: "", permissions: [] };
  }

  function togglePermission(permId: string) {
    if (formData.permissions.includes(permId)) {
      formData.permissions = formData.permissions.filter((p) => p !== permId);
    } else {
      formData.permissions = [...formData.permissions, permId];
    }
  }

  async function handleSave() {
    if (!formData.name.trim()) {
      error = "El nombre es requerido";
      return;
    }

    saving = true;
    error = "";

    try {
      if (editingRoleId) {
        const result = await roleService.updateRole(editingRoleId, {
          name: formData.name,
          description: formData.description || undefined,
          permissions: formData.permissions,
        });
        if (result.ok) {
          showSuccess("Rol actualizado");
          cancelEdit();
          await loadData();
        } else {
          error = result.error;
        }
      } else {
        const result = await roleService.createRole({
          name: formData.name,
          description: formData.description || undefined,
          permissions: formData.permissions,
        });
        if (result.ok) {
          showSuccess("Rol creado");
          cancelEdit();
          await loadData();
        } else {
          error = result.error;
        }
      }
    } catch (e) {
      error = `Error: ${e}`;
    } finally {
      saving = false;
    }
  }

  async function handleDelete(roleId: string) {
    if (!confirm("¿Eliminar este rol?")) return;

    try {
      const result = await roleService.deleteRole(roleId);
      if (result.ok) {
        showSuccess("Rol eliminado");
        await loadData();
      } else {
        error = result.error;
      }
    } catch (e) {
      error = `Error: ${e}`;
    }
  }

  // Agrupar permisos por módulo
  function getGroupedPermissions(): Map<string, Permission[]> {
    const grouped = new Map<string, Permission[]>();
    for (const perm of permissions) {
      const list = grouped.get(perm.module) || [];
      list.push(perm);
      grouped.set(perm.module, list);
    }
    return grouped;
  }
</script>

<div
  class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
  in:scale={{ duration: 300, start: 0.95 }}
>
  <div class="max-w-4xl space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">
          Roles y Permisos
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
          Gestiona los roles del sistema y sus permisos asociados.
        </p>
      </div>
      <button
        onclick={startCreate}
        class="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white"
      >
        <Plus class="w-4 h-4" />
        Nuevo Rol
      </button>
    </div>

    {#if loading}
      <div class="flex items-center justify-center py-12">
        <RefreshCw class="w-6 h-6 animate-spin text-gray-400" />
      </div>
    {:else}
      {#if error}
        <div
          class="p-3 rounded-md bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/30 flex items-start gap-2"
        >
          <AlertCircle
            class="w-4 h-4 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5"
          />
          <span class="text-sm text-red-700 dark:text-red-300">{error}</span>
        </div>
      {/if}

      {#if successMessage}
        <div
          class="p-3 rounded-md bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800/30 flex items-start gap-2"
        >
          <Check
            class="w-4 h-4 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5"
          />
          <span class="text-sm text-green-700 dark:text-green-300"
            >{successMessage}</span
          >
        </div>
      {/if}

      <!-- Create/Edit Form -->
      {#if showCreateForm || editingRoleId}
        <div
          class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] overflow-hidden"
        >
          <div
            class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2"
          >
            <Shield class="w-4 h-4 text-gray-500" />
            <h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
              {editingRoleId ? "Editar Rol" : "Nuevo Rol"}
            </h3>
          </div>
          <div class="p-4 space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label
                  for="roleName"
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                  >Nombre</label
                >
                <input
                  id="roleName"
                  type="text"
                  bind:value={formData.name}
                  placeholder="Ej: Auditor"
                  class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100"
                />
              </div>
              <div>
                <label
                  for="roleDesc"
                  class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                  >Descripción</label
                >
                <input
                  id="roleDesc"
                  type="text"
                  bind:value={formData.description}
                  placeholder="Descripción opcional"
                  class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100"
                />
              </div>
            </div>

            <!-- Permissions Grid -->
            <div>
              <span
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
                >Permisos ({formData.permissions.length} seleccionados)</span
              >
              <div
                class="max-h-64 overflow-y-auto border border-gray-200 dark:border-gray-700 rounded-md p-3"
              >
                {#each [...getGroupedPermissions()] as [moduleName, perms]}
                  <div class="mb-3">
                    <div
                      class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase mb-1"
                    >
                      {moduleName}
                    </div>
                    <div class="flex flex-wrap gap-2">
                      {#each perms as perm}
                        <button
                          type="button"
                          onclick={() => togglePermission(perm.id)}
                          class="px-2 py-1 text-xs rounded-md border transition-colors {formData.permissions.includes(
                            perm.id,
                          )
                            ? 'bg-[#2da44e] text-white border-[#2da44e]'
                            : 'bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 border-gray-300 dark:border-gray-600 hover:bg-gray-200 dark:hover:bg-gray-700'}"
                        >
                          {perm.action}
                        </button>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
            </div>

            <div class="flex items-center gap-2 pt-2">
              <button
                onclick={handleSave}
                disabled={saving}
                class="inline-flex items-center gap-2 px-3 py-1.5 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white disabled:opacity-50"
              >
                {#if saving}<RefreshCw
                    class="w-4 h-4 animate-spin"
                  />{:else}<Check class="w-4 h-4" />{/if}
                <span>Guardar</span>
              </button>
              <button
                onclick={cancelEdit}
                class="px-3 py-1.5 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d]"
              >
                Cancelar
              </button>
            </div>
          </div>
        </div>
      {/if}

      <!-- Roles List -->
      <div
        class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] overflow-hidden"
      >
        <div
          class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2"
        >
          <Users class="w-4 h-4 text-gray-500" />
          <h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
            Roles del Sistema ({roles.length})
          </h3>
        </div>
        <div class="divide-y divide-gray-200 dark:divide-gray-700">
          {#each roles as role}
            <div
              class="p-4 flex items-center justify-between hover:bg-gray-50 dark:hover:bg-[#161b22] transition-colors"
            >
              <div class="flex-1">
                <div class="flex items-center gap-2">
                  <span class="font-medium text-gray-900 dark:text-gray-100"
                    >{role.name}</span
                  >
                  {#if role.isSystem}
                    <span
                      class="inline-flex items-center gap-1 px-1.5 py-0.5 text-xs rounded-md bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300"
                    >
                      <Lock class="w-3 h-3" />
                      Sistema
                    </span>
                  {/if}
                </div>
                {#if role.description}
                  <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5">
                    {role.description}
                  </p>
                {/if}
                <p class="text-xs text-gray-400 dark:text-gray-500 mt-1">
                  {role.permissions.length} permisos
                </p>
              </div>
              <div class="flex items-center gap-2">
                <button
                  onclick={() => startEdit(role)}
                  class="p-1.5 rounded-md text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800"
                  title="Editar"
                >
                  <Edit2 class="w-4 h-4" />
                </button>
                {#if !role.isSystem}
                  <button
                    onclick={() => handleDelete(role.id)}
                    class="p-1.5 rounded-md text-gray-500 hover:text-red-600 dark:hover:text-red-400 hover:bg-gray-100 dark:hover:bg-gray-800"
                    title="Eliminar"
                  >
                    <Trash2 class="w-4 h-4" />
                  </button>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>
