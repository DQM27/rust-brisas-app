<!-- src/lib/components/export/TemplateEditor.svelte -->
<script lang="ts">
  import { X, Save, Eye } from "lucide-svelte";
  import { fade, scale } from "svelte/transition";
  import type { PdfTemplate } from "$lib/types/template";

  interface Props {
    template: PdfTemplate;
    onSave: (template: PdfTemplate) => void;
    onCancel: () => void;
  }

  let { template, onSave, onCancel }: Props = $props();

  // Local state copy
  let formData = $state<PdfTemplate>({ ...template });

  function handleSubmit() {
    if (!formData.name.trim()) {
      alert("El nombre es requerido");
      return;
    }
    onSave(formData);
  }
</script>

<div
  class="fixed inset-0 bg-black/60 backdrop-blur-md z-[70] flex items-center justify-center p-4"
  role="presentation"
>
  <div
    class="bg-[#1e1e1e] border border-white/10 rounded-lg shadow-2xl w-full max-w-lg flex flex-col max-h-[90vh]"
    transition:scale={{ duration: 200, start: 0.95 }}
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between p-4 border-b border-white/10 bg-[#252526]"
    >
      <h2 class="text-lg font-semibold text-white">
        {formData.id === template.id ? "Editar Estilo" : "Nuevo Estilo"}
      </h2>
      <button
        onclick={onCancel}
        class="p-1 rounded-md text-gray-400 hover:text-white hover:bg-white/5 transition-colors"
      >
        <X size={20} />
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-5 space-y-6">
      <!-- Basics -->
      <div>
        <label
          for="template-name"
          class="block text-sm font-medium text-gray-400 mb-2"
          >Nombre del Estilo</label
        >
        <input
          id="template-name"
          type="text"
          bind:value={formData.name}
          class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Ej: Reporte Ejecutivo"
        />
      </div>

      <!-- Layout -->
      <div class="space-y-4">
        <h3
          class="text-sm font-medium text-blue-400 uppercase tracking-wider border-b border-white/5 pb-1"
        >
          Diseño de Página
        </h3>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="page-size" class="block text-xs text-gray-400 mb-1"
              >Tamaño</label
            >
            <select
              id="page-size"
              bind:value={formData.layout.page_size}
              class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm"
            >
              <option value="us-letter">Carta (US Letter)</option>
              <option value="a4">A4</option>
              <option value="legal">Legal</option>
            </select>
          </div>
          <div>
            <label for="orientation" class="block text-xs text-gray-400 mb-1"
              >Orientación Default</label
            >
            <select
              id="orientation"
              bind:value={formData.layout.orientation}
              class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm"
            >
              <option value="landscape">Horizontal</option>
              <option value="portrait">Vertical</option>
            </select>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="margin-x" class="block text-xs text-gray-400 mb-1"
              >Margen X</label
            >
            <input
              id="margin-x"
              type="text"
              bind:value={formData.layout.margin_x}
              class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm"
            />
          </div>
          <div>
            <label for="margin-y" class="block text-xs text-gray-400 mb-1"
              >Margen Y</label
            >
            <input
              id="margin-y"
              type="text"
              bind:value={formData.layout.margin_y}
              class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm"
            />
          </div>
        </div>
      </div>

      <!-- Colores -->
      <div class="space-y-4">
        <h3
          class="text-sm font-medium text-pink-400 uppercase tracking-wider border-b border-white/5 pb-1"
        >
          Colores
        </h3>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="header-fill" class="block text-xs text-gray-400 mb-1"
              >Fondo Encabezado</label
            >
            <div class="flex items-center gap-2">
              <input
                id="header-fill"
                type="color"
                bind:value={formData.colors.header_fill}
                class="h-8 w-12 bg-transparent border-none cursor-pointer"
              />
              <span class="text-xs text-gray-500 font-mono"
                >{formData.colors.header_fill}</span
              >
            </div>
          </div>
          <div>
            <label for="header-text" class="block text-xs text-gray-400 mb-1"
              >Texto Encabezado</label
            >
            <div class="flex items-center gap-2">
              <input
                id="header-text"
                type="color"
                bind:value={formData.colors.header_text}
                class="h-8 w-12 bg-transparent border-none cursor-pointer"
              />
              <span class="text-xs text-gray-500 font-mono"
                >{formData.colors.header_text}</span
              >
            </div>
          </div>
          <div>
            <label for="row-text" class="block text-xs text-gray-400 mb-1"
              >Texto Filas</label
            >
            <div class="flex items-center gap-2">
              <input
                id="row-text"
                type="color"
                bind:value={formData.colors.row_text}
                class="h-8 w-12 bg-transparent border-none cursor-pointer"
              />
              <span class="text-xs text-gray-500 font-mono"
                >{formData.colors.row_text}</span
              >
            </div>
          </div>
          <div>
            <label for="border-color" class="block text-xs text-gray-400 mb-1"
              >Bordes</label
            >
            <div class="flex items-center gap-2">
              <input
                id="border-color"
                type="color"
                bind:value={formData.colors.border}
                class="h-8 w-12 bg-transparent border-none cursor-pointer"
              />
              <span class="text-xs text-gray-500 font-mono"
                >{formData.colors.border}</span
              >
            </div>
          </div>
        </div>
      </div>

      <!-- Fuentes -->
      <div class="space-y-4">
        <h3
          class="text-sm font-medium text-yellow-400 uppercase tracking-wider border-b border-white/5 pb-1"
        >
          Tipografía
        </h3>

        <div>
          <label for="font-family" class="block text-xs text-gray-400 mb-1"
            >Familia</label
          >
          <select
            id="font-family"
            bind:value={formData.fonts.family}
            class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm"
          >
            <option value="New Computer Modern"
              >New Computer Modern (Default)</option
            >
            <option value="Arial">Arial</option>
            <option value="Times New Roman">Times New Roman</option>
            <option value="Courier New">Courier New</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="font-size" class="block text-xs text-gray-400 mb-1"
              >Tamaño Texto (pt)</label
            >
            <input
              id="font-size"
              type="number"
              bind:value={formData.fonts.size}
              class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm"
              min="6"
              max="24"
            />
          </div>
          <div>
            <label for="header-size" class="block text-xs text-gray-400 mb-1"
              >Tamaño Header (pt)</label
            >
            <input
              id="header-size"
              type="number"
              bind:value={formData.fonts.header_size}
              class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm"
              min="6"
              max="30"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div
      class="p-4 border-t border-white/10 flex justify-end gap-3 bg-[#252526]"
    >
      <button
        onclick={onCancel}
        class="px-4 py-2 rounded-md text-sm font-medium text-gray-300 hover:text-white hover:bg-white/5 transition-colors"
      >
        Cancelar
      </button>
      <button
        onclick={handleSubmit}
        class="flex items-center gap-2 px-6 py-2 rounded-md text-sm font-medium bg-blue-600 text-white hover:bg-blue-700 transition-colors shadow-lg shadow-blue-500/20"
      >
        <Save size={18} />
        Guardar Estilo
      </button>
    </div>
  </div>
</div>
