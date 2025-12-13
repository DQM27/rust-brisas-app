<!-- src/lib/components/settings/ExportSettingsPanel.svelte -->
<script lang="ts">
  import {
    FileText,
    Plus,
    Edit2,
    Trash2,
    Check,
    Save,
    X,
    Copy,
    FileSpreadsheet,
    Table2,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { exportProfileStore } from "$lib/stores/exportProfileStore";
  import type { ExportProfile, PdfDesign } from "$lib/types/exportProfile";
  import { DEFAULT_PDF_DESIGN } from "$lib/types/exportProfile";

  onMount(() => {
    exportProfileStore.load();
  });

  // Profile Management
  let showProfileEditor = $state(false);
  let editingProfile = $state<ExportProfile | null>(null);

  function createProfile() {
    editingProfile = {
      id: crypto.randomUUID(),
      name: "Nuevo Perfil",
      format: "pdf",
      is_default: false,
      title: "Reporte",
      show_preview: false,
      pdf_design: { ...DEFAULT_PDF_DESIGN },
    };
    showProfileEditor = true;
  }

  function editProfile(profile: ExportProfile) {
    editingProfile = JSON.parse(JSON.stringify(profile));
    showProfileEditor = true;
  }

  async function saveProfile() {
    if (!editingProfile) return;
    if (!editingProfile.name.trim()) {
      alert("El nombre del perfil es requerido");
      return;
    }
    const success = await exportProfileStore.save(editingProfile);
    if (success) {
      showProfileEditor = false;
      editingProfile = null;
    }
  }

  async function deleteProfile(id: string) {
    if (confirm("¿Eliminar este perfil?")) {
      await exportProfileStore.delete(id);
    }
  }

  async function duplicateProfile(profile: ExportProfile) {
    const newProfile: ExportProfile = {
      ...JSON.parse(JSON.stringify(profile)),
      id: crypto.randomUUID(),
      name: `${profile.name} (Copia)`,
      is_default: false,
    };
    await exportProfileStore.save(newProfile);
  }

  async function setDefaultProfile(id: string) {
    await exportProfileStore.setDefault(id);
  }

  // Asegurar inicialización correcta según formato
  $effect(() => {
    if (editingProfile) {
      if (editingProfile.format === "pdf" && !editingProfile.pdf_design) {
        editingProfile.pdf_design = { ...DEFAULT_PDF_DESIGN };
        editingProfile.csv_options = undefined;
      } else if (editingProfile.format === "csv") {
        if (!editingProfile.csv_options) {
          editingProfile.csv_options = {
            delimiter: "comma",
            include_bom: true,
          };
        }
        editingProfile.pdf_design = undefined;
      } else if (editingProfile.format === "excel") {
        editingProfile.pdf_design = undefined;
        editingProfile.csv_options = undefined;
      }
    }
  });

  const colorOptions: { key: keyof PdfDesign["colors"]; label: string }[] = [
    { key: "header_fill", label: "Fondo Header" },
    { key: "header_text", label: "Texto Header" },
    { key: "row_text", label: "Texto Filas" },
    { key: "border", label: "Bordes" },
  ];
</script>

<div class="space-y-6">
  <!-- Header -->
  <div
    class="bg-gray-50 dark:bg-[#161b22] rounded-lg border border-gray-200 dark:border-gray-700 p-6"
  >
    <div class="flex items-center gap-3">
      <div class="p-2 bg-[#2da44e]/10 rounded-lg">
        <FileText class="w-6 h-6 text-[#2da44e]" />
      </div>
      <div>
        <h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">
          Perfiles de Exportación
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
          Gestiona perfiles con diseño completo y opciones de exportación
        </p>
      </div>
    </div>
  </div>

  <!-- Profiles List -->
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <p class="text-sm text-gray-500 dark:text-gray-400">
        Cada perfil incluye formato, diseño visual y opciones
      </p>
      <button
        onclick={createProfile}
        class="flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white transition-colors"
      >
        <Plus size={18} />
        Nuevo Perfil
      </button>
    </div>

    <div class="grid gap-3">
      {#if $exportProfileStore.loading}
        <div class="p-8 text-center text-gray-500 dark:text-gray-400 text-sm">
          Cargando perfiles...
        </div>
      {:else if $exportProfileStore.profiles.length === 0}
        <div class="p-8 text-center text-gray-500 dark:text-gray-400 text-sm">
          No hay perfiles. Crea uno para empezar.
        </div>
      {:else}
        {#each $exportProfileStore.profiles as profile}
          <div
            class="p-4 rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-[#0d1117] hover:border-gray-300 dark:hover:border-gray-600 transition-colors"
          >
            <div class="flex items-start justify-between">
              <div class="flex items-start gap-3 flex-1">
                <!-- Icon -->
                <div class="p-2 rounded-md bg-gray-100 dark:bg-[#161b22]">
                  {#if profile.format === "pdf"}
                    <FileText size={20} class="text-red-500" />
                  {:else if profile.format === "excel"}
                    <FileSpreadsheet size={20} class="text-green-500" />
                  {:else}
                    <Table2 size={20} class="text-blue-500" />
                  {/if}
                </div>

                <!-- Info -->
                <div class="flex-1">
                  <div class="flex items-center gap-2">
                    <h3 class="font-medium text-gray-900 dark:text-gray-100">
                      {profile.name}
                    </h3>
                    {#if profile.is_default}
                      <span
                        class="px-2 py-0.5 text-xs rounded-full bg-[#2da44e]/10 text-[#2da44e] font-medium"
                      >
                        Predeterminado
                      </span>
                    {/if}
                  </div>
                  <div
                    class="mt-1 text-sm text-gray-500 dark:text-gray-400 space-y-1"
                  >
                    <div>
                      Formato: <span class="font-medium"
                        >{profile.format.toUpperCase()}</span
                      >
                    </div>
                    {#if profile.format === "pdf" && profile.pdf_design}
                      <div>
                        {profile.pdf_design.orientation === "landscape"
                          ? "Horizontal"
                          : "Vertical"}, {profile.pdf_design.page_size},
                        Márgenes: {profile.pdf_design.margin_x}{profile
                          .pdf_design.margin_x_unit} × {profile.pdf_design
                          .margin_y}{profile.pdf_design.margin_y_unit}
                      </div>
                      <div class="flex items-center gap-2">
                        <div
                          class="w-4 h-4 rounded border"
                          style="background: {profile.pdf_design.colors
                            .header_fill};"
                        ></div>
                        <div
                          class="w-4 h-4 rounded border"
                          style="background: {profile.pdf_design.colors
                            .border};"
                        ></div>
                      </div>
                    {:else if profile.format === "csv" && profile.csv_options}
                      <div>
                        Delimitador: {profile.csv_options.delimiter}
                      </div>
                    {/if}
                  </div>
                </div>
              </div>

              <!-- Actions -->
              <div class="flex items-center gap-1">
                {#if !profile.is_default}
                  <button
                    onclick={() => setDefaultProfile(profile.id)}
                    class="p-2 rounded-md text-gray-400 hover:text-[#2da44e] hover:bg-gray-100 dark:hover:bg-[#161b22] transition-colors"
                    title="Predeterminado"
                  >
                    <Check size={16} />
                  </button>
                {/if}
                <button
                  onclick={() => duplicateProfile(profile)}
                  class="p-2 rounded-md text-gray-400 hover:text-blue-500 hover:bg-gray-100 dark:hover:bg-[#161b22] transition-colors"
                  title="Duplicar"
                >
                  <Copy size={16} />
                </button>
                <button
                  onclick={() => editProfile(profile)}
                  class="p-2 rounded-md text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-[#161b22] transition-colors"
                  title="Editar"
                >
                  <Edit2 size={16} />
                </button>
                <button
                  onclick={() => deleteProfile(profile.id)}
                  class="p-2 rounded-md text-gray-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/10 transition-colors"
                  title="Eliminar"
                >
                  <Trash2 size={16} />
                </button>
              </div>
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </div>
</div>

<!-- Profile Editor Modal -->
{#if showProfileEditor && editingProfile}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    transition:fade
  >
    <div
      class="bg-white dark:bg-[#0d1117] rounded-lg border border-gray-200 dark:border-gray-700 shadow-xl w-full max-w-2xl flex flex-col max-h-[90vh]"
    >
      <!-- Header -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-b border-gray-200 dark:border-gray-700"
      >
        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
          Editar Perfil
        </h3>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6 space-y-5">
        <!-- Nombre y Formato -->
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label
              for="profileName"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              Nombre
            </label>
            <input
              id="profileName"
              type="text"
              bind:value={editingProfile.name}
              class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100"
            />
          </div>

          <div>
            <label
              for="profileFormat"
              class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              Formato
            </label>
            <select
              id="profileFormat"
              bind:value={editingProfile.format}
              class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117]"
            >
              <option value="pdf">PDF</option>
              <option value="excel">Excel</option>
              <option value="csv">CSV</option>
            </select>
          </div>
        </div>

        <!-- PDF Options -->
        {#if editingProfile.format === "pdf" && editingProfile.pdf_design}
          <div class="space-y-4 p-4 bg-gray-50 dark:bg-[#161b22] rounded-lg">
            <h4 class="font-medium">Diseño PDF</h4>

            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="pdfSize" class="block text-sm mb-2">Tamaño</label>
                <select
                  id="pdfSize"
                  bind:value={editingProfile.pdf_design.page_size}
                  class="w-full px-3 py-2 text-sm rounded-md border bg-white dark:bg-[#0d1117]"
                >
                  <option value="us-letter">Carta</option>
                  <option value="a4">A4</option>
                  <option value="legal">Legal</option>
                </select>
              </div>

              <div>
                <label for="pdfOrientation" class="block text-sm mb-2"
                  >Orientación</label
                >
                <select
                  id="pdfOrientation"
                  bind:value={editingProfile.pdf_design.orientation}
                  class="w-full px-3 py-2 text-sm rounded-md border bg-white dark:bg-[#0d1117]"
                >
                  <option value="landscape">Horizontal</option>
                  <option value="portrait">Vertical</option>
                </select>
              </div>
            </div>

            <!-- Márgenes -->
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="pdfMarginX" class="block text-sm mb-2"
                  >Margen X</label
                >
                <div class="flex gap-2">
                  <input
                    id="pdfMarginX"
                    type="number"
                    bind:value={editingProfile.pdf_design.margin_x}
                    class="flex-1 px-3 py-2 text-sm rounded-md border bg-white dark:bg-[#0d1117]"
                  />
                  <select
                    aria-label="Unidad Margen X"
                    bind:value={editingProfile.pdf_design.margin_x_unit}
                    class="px-3 py-2 text-sm rounded-md border bg-white dark:bg-[#0d1117]"
                  >
                    <option value="mm">mm</option>
                    <option value="cm">cm</option>
                    <option value="in">in</option>
                    <option value="pt">pt</option>
                  </select>
                </div>
              </div>

              <div>
                <label for="pdfMarginY" class="block text-sm mb-2"
                  >Margen Y</label
                >
                <div class="flex gap-2">
                  <input
                    id="pdfMarginY"
                    type="number"
                    bind:value={editingProfile.pdf_design.margin_y}
                    class="flex-1 px-3 py-2 text-sm rounded-md border bg-white dark:bg-[#0d1117]"
                  />
                  <select
                    aria-label="Unidad Margen Y"
                    bind:value={editingProfile.pdf_design.margin_y_unit}
                    class="px-3 py-2 text-sm rounded-md border bg-white dark:bg-[#0d1117]"
                  >
                    <option value="mm">mm</option>
                    <option value="cm">cm</option>
                    <option value="in">in</option>
                    <option value="pt">pt</option>
                  </select>
                </div>
              </div>
            </div>

            <!-- Colores -->
            <div class="grid grid-cols-2 gap-3">
              {#each colorOptions as c}
                <div class="flex items-center gap-2">
                  <input
                    type="color"
                    bind:value={editingProfile.pdf_design.colors[c.key]}
                    class="h-8 w-12"
                  />
                  <span class="text-xs">{c.label}</span>
                </div>
              {/each}
            </div>
          </div>
        {:else if editingProfile.format === "csv" && editingProfile.csv_options}
          <div class="space-y-4 p-4 bg-gray-50 dark:bg-[#161b22] rounded-lg">
            <h4 class="font-medium">Opciones CSV</h4>

            <div>
              <label for="csvDelimiter" class="block text-sm mb-2"
                >Delimitador</label
              >
              <select
                id="csvDelimiter"
                bind:value={editingProfile.csv_options.delimiter}
                class="w-full px-3 py-2 text-sm rounded-md border bg-white dark:bg-[#0d1117]"
              >
                <option value="comma">Coma</option>
                <option value="semicolon">Punto y coma</option>
                <option value="tab">Tabulación</option>
                <option value="pipe">Barra</option>
              </select>
            </div>

            <label class="flex items-center gap-2">
              <input
                type="checkbox"
                bind:checked={editingProfile.csv_options.include_bom}
                class="w-4 h-4 rounded"
              />
              <span class="text-sm">UTF-8 BOM</span>
            </label>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-t flex gap-3 justify-end"
      >
        <button
          onclick={() => {
            showProfileEditor = false;
            editingProfile = null;
          }}
          class="px-4 py-2 text-sm rounded-md border hover:bg-gray-100 dark:hover:bg-[#21262d]"
        >
          Cancelar
        </button>
        <button
          onclick={saveProfile}
          class="px-4 py-2 text-sm rounded-md bg-[#2da44e] text-white hover:bg-[#2c974b] flex items-center gap-2"
        >
          <Save size={16} />
          Guardar
        </button>
      </div>
    </div>
  </div>
{/if}
