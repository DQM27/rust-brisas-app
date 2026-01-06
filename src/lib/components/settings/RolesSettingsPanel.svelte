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
    Key,
    UserCircle,
    Save,
    X,
    CheckCircle2,
  } from "lucide-svelte";
  import { scale, fade, slide } from "svelte/transition";
  import { onMount } from "svelte";
  import * as roleService from "$lib/logic/role/roleService";
  import type { RoleResponse, Permission } from "$lib/types/role";
  import {
    ROLE_ADMIN_ID,
    ROLE_SUPERVISOR_ID,
    ROLE_GUARDIA_ID,
  } from "$lib/types/role";
  import { can } from "$lib/logic/permissions";
  import { currentUser } from "$lib/stores/auth";

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

  // Search state
  let searchTerm = $state("");

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

  function canCreate() {
    return $currentUser && can($currentUser, "CREATE_ROLE");
  }

  function canUpdate() {
    return $currentUser && can($currentUser, "UPDATE_ROLE");
  }

  function canDelete() {
    return $currentUser && can($currentUser, "DELETE_ROLE");
  }

  function startEdit(role: RoleResponse) {
    editingRoleId = role.id;
    formData = {
      name: role.name,
      description: role.description || "",
      permissions: [...role.permissions],
    };
    showCreateForm = true; // Use same flag to show form overlay
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
          showSuccess("Rol actualizado correctamente");
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
          showSuccess("Rol creado correctamente");
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
    if (
      !confirm("¿Estás seguro de que deseas eliminar este rol permanentemente?")
    )
      return;

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

  // Helpers
  function getGroupedPermissions(): Map<string, Permission[]> {
    const grouped = new Map<string, Permission[]>();
    for (const perm of permissions) {
      const list = grouped.get(perm.module) || [];
      list.push(perm);
      grouped.set(perm.module, list);
    }
    return grouped;
  }

  const actionTranslations: Record<string, string> = {
    view: "Ver",
    read: "Leer",
    create: "Crear",
    update: "Editar",
    delete: "Eliminar",
    export: "Exportar",
  };

  function translateAction(action: string): string {
    return actionTranslations[action.toLowerCase()] || action;
  }

  const moduleTranslations: Record<string, string> = {
    users: "Usuarios",
    roles: "Roles",
    contratistas: "Contratistas",
    empresas: "Empresas",
    proveedores: "Proveedores",
    visitantes: "Visitantes",
    ingresos: "Ingresos",
    citas: "Citas",
    settings_general: "Configuración General",
    settings_visual: "Apariencia",
    settings_security: "Seguridad",
    settings_sessions: "Sesiones",
    backup: "Copias de Seguridad",
    export: "Exportación",
    import: "Importación",
    config: "Configuración",
    trash: "Papelera",
  };

  function translateModule(module: string): string {
    return (
      moduleTranslations[module.toLowerCase()] || module.replace(/_/g, " ")
    );
  }

  function toggleModulePermissions(perms: Permission[]) {
    const allSelected = perms.every((p) => formData.permissions.includes(p.id));
    if (allSelected) {
      formData.permissions = formData.permissions.filter(
        (id) => !perms.find((p) => p.id === id),
      );
    } else {
      const newIds = perms
        .map((p) => p.id)
        .filter((id) => !formData.permissions.includes(id));
      formData.permissions = [...formData.permissions, ...newIds];
    }
  }

  function isModuleSelected(perms: Permission[]): boolean {
    return perms.every((p) => formData.permissions.includes(p.id));
  }

  // Filter roles
  let filteredRoles = $derived(
    roles.filter(
      (r) =>
        r.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        (r.description &&
          r.description.toLowerCase().includes(searchTerm.toLowerCase())),
    ),
  );
</script>

<div
  class="h-full flex flex-col bg-surface-1 p-6 relative overflow-hidden"
  in:fade
>
  <!-- Main Grid View -->
  {#if !showCreateForm}
    <div class="flex flex-col h-full" in:fade={{ duration: 200 }}>
      <!-- Header -->
      <div
        class="mb-8 flex flex-col md:flex-row md:items-center justify-between gap-4"
      >
        <div>
          <h2 class="text-2xl font-bold text-white flex items-center gap-3">
            <Shield class="text-primary-400" />
            Roles y Permisos
          </h2>
          <p class="text-gray-400 mt-1">
            Configura el acceso y control del sistema.
          </p>
        </div>

        <div class="flex items-center gap-3">
          <div class="relative">
            <input
              type="text"
              bind:value={searchTerm}
              placeholder="Buscar rol..."
              class="bg-surface-2 border border-white/10 rounded-lg pl-3 pr-8 py-2 text-sm text-white focus:outline-none focus:border-primary-500 transition-all w-64"
            />
          </div>

          <button
            onclick={startCreate}
            disabled={!canCreate()}
            class="px-4 py-2 bg-[#2563eb] hover:bg-[#1d4ed8] text-white rounded-lg font-medium transition-all flex items-center gap-2 shadow-none disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Plus size={18} />
            Nuevo Rol
          </button>
        </div>
      </div>

      <!-- Loading/Error/Success -->
      {#if loading}
        <div class="flex-1 flex items-center justify-center">
          <div class="flex flex-col items-center gap-4">
            <div class="relative w-12 h-12">
              <div
                class="absolute inset-0 border-4 border-primary-500/30 rounded-full"
              ></div>
              <div
                class="absolute inset-0 border-4 border-primary-500 border-t-transparent rounded-full animate-spin"
              ></div>
            </div>
            <p class="text-gray-400 font-medium">Cargando roles...</p>
          </div>
        </div>
      {:else}
        {#if error}
          <div
            class="p-4 mb-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 flex items-center gap-3"
          >
            <AlertCircle />
            {error}
          </div>
        {/if}
        {#if successMessage}
          <div
            class="p-4 mb-4 rounded-xl bg-green-500/10 border border-green-500/20 text-green-400 flex items-center gap-3"
            in:slide
          >
            <CheckCircle2 />
            {successMessage}
          </div>
        {/if}

        <div
          class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 overflow-y-auto pb-4 custom-scrollbar"
        >
          {#each filteredRoles as role (role.id)}
            <div
              class="bg-surface-2 border border-white/5 rounded-xl p-5 hover:border-white/10 transition-colors group relative flex flex-col h-full"
            >
              <!-- Top Bar -->
              <div class="flex justify-between items-start mb-4">
                <div
                  class="p-3 rounded-lg {role.isSystem
                    ? 'bg-purple-500/10 text-purple-400'
                    : 'bg-blue-500/10 text-blue-400'}"
                >
                  {#if role.isSystem}
                    <Lock size={24} />
                  {:else}
                    <Users size={24} />
                  {/if}
                </div>
                {#if role.isSystem}
                  <span
                    class="px-2 py-1 rounded bg-purple-500/20 text-purple-300 text-xs font-bold uppercase tracking-wider border border-purple-500/10"
                  >
                    Sistema
                  </span>
                {/if}
              </div>

              <!-- Info -->
              <div class="mb-6 flex-1">
                <h3
                  class="text-lg font-bold text-white mb-1 group-hover:text-primary-400 transition-colors"
                >
                  {role.name}
                </h3>
                <p class="text-sm text-gray-400 line-clamp-2">
                  {role.description || "Sin descripción"}
                </p>
              </div>

              <!-- Stats -->
              <div class="flex items-center gap-2 mb-6">
                <div
                  class="flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-surface-3 border border-white/5"
                >
                  <Key size={14} class="text-gray-400" />
                  <span class="text-sm font-mono text-gray-300"
                    >{role.permissions.length}</span
                  >
                  <span class="text-xs text-gray-500">permisos</span>
                </div>
              </div>

              <!-- Actions -->
              <div
                class="flex items-center gap-2 mt-auto pt-4 border-t border-white/5"
              >
                {#if canUpdate()}
                  <button
                    onclick={() => startEdit(role)}
                    class="flex-1 flex items-center justify-center gap-2 py-2 rounded-lg bg-white/5 hover:bg-white/10 text-sm font-medium text-gray-300 transition-colors"
                  >
                    <Edit2 size={16} /> Editar
                  </button>
                {/if}

                {#if !role.isSystem && canDelete()}
                  <button
                    onclick={() => handleDelete(role.id)}
                    class="p-2 rounded-lg bg-red-500/10 hover:bg-red-500/20 text-red-400 transition-colors"
                    title="Eliminar Rol"
                  >
                    <Trash2 size={18} />
                  </button>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Create/Edit Form Overlay -->
  {:else}
    <div
      class="absolute inset-0 bg-surface-1 z-20 flex flex-col p-6 overflow-hidden"
      in:slide={{ axis: "x", duration: 300 }}
    >
      <!-- Form Header -->
      <div
        class="flex items-center justify-between mb-8 pb-4 border-b border-white/5"
      >
        <div>
          <h2 class="text-2xl font-bold text-white flex items-center gap-3">
            {#if editingRoleId}
              <Edit2 class="text-primary-400" /> Editar Rol
            {:else}
              <Plus class="text-green-400" /> Nuevo Rol
            {/if}
          </h2>
          <p class="text-gray-400 mt-1">
            Configura los detalles y permisos del rol.
          </p>
        </div>

        <div class="flex items-center gap-3">
          <button
            onclick={cancelEdit}
            class="px-4 py-2 rounded-lg border border-gray-600 bg-white/5 hover:bg-white/10 text-gray-200 transition-colors font-medium text-sm"
          >
            Cancelar
          </button>
          <button
            onclick={handleSave}
            disabled={saving}
            class="px-6 py-2 bg-[#2563eb] hover:bg-[#1d4ed8] text-white rounded-lg font-medium shadow-none disabled:opacity-50 transition-all flex items-center gap-2 text-sm"
          >
            {#if saving}
              <RefreshCw size={18} class="animate-spin" /> Guardando...
            {:else}
              <Save size={18} /> Guardar Cambios
            {/if}
          </button>
        </div>
      </div>

      <!-- Form Content -->
      <div class="flex-1 overflow-y-auto custom-scrollbar pr-2">
        <!-- Basic Info -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
          <div class="space-y-2">
            <label for="name" class="block text-sm font-medium text-gray-300"
              >Nombre del Rol</label
            >
            <div class="relative">
              <UserCircle
                class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500"
                size={18}
              />
              <input
                id="name"
                type="text"
                bind:value={formData.name}
                placeholder="Ej: Auditor"
                class="w-full bg-surface-2 border border-white/10 rounded-xl pl-10 pr-4 py-3 text-white focus:outline-none focus:border-primary-500 transition-all"
              />
            </div>
          </div>

          <div class="space-y-2">
            <label for="desc" class="block text-sm font-medium text-gray-300"
              >Descripción</label
            >
            <input
              id="desc"
              type="text"
              bind:value={formData.description}
              placeholder="Descripción breve..."
              class="w-full bg-surface-2 border border-white/10 rounded-xl px-4 py-3 text-white focus:outline-none focus:border-primary-500 transition-all"
            />
          </div>
        </div>

        <!-- Permissions -->
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <h3
              class="text-lg font-semibold text-white flex items-center gap-2"
            >
              <Key class="text-yellow-400" size={20} /> Permisos
              <span class="text-sm font-normal text-gray-500 ml-2"
                >({formData.permissions.length} seleccionados)</span
              >
            </h3>
          </div>

          <div class="grid grid-cols-1 xl:grid-cols-2 gap-6 pb-8">
            {#each [...getGroupedPermissions()] as [moduleName, perms]}
              <div
                class="bg-surface-2 border border-white/5 rounded-xl overflow-hidden flex flex-col"
              >
                <!-- Module Header -->
                <div
                  class="px-4 py-3 bg-surface-3 flex items-center justify-between border-b border-white/5"
                >
                  <span
                    class="font-medium text-gray-200 uppercase tracking-wide text-sm"
                    >{translateModule(moduleName)}</span
                  >
                  <button
                    onclick={() => toggleModulePermissions(perms)}
                    class="text-xs px-2 py-1 rounded hover:bg-white/10 transition-colors {isModuleSelected(
                      perms,
                    )
                      ? 'text-primary-400'
                      : 'text-gray-500'}"
                  >
                    {isModuleSelected(perms)
                      ? "Desmarcar Todos"
                      : "Marcar Todos"}
                  </button>
                </div>

                <!-- Perms Grid -->
                <div class="p-4 grid grid-cols-2 sm:grid-cols-3 gap-3">
                  {#each perms as perm}
                    <button
                      onclick={() => togglePermission(perm.id)}
                      class="relative flex items-center justify-center py-2 px-3 rounded-lg border transition-all duration-200 text-sm {formData.permissions.includes(
                        perm.id,
                      )
                        ? 'bg-[#2563eb] border-transparent text-white shadow-md hover:bg-[#1d4ed8] ring-1 ring-white/20 font-semibold'
                        : 'bg-transparent border-[#30363d] text-gray-500 hover:text-gray-300 hover:bg-[#21262d] opacity-60 hover:opacity-100'}"
                    >
                      <span class="relative z-10"
                        >{translateAction(perm.action)}</span
                      >
                    </button>
                  {/each}
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .custom-scrollbar {
    scrollbar-width: thin;
    scrollbar-color: rgba(255, 255, 255, 0.1) transparent;
  }
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background-color: rgba(255, 255, 255, 0.1);
    border-radius: 20px;
  }
</style>
