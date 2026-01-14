<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import {
    AlertCircle,
    RotateCw,
    Database,
    Download,
    Upload,
    Trash2,
    Clock,
    Settings,
    ArchiveRestore,
  } from "lucide-svelte";
  import type { ColDef } from "@ag-grid-community/core";

  // Components
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";

  // Services
  import {
    listBackups,
    deleteBackup,
    restoreFromAutoBackup,
    cleanupOldBackups,
    getBackupConfig,
    updateBackupConfig,
    backupDatabaseAuto,
    backupDatabase,
    restoreDatabase,
  } from "$lib/services/backupService";
  import { message, confirm } from "@tauri-apps/plugin-dialog";

  // Types
  import type { BackupEntry, BackupConfig } from "$lib/types/backup";
  import type { CustomToolbarButton } from "$lib/types/agGrid";

  // Logic
  import { BackupColumns } from "$lib/logic/backup/backupColumns";
  import { createCustomButton } from "$lib/config/agGridConfigs";

  // Stores
  import { currentUser } from "$lib/stores/auth";
  import { can } from "$lib/logic/permissions";

  // ==========================================
  // ESTADO LOCAL
  // ==========================================
  let backups = $state<BackupEntry[]>([]);
  let loading = $state(false);
  let error = $state("");

  // Config
  let config = $state<BackupConfig | null>(null);
  let configEnabled = $state(false);
  let configHora = $state("02:00");
  let configDiasRetencion = $state(30);
  let isSavingConfig = $state(false);

  // Selección
  let selectedRows = $state<BackupEntry[]>([]);

  // Permisos
  const canUpdate = $derived(
    $currentUser && can($currentUser, "UPDATE_SETTINGS_BACKUP"),
  );

  // ==========================================
  // COLUMNAS AG GRID
  // ==========================================
  const columnDefs: ColDef<BackupEntry>[] = BackupColumns.getColumns({
    onRestore: handleRestore,
    onDelete: handleDelete,
  });

  // ==========================================
  // BOTONES TOOLBAR POR CONTEXTO
  // ==========================================
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    const defaultBtns: CustomToolbarButton[] = [
      {
        id: "backup-now",
        label: "Crear Backup",
        icon: Database,
        onClick: handleBackupNow,
        variant: "success",
        tooltip: "Crear backup inmediato al directorio automático",
      },
      {
        id: "backup-manual",
        label: "Exportar",
        icon: Download,
        onClick: handleBackupManual,
        variant: "default",
        tooltip: "Exportar a ubicación personalizada",
      },
      {
        id: "restore-file",
        label: "Importar",
        icon: Upload,
        onClick: handleRestoreFromFile,
        variant: "default",
        tooltip: "Restaurar desde archivo externo",
      },
      {
        id: "refresh",
        label: "Actualizar",
        icon: RotateCw,
        onClick: loadBackups,
        variant: "default",
        tooltip: "Recargar lista de backups",
      },
    ];

    const singleSelectBtns: CustomToolbarButton[] = [
      {
        id: "restore",
        label: "Restaurar",
        icon: ArchiveRestore,
        onClick: () => {
          if (selected) handleRestore(selected);
        },
        variant: "primary",
        tooltip: "Restaurar este backup",
      },
      {
        id: "delete",
        label: "Eliminar",
        icon: Trash2,
        onClick: () => {
          if (selected) handleDelete(selected);
        },
        variant: "danger",
        tooltip: "Eliminar backup",
      },
    ];

    const multiSelectBtns: CustomToolbarButton[] = [
      {
        id: "delete-multi",
        label: `Eliminar (${selectedRows.length})`,
        icon: Trash2,
        onClick: handleDeleteMultiple,
        variant: "danger",
        tooltip: "Eliminar backups seleccionados",
      },
    ];

    return {
      default: defaultBtns,
      singleSelect: singleSelectBtns,
      multiSelect: multiSelectBtns,
    };
  });

  // ==========================================
  // HANDLERS - DATA
  // ==========================================
  async function loadBackups() {
    loading = true;
    error = "";
    try {
      const [backupList, backupConfig] = await Promise.all([
        listBackups(),
        getBackupConfig(),
      ]);
      backups = backupList;
      config = backupConfig;

      if (config) {
        configEnabled = config.enabled;
        configHora = config.hora;
        configDiasRetencion = config.diasRetencion;
      }
    } catch (err) {
      console.error("Error loading backups:", err);
      error = String(err);
    }
    loading = false;
  }

  // ==========================================
  // HANDLERS - BACKUP
  // ==========================================
  async function handleBackupNow() {
    const toastId = toast.loading("Creando backup...");
    try {
      const filename = await backupDatabaseAuto();
      toast.success(`Backup creado: ${filename}`, { id: toastId });
      await loadBackups();
    } catch (err) {
      console.error("Error creating backup:", err);
      toast.error(`Error: ${err}`, { id: toastId });
    }
  }

  async function handleBackupManual() {
    try {
      await backupDatabase();
    } catch (err) {
      console.error("Error in manual backup:", err);
    }
  }

  async function handleRestoreFromFile() {
    try {
      await restoreDatabase();
    } catch (err) {
      console.error("Error restoring from file:", err);
    }
  }

  // ==========================================
  // HANDLERS - RESTORE FROM GRID
  // ==========================================
  async function handleRestore(entry: BackupEntry) {
    try {
      await restoreFromAutoBackup(entry.nombre);
    } catch (err) {
      console.error("Error restoring backup:", err);
      await message(`Error al restaurar: ${err}`, {
        title: "Error",
        kind: "error",
      });
    }
  }

  // ==========================================
  // HANDLERS - DELETE
  // ==========================================
  async function handleDelete(entry: BackupEntry) {
    const confirmed = await confirm(
      `¿Eliminar "${entry.nombre}"?\n\nEsta acción no se puede deshacer.`,
      { title: "Confirmar Eliminación", kind: "warning" },
    );
    if (!confirmed) return;

    const toastId = toast.loading("Eliminando...");
    try {
      await deleteBackup(entry.nombre);
      toast.success("Backup eliminado", { id: toastId });
      await loadBackups();
    } catch (err) {
      console.error("Error deleting backup:", err);
      toast.error(`Error: ${err}`, { id: toastId });
    }
  }

  async function handleDeleteMultiple() {
    const confirmed = await confirm(
      `¿Eliminar ${selectedRows.length} backups?\n\nEsta acción no se puede deshacer.`,
      { title: "Confirmar Eliminación", kind: "warning" },
    );
    if (!confirmed) return;

    const toastId = toast.loading("Eliminando...");
    let errors = 0;
    for (const entry of selectedRows) {
      try {
        await deleteBackup(entry.nombre);
      } catch {
        errors++;
      }
    }

    if (errors === 0) {
      toast.success("Backups eliminados", { id: toastId });
    } else {
      toast.error(`${errors} errores`, { id: toastId });
    }
    await loadBackups();
  }

  // ==========================================
  // HANDLERS - CONFIG
  // ==========================================
  async function handleSaveConfig() {
    isSavingConfig = true;
    try {
      config = await updateBackupConfig(
        configEnabled,
        configHora,
        configDiasRetencion,
      );
      toast.success("Configuración guardada");
    } catch (err) {
      console.error("Error saving config:", err);
      toast.error(`Error: ${err}`);
    } finally {
      isSavingConfig = false;
    }
  }

  async function handleCleanup() {
    const confirmed = await confirm(
      `¿Eliminar backups con más de ${configDiasRetencion} días?`,
      { title: "Limpiar Antiguos", kind: "warning" },
    );
    if (!confirmed) return;

    const toastId = toast.loading("Limpiando...");
    try {
      const count = await cleanupOldBackups();
      toast.success(`${count} backup(s) eliminado(s)`, { id: toastId });
      await loadBackups();
    } catch (err) {
      console.error("Error cleaning up:", err);
      toast.error(`Error: ${err}`, { id: toastId });
    }
  }

  // ==========================================
  // LIFECYCLE
  // ==========================================
  onMount(() => {
    loadBackups();
  });
</script>

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
  <!-- Header -->
  <div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
    <div class="flex items-center gap-4">
      <div>
        <h2 class="text-xl font-semibold text-gray-100 flex items-center gap-2">
          <Database class="w-5 h-5 text-purple-500" />
          Copias de Seguridad
        </h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión de backups automáticos y manuales • Usa el botón
          "Configuración" en la toolbar para ajustar auto-backup
        </p>
      </div>
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-hidden relative bg-[#1e1e1e]">
    {#if error}
      <div class="p-6">
        <div
          class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
          transition:fade
        >
          <AlertCircle size={20} />
          <div>
            <div class="font-medium">Error al cargar backups</div>
            <div class="text-sm opacity-90">{error}</div>
          </div>
        </div>
      </div>
    {:else if loading}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <svg
            class="mx-auto h-8 w-8 animate-spin text-purple-500"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            />
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
          <p class="mt-4 text-sm text-gray-400">Cargando backups...</p>
        </div>
      </div>
    {:else if backups.length === 0}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <Database size={48} class="mx-auto text-gray-400" />
          <p class="mt-4 text-lg font-medium text-gray-300">No hay backups</p>
          <p class="mt-2 text-sm text-gray-400">
            Crea tu primer backup usando el botón "Crear Backup"
          </p>
          {#if canUpdate}
            <button
              onclick={handleBackupNow}
              class="mt-4 px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition-colors"
            >
              Crear Backup Ahora
            </button>
          {/if}
        </div>
      </div>
    {:else}
      <AGGridWrapper
        gridId="backup-list"
        {columnDefs}
        rowData={backups}
        {customButtons}
        getRowId={(params) => params.data.ruta}
        persistenceKey="backup-list-columns"
        onSelectionChanged={(rows) => (selectedRows = rows)}
      />
    {/if}
  </div>
</div>
