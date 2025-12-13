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
    Palette,
    Settings2,
    FileSpreadsheet,
    Table2,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { slide, fade } from "svelte/transition";
  import { templateStore } from "$lib/stores/templateStore";
  import { exportProfileStore } from "$lib/stores/exportProfileStore";
  import type { PdfTemplate } from "$lib/types/template";
  import type { ExportProfile } from "$lib/types/exportProfile";
  import TemplateEditor from "../export/TemplateEditor.svelte";

  onMount(() => {
    templateStore.load();
    exportProfileStore.load();
  });

  // Template Management
  let showTemplateEditor = $state(false);
  let editingTemplate = $state<PdfTemplate | null>(null);

  function createTemplate() {
    editingTemplate = {
      id: crypto.randomUUID(),
      name: "Nuevo Estilo",
      is_predefined: false,
      layout: {
        page_size: "us-letter",
        orientation: "landscape",
        margin_x: "2cm",
        margin_y: "2cm",
      },
      colors: {
        header_fill: "#2da44e",
        header_text: "#ffffff",
        row_text: "#000000",
        border: "#cccccc",
      },
      fonts: {
        family: "New Computer Modern",
        size: 10,
        header_size: 12,
      },
    };
    showTemplateEditor = true;
  }

  function editTemplate(template: PdfTemplate) {
    editingTemplate = JSON.parse(JSON.stringify(template));
    showTemplateEditor = true;
  }

  async function saveTemplate(template: PdfTemplate) {
    const success = await templateStore.save(template);
    if (success) {
      showTemplateEditor = false;
      editingTemplate = null;
    }
  }

  async function deleteTemplate(id: string) {
    if (confirm("¿Eliminar este estilo visual?")) {
      await templateStore.delete(id);
    }
  }

  // Export Profile Management
  let showProfileEditor = $state(false);
  let editingProfile = $state<ExportProfile | null>(null);

  function createProfile() {
    editingProfile = {
      id: crypto.randomUUID(),
      name: "Nuevo Perfil",
      format: "pdf",
      is_default: false,
      options: {
        title: "Reporte",
        orientation: "landscape",
        template_id:
          $templateStore.templates.length > 0
            ? $templateStore.templates[0].id
            : undefined,
        show_preview: false,
      },
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
    if (confirm("¿Eliminar este perfil de exportación?")) {
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

  // UI State
  let activeTab = $state<"templates" | "profiles">("profiles");
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
          Configuración de Exportación
        </h2>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
          Administra perfiles de exportación y estilos visuales para PDFs
        </p>
      </div>
    </div>
  </div>

  <!-- Tabs -->
  <div class="flex gap-2 border-b border-gray-200 dark:border-gray-700">
    <button
      onclick={() => (activeTab = "profiles")}
      class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab ===
      'profiles'
        ? 'border-[#2da44e] text-[#2da44e]'
        : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'}"
    >
      <div class="flex items-center gap-2">
        <Settings2 size={16} />
        Perfiles de Exportación
      </div>
    </button>
    <button
      onclick={() => (activeTab = "templates")}
      class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab ===
      'templates'
        ? 'border-[#2da44e] text-[#2da44e]'
        : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'}"
    >
      <div class="flex items-center gap-2">
        <Palette size={16} />
        Estilos Visuales PDF
      </div>
    </button>
  </div>

  <!-- Content -->
  {#if activeTab === "profiles"}
    <!-- Export Profiles -->
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Los perfiles te permiten guardar configuraciones predefinidas de
          exportación
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
          <div
            class="p-8 text-center text-gray-500 dark:text-gray-400 text-sm"
          >
            Cargando perfiles...
          </div>
        {:else if $exportProfileStore.profiles.length === 0}
          <div
            class="p-8 text-center text-gray-500 dark:text-gray-400 text-sm"
          >
            No hay perfiles configurados. Crea uno para empezar.
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
                      <h3
                        class="font-medium text-gray-900 dark:text-gray-100"
                      >
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
                      {#if profile.format === "pdf"}
                        <div>
                          Orientación: <span class="capitalize"
                            >{profile.options.orientation || "landscape"}</span
                          >
                        </div>
                        {#if profile.options.template_id}
                          {@const template = $templateStore.templates.find(
                            (t) => t.id === profile.options.template_id,
                          )}
                          {#if template}
                            <div>Estilo: {template.name}</div>
                          {/if}
                        {/if}
                      {:else if profile.format === "csv"}
                        <div>
                          Delimitador: {profile.options.delimiter || "comma"}
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
                      title="Establecer como predeterminado"
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
  {:else}
    <!-- PDF Templates -->
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <p class="text-sm text-gray-500 dark:text-gray-400">
          Personaliza la apariencia de tus documentos PDF
        </p>
        <button
          onclick={createTemplate}
          class="flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white transition-colors"
        >
          <Plus size={18} />
          Nuevo Estilo
        </button>
      </div>

      <div class="grid gap-3">
        {#if $templateStore.loading}
          <div
            class="p-8 text-center text-gray-500 dark:text-gray-400 text-sm"
          >
            Cargando estilos...
          </div>
        {:else if $templateStore.templates.length === 0}
          <div
            class="p-8 text-center text-gray-500 dark:text-gray-400 text-sm"
          >
            No hay estilos disponibles
          </div>
        {:else}
          {#each $templateStore.templates as template}
            <div
              class="p-4 rounded-lg border border-gray-200 dark:border-gray-700 bg-white dark:bg-[#0d1117] hover:border-gray-300 dark:hover:border-gray-600 transition-colors"
            >
              <div class="flex items-start justify-between">
                <div class="flex items-start gap-3 flex-1">
                  <!-- Color Preview -->
                  <div class="flex gap-1">
                    <div
                      class="w-8 h-8 rounded-md border border-gray-200 dark:border-gray-700 shadow-sm"
                      style="background-color: {template.colors.header_fill};"
                      title="Color de encabezado"
                    ></div>
                    <div
                      class="w-8 h-8 rounded-md border border-gray-200 dark:border-gray-700 shadow-sm"
                      style="background-color: {template.colors.border};"
                      title="Color de bordes"
                    ></div>
                  </div>

                  <!-- Info -->
                  <div class="flex-1">
                    <div class="flex items-center gap-2">
                      <h3
                        class="font-medium text-gray-900 dark:text-gray-100"
                      >
                        {template.name}
                      </h3>
                      {#if template.is_predefined}
                        <span
                          class="px-2 py-0.5 text-xs rounded-full bg-blue-500/10 text-blue-500 font-medium"
                        >
                          Oficial
                        </span>
                      {/if}
                    </div>
                    <div
                      class="mt-1 text-sm text-gray-500 dark:text-gray-400 space-y-0.5"
                    >
                      <div>
                        Tamaño: <span class="font-medium"
                          >{template.layout.page_size}</span
                        >
                      </div>
                      <div>
                        Fuente: <span class="font-medium"
                          >{template.fonts.family}</span
                        > ({template.fonts.size}pt)
                      </div>
                    </div>
                  </div>
                </div>

                <!-- Actions -->
                {#if !template.is_predefined}
                  <div class="flex items-center gap-1">
                    <button
                      onclick={() => editTemplate(template)}
                      class="p-2 rounded-md text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-[#161b22] transition-colors"
                      title="Editar"
                    >
                      <Edit2 size={16} />
                    </button>
                    <button
                      onclick={() => deleteTemplate(template.id)}
                      class="p-2 rounded-md text-gray-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/10 transition-colors"
                      title="Eliminar"
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                {/if}
              </div>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  {/if}
</div>

<!-- Template Editor Modal -->
{#if showTemplateEditor && editingTemplate}
  <TemplateEditor
    template={editingTemplate}
    onSave={saveTemplate}
    onCancel={() => {
      showTemplateEditor = false;
      editingTemplate = null;
    }}
  />
{/if}

<!-- Profile Editor Modal -->
{#if showProfileEditor && editingProfile}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
    transition:fade
  >
    <div
      class="bg-white dark:bg-[#0d1117] rounded-lg border border-gray-200 dark:border-gray-700 shadow-xl w-full max-w-lg flex flex-col max-h-[90vh]"
      transition:fade
    >
      <!-- Header -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-b border-gray-200 dark:border-gray-700"
      >
        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
          {editingProfile.id ? "Editar" : "Nuevo"} Perfil de Exportación
        </h3>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-6 space-y-5">
        <div>
          <label
            for="profile-name"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            Nombre del Perfil
          </label>
          <input
            id="profile-name"
            type="text"
            bind:value={editingProfile.name}
            placeholder="Ej: Reporte Ejecutivo"
            class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
          />
        </div>

        <div>
          <label
            for="profile-format"
            class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            Formato
          </label>
          <select
            id="profile-format"
            bind:value={editingProfile.format}
            class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
          >
            <option value="pdf">PDF</option>
            <option value="excel">Excel</option>
            <option value="csv">CSV</option>
          </select>
        </div>

        {#if editingProfile.format === "pdf"}
          <div class="space-y-4">
            <div>
              <label
                for="profile-orientation"
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
              >
                Orientación
              </label>
              <select
                id="profile-orientation"
                bind:value={editingProfile.options.orientation}
                class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
              >
                <option value="landscape">Horizontal</option>
                <option value="portrait">Vertical</option>
              </select>
            </div>

            <div>
              <label
                for="profile-template"
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
              >
                Estilo Visual
              </label>
              <select
                id="profile-template"
                bind:value={editingProfile.options.template_id}
                class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
              >
                {#each $templateStore.templates as t}
                  <option value={t.id}>{t.name}</option>
                {/each}
              </select>
            </div>
          </div>
        {:else if editingProfile.format === "csv"}
          <div class="space-y-4">
            <div>
              <label
                for="profile-delimiter"
                class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
              >
                Delimitador
              </label>
              <select
                id="profile-delimiter"
                bind:value={editingProfile.options.delimiter}
                class="w-full px-3 py-2 text-sm rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-[#2da44e] focus:border-transparent"
              >
                <option value="comma">Coma (,)</option>
                <option value="semicolon">Punto y coma (;)</option>
                <option value="tab">Tabulación</option>
                <option value="pipe">Barra vertical (|)</option>
              </select>
            </div>

            <label
              class="flex items-center gap-2 cursor-pointer p-2 rounded-md hover:bg-gray-100 dark:hover:bg-[#161b22] transition-colors"
            >
              <input
                type="checkbox"
                bind:checked={editingProfile.options.include_bom}
                class="w-4 h-4 rounded border-gray-300 dark:border-gray-600 text-[#2da44e] focus:ring-[#2da44e]"
              />
              <span class="text-sm text-gray-700 dark:text-gray-300"
                >UTF-8 BOM (para Excel)</span
              >
            </label>
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div
        class="bg-gray-50 dark:bg-[#161b22] px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex items-center justify-end gap-3"
      >
        <button
          onclick={() => {
            showProfileEditor = false;
            editingProfile = null;
          }}
          class="px-4 py-2 text-sm font-medium rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#21262d] text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-[#30363d] transition-colors"
        >
          Cancelar
        </button>
        <button
          onclick={saveProfile}
          class="px-4 py-2 text-sm font-medium rounded-md bg-[#2da44e] hover:bg-[#2c974b] text-white transition-colors flex items-center gap-2"
        >
          <Save size={16} />
          Guardar Perfil
        </button>
      </div>
    </div>
  </div>
{/if}
