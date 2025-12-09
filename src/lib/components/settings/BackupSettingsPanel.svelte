<script lang="ts">
  import { Database, ArchiveRestore, Save, Upload } from "lucide-svelte";
  import { scale } from "svelte/transition";
  import { backupDatabase, restoreDatabase } from "$lib/services/backupService";
</script>

<div
  class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
  in:scale={{ duration: 300, start: 0.95 }}
>
  <div class="mb-6">
    <h2 class="text-2xl font-bold text-primary">Copia de Seguridad</h2>
    <p class="text-secondary mt-1">
      Protege tus datos realizando copias periódicas.
    </p>
  </div>

  <div class="grid gap-4 max-w-3xl pb-8">
    <!-- ================================================================== -->
    <!-- BACKUP CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5 border-l-4 border-l-blue-500">
      <div class="flex items-center gap-4 mb-4">
        <div
          class="p-3 rounded-lg bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400"
        >
          <Save size={22} />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-primary">Crear Backup</h3>
          <p class="text-sm text-secondary">
            Exporta toda la base de datos a un archivo local.
          </p>
        </div>
      </div>

      <div class="flex flex-col gap-3">
        <p class="text-sm text-secondary mb-2">
          Se generará un archivo <code>.db</code> que contiene todos los registros
          actuales (usuarios, contratistas, historial). Este proceso no interrumpe
          el uso de la aplicación.
        </p>
        <button
          class="btn-primary w-fit flex items-center gap-2"
          onclick={() => backupDatabase()}
        >
          <Database size={18} />
          Generar Copia de Seguridad
        </button>
      </div>
    </div>

    <!-- ================================================================== -->
    <!-- RESTORE CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5 border-l-4 border-l-red-500">
      <div class="flex items-center gap-4 mb-4">
        <div
          class="p-3 rounded-lg bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400"
        >
          <Upload size={22} />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-primary">Restaurar Datos</h3>
          <p class="text-sm text-secondary">
            Reemplaza la base de datos actual con un backup.
          </p>
        </div>
      </div>

      <div
        class="bg-red-50 dark:bg-red-900/20 p-4 rounded-lg mb-4 text-sm text-red-700 dark:text-red-300"
      >
        <strong>⚠️ Advertencia:</strong> Esta acción borrará todos los datos actuales
        y los reemplazará por los del archivo seleccionado. La aplicación se reiniciará
        automáticamente.
      </div>

      <div class="flex flex-col gap-3">
        <button
          class="btn-base bg-surface-2 hover:bg-surface-3 text-primary w-fit flex items-center gap-2 border border-emphasis"
          onclick={() => restoreDatabase()}
        >
          <ArchiveRestore size={18} />
          Seleccionar archivo para restaurar
        </button>
      </div>
    </div>
  </div>
</div>
