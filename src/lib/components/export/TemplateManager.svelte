<!-- src/lib/components/export/TemplateManager.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { X, Plus, Trash2, Edit2, Check } from "lucide-svelte";
  import { fade, scale } from "svelte/transition";
  import { templateStore } from "$lib/stores/templateStore";
  import type { PdfTemplate } from "$lib/types/template";
  import TemplateEditor from "./TemplateEditor.svelte";
  import { DEFAULT_TEMPLATE } from "$lib/types/template";

  interface Props {
    onClose: () => void;
    onSelect: (template: PdfTemplate) => void;
    currentTemplateId?: string;
  }

  let { onClose, onSelect, currentTemplateId }: Props = $props();

  let showEditor = $state(false);
  let editingTemplate = $state<PdfTemplate | null>(null);

  onMount(() => {
    templateStore.load();
  });

  function handleCreate() {
    editingTemplate = {
      ...DEFAULT_TEMPLATE,
      id: crypto.randomUUID(),
      name: "Nuevo Estilo",
    };
    showEditor = true;
  }

  function handleEdit(template: PdfTemplate) {
    // Clone to avoid mutating store directly
    editingTemplate = JSON.parse(JSON.stringify(template));
    showEditor = true;
  }

  async function handleDelete(id: string) {
    if (confirm("¿Estás seguro de eliminar este estilo?")) {
      await templateStore.delete(id);
    }
  }

  function handleSave(template: PdfTemplate) {
    templateStore.save(template).then((success) => {
      if (success) {
        showEditor = false;
        editingTemplate = null;
      }
    });
  }
</script>

<div
  class="fixed inset-0 bg-black/50 backdrop-blur-sm z-[60] flex items-center justify-center p-4"
  role="presentation"
>
  <div
    class="bg-[#1e1e1e] border border-white/10 rounded-lg shadow-2xl w-full max-w-2xl flex flex-col max-h-[80vh]"
    transition:scale={{ duration: 200, start: 0.95 }}
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-white/10">
      <div>
        <h2 class="text-lg font-semibold text-white">
          Administrar Estilos PDF
        </h2>
        <p class="text-xs text-gray-400 mt-0.5">
          Selecciona o crea un estilo personalizado
        </p>
      </div>
      <button
        onclick={onClose}
        class="p-1.5 rounded-md text-gray-400 hover:text-white hover:bg-white/5 transition-colors"
      >
        <X size={20} />
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-4 space-y-3">
      {#if $templateStore.loading}
        <div class="text-center py-8 text-gray-400">Cargando estilos...</div>
      {:else if $templateStore.templates.length === 0}
        <div class="text-center py-8 text-gray-400">
          No hay estilos disponibles
        </div>
      {:else}
        {#each $templateStore.templates as template}
          <div
            class="flex items-center justify-between p-3 rounded-lg border border-white/10 bg-[#252526] hover:border-white/20 transition-colors"
          >
            <div class="flex items-center gap-3">
              <!-- Preview de colores -->
              <div class="flex gap-1">
                <div
                  class="w-4 h-4 rounded-full border border-white/20 shadow-sm"
                  style="background-color: {template.colors.header_fill};"
                ></div>
                <div
                  class="w-4 h-4 rounded-full border border-white/20 shadow-sm"
                  style="background-color: {template.colors.header_text};"
                ></div>
              </div>

              <div>
                <h3
                  class="text-white font-medium text-sm flex items-center gap-2"
                >
                  {template.name}
                  {#if template.is_predefined}
                    <span
                      class="px-1.5 py-0.5 rounded text-[10px] bg-blue-500/20 text-blue-400 font-medium"
                      >Oficial</span
                    >
                  {/if}
                </h3>
                <p class="text-xs text-gray-400">
                  {template.layout.page_size} • {template.fonts.family}
                </p>
              </div>
            </div>

            <div class="flex items-center gap-2">
              {#if currentTemplateId === template.id}
                <span
                  class="text-green-500 flex items-center gap-1 text-xs font-medium mr-2"
                >
                  <Check size={14} /> Activo
                </span>
              {/if}

              <button
                onclick={() => onSelect(template)}
                class="px-3 py-1.5 rounded-md text-xs font-medium bg-blue-600 text-white hover:bg-blue-700 transition-colors"
              >
                Seleccionar
              </button>

              {#if !template.is_predefined}
                <button
                  onclick={() => handleEdit(template)}
                  class="p-2 rounded-md text-gray-400 hover:text-white hover:bg-white/5 transition-colors"
                  title="Editar"
                >
                  <Edit2 size={16} />
                </button>
                <button
                  onclick={() => handleDelete(template.id)}
                  class="p-2 rounded-md text-gray-400 hover:text-red-400 hover:bg-red-500/10 transition-colors"
                  title="Eliminar"
                >
                  <Trash2 size={16} />
                </button>
              {/if}
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <!-- Footer -->
    <div
      class="p-4 border-t border-white/10 flex justify-between bg-[#252526] rounded-b-lg"
    >
      <div class="text-xs text-gray-500 flex items-center">
        * Los estilos oficiales no se pueden eliminar
      </div>
      <button
        onclick={handleCreate}
        class="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium bg-green-600 text-white hover:bg-green-700 transition-colors"
      >
        <Plus size={18} />
        Crear Nuevo Estilo
      </button>
    </div>
  </div>
</div>

{#if showEditor && editingTemplate}
  <TemplateEditor
    template={editingTemplate}
    onSave={handleSave}
    onCancel={() => {
      showEditor = false;
      editingTemplate = null;
    }}
  />
{/if}
