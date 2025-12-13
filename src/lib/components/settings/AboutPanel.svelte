<script lang="ts">
  import {
    Check,
    Mail,
    Info,
    Loader2,
    Paperclip,
    X,
    Bug,
    Lightbulb,
    Sparkles,
    AlertCircle,
  } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "svelte-5-french-toast";

  interface Props {
    tabId?: string;
    data?: Record<string, unknown>;
  }

  let { tabId, data }: Props = $props();

  interface ReporteResponse {
    id: string;
    tipo: string;
    asunto: string;
    mensaje: string;
    estado: string;
    errorEnvio: string | null;
  }

  let selectedTipo = $state<string>("");
  let subject = $state("");
  let message = $state("");
  let contactInfo = $state("");
  let isSending = $state(false);
  let lastResult = $state<ReporteResponse | null>(null);

  // Attachment state
  let attachmentFile: File | null = $state(null);
  let attachmentName = $state("");
  let attachmentBase64 = $state("");

  const tipoOptions = [
    {
      value: "sugerencia",
      label: "Sugerencia",
      icon: Lightbulb,
      color: "text-blue-500",
      bg: "bg-blue-500/10 border-blue-500/20 hover:bg-blue-500/20",
    },
    {
      value: "error",
      label: "Reporte de Error",
      icon: Bug,
      color: "text-red-500",
      bg: "bg-red-500/10 border-red-500/20 hover:bg-red-500/20",
    },
    {
      value: "mejora",
      label: "Mejora",
      icon: Sparkles,
      color: "text-purple-500",
      bg: "bg-purple-500/10 border-purple-500/20 hover:bg-purple-500/20",
    },
  ];

  function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      const file = input.files[0];
      if (file.size > 5 * 1024 * 1024) {
        toast.error("El archivo es demasiado grande (Max 5MB)");
        return;
      }

      attachmentFile = file;
      attachmentName = file.name;

      const reader = new FileReader();
      reader.onload = (e) => {
        const result = e.target?.result as string;
        attachmentBase64 = result.split(",")[1];
      };
      reader.readAsDataURL(file);
    }
  }

  function clearFile() {
    attachmentFile = null;
    attachmentName = "";
    attachmentBase64 = "";
    const input = document.getElementById(
      "about-file-upload",
    ) as HTMLInputElement;
    if (input) input.value = "";
  }

  async function handleSubmit() {
    if (!message.trim()) {
      toast.error("Por favor escribe un mensaje");
      return;
    }

    if (!selectedTipo) {
      toast.error("Por favor selecciona un tipo de reporte");
      return;
    }

    isSending = true;
    lastResult = null;

    try {
      const result = await invoke<ReporteResponse>("create_reporte", {
        input: {
          tipo: selectedTipo,
          asunto: subject || getTipoLabel(selectedTipo),
          mensaje: message,
          contacto: contactInfo || null,
          adjuntoBase64: attachmentBase64 || null,
          nombreAdjunto: attachmentName || null,
        },
      });

      lastResult = result;
      toast.success("Reporte enviado exitosamente");

      // Reset form
      message = "";
      subject = "";
      contactInfo = "";
      selectedTipo = "";
      clearFile();
    } catch (error: any) {
      console.error("Error sending report:", error);

      if (error?.toString().includes("ID:")) {
        toast.error("Reporte guardado pero hubo un error al enviar");
      } else {
        toast.error("Error al enviar reporte");
      }
    } finally {
      isSending = false;
    }
  }

  function getTipoLabel(tipo: string): string {
    return tipoOptions.find((t) => t.value === tipo)?.label || tipo;
  }
</script>

<div class="h-full overflow-auto bg-surface-1">
  <div
    class="p-6 max-w-5xl mx-auto space-y-8 font-sans text-gray-900 dark:text-gray-100"
  >
    <!-- Header Section -->
    <div class="text-center space-y-4 mb-10">
      <h1 class="text-4xl font-extrabold tracking-tight">
        Acerca de Brisas App
      </h1>
      <p class="text-gray-500 dark:text-gray-400 text-lg max-w-2xl mx-auto">
        Gestion integral de accesos y control de contratistas.
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 items-start">
      <!-- Info Card (GitHub Box Style) -->
      <div
        class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] shadow-sm overflow-hidden"
      >
        <!-- Header -->
        <div
          class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2"
        >
          <Info class="w-4 h-4 text-gray-500" />
          <h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
            Informacion del Sistema
          </h3>
        </div>

        <!-- Body -->
        <div class="p-4 space-y-4 text-sm">
          <p class="text-gray-600 dark:text-gray-300 leading-relaxed">
            Brisas App es una plataforma robusta construida con tecnologias de
            alto rendimiento para garantizar la eficiencia operativa.
          </p>

          <div
            class="border rounded-md border-gray-200 dark:border-gray-700 divide-y divide-gray-200 dark:divide-gray-700"
          >
            <div class="flex justify-between items-center px-3 py-2">
              <span class="text-gray-500 dark:text-gray-400 font-medium"
                >Version</span
              >
              <div class="flex items-center gap-2">
                <span
                  class="bg-gray-100 dark:bg-[#21262d] text-gray-700 dark:text-gray-300 px-2 py-0.5 rounded-full text-xs font-mono border border-gray-200 dark:border-gray-600"
                  >v1.2.0</span
                >
              </div>
            </div>
            <div class="flex justify-between items-center px-3 py-2">
              <span class="text-gray-500 dark:text-gray-400 font-medium"
                >Desarrollador</span
              >
              <span class="text-gray-900 dark:text-gray-100 font-semibold"
                >Fempro Brisas</span
              >
            </div>
            <div class="flex justify-between items-center px-3 py-2">
              <span class="text-gray-500 dark:text-gray-400 font-medium"
                >Stack</span
              >
              <span class="text-[#0969da] dark:text-[#58a6ff]"
                >Rust - Tauri - Svelte</span
              >
            </div>
          </div>

          <div
            class="bg-blue-50 dark:bg-blue-900/20 text-blue-800 dark:text-blue-300 p-3 rounded-md text-xs border border-blue-200 dark:border-blue-800/30"
          >
            <strong>Novedades?</strong> Las notas de actualizacion detalladas se
            encuentran en la seccion de actualizaciones del sistema.
          </div>
        </div>
      </div>

      <!-- Feedback Form Card (GitHub Box Style) -->
      <div
        class="rounded-md border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#0d1117] shadow-sm"
      >
        <!-- Header -->
        <div
          class="bg-gray-50 dark:bg-[#161b22] px-4 py-3 border-b border-gray-200 dark:border-gray-700 flex items-center gap-2"
        >
          <Mail class="w-4 h-4 text-gray-500" />
          <h3 class="font-semibold text-sm text-gray-900 dark:text-gray-100">
            Enviar Reporte
          </h3>
        </div>

        <!-- Body -->
        <div class="p-4">
          <form
            class="space-y-4"
            onsubmit={(e) => {
              e.preventDefault();
              handleSubmit();
            }}
          >
            <!-- Tipo Selector (Card Style) -->
            <div class="space-y-2">
              <span
                class="block text-xs font-semibold text-gray-900 dark:text-gray-100"
              >
                Tipo de Reporte
              </span>
              <div class="grid grid-cols-3 gap-2">
                {#each tipoOptions as tipo}
                  {@const Icon = tipo.icon}
                  <button
                    type="button"
                    onclick={() => (selectedTipo = tipo.value)}
                    class="flex flex-col items-center gap-1.5 p-3 rounded-md border transition-all text-center
                      {selectedTipo === tipo.value
                      ? `${tipo.bg} border-2 ${tipo.color}`
                      : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-[#161b22]'}"
                  >
                    <Icon
                      class="w-5 h-5 {selectedTipo === tipo.value
                        ? tipo.color
                        : 'text-gray-400'}"
                    />
                    <span
                      class="text-xs font-medium {selectedTipo === tipo.value
                        ? tipo.color
                        : 'text-gray-600 dark:text-gray-400'}"
                    >
                      {tipo.label}
                    </span>
                  </button>
                {/each}
              </div>
            </div>

            <!-- Subject (Optional) -->
            <div class="space-y-1">
              <label
                for="about-subject"
                class="block text-xs font-semibold text-gray-900 dark:text-gray-100"
              >
                Asunto <span class="text-gray-500 font-normal">(Opcional)</span>
              </label>
              <input
                type="text"
                id="about-subject"
                bind:value={subject}
                class="w-full bg-white dark:bg-[#0d1117] border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-[#0969da] dark:focus:ring-[#58a6ff] focus:border-[#0969da] dark:focus:border-[#58a6ff] placeholder-gray-400"
                placeholder="Breve descripcion del reporte"
              />
            </div>

            <!-- Message -->
            <div class="space-y-1">
              <label
                for="about-message"
                class="block text-xs font-semibold text-gray-900 dark:text-gray-100"
                >Mensaje <span class="text-red-500">*</span></label
              >
              <textarea
                id="about-message"
                bind:value={message}
                rows={5}
                class="w-full bg-white dark:bg-[#0d1117] border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-[#0969da] dark:focus:ring-[#58a6ff] focus:border-[#0969da] dark:focus:border-[#58a6ff] resize-none placeholder-gray-400"
                placeholder="Describe tu sugerencia, error o mejora..."
              ></textarea>
            </div>

            <!-- Attachments -->
            <div class="space-y-1">
              <span
                class="block text-xs font-semibold text-gray-900 dark:text-gray-100"
                >Adjuntar Imagen <span class="text-gray-500 font-normal"
                  >(Opcional)</span
                ></span
              >
              <div class="flex items-center gap-2">
                <label
                  for="about-file-upload"
                  class="cursor-pointer inline-flex items-center px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm text-xs font-medium text-gray-700 dark:text-gray-200 bg-white dark:bg-[#161b22] hover:bg-gray-50 dark:hover:bg-[#21262d] transition-colors"
                >
                  <Paperclip class="w-3.5 h-3.5 mr-2 text-gray-500" />
                  Seleccionar archivo
                  <input
                    id="about-file-upload"
                    type="file"
                    class="hidden"
                    accept="image/*"
                    onchange={handleFileSelect}
                  />
                </label>
                {#if attachmentName}
                  <div
                    class="flex items-center gap-1 bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded text-xs text-gray-600 dark:text-gray-300 border border-gray-200 dark:border-gray-700"
                  >
                    <span class="truncate max-w-[150px]">{attachmentName}</span>
                    <button
                      type="button"
                      onclick={clearFile}
                      class="hover:text-red-500"><X class="w-3 h-3" /></button
                    >
                  </div>
                {/if}
              </div>
            </div>

            <!-- Contact -->
            <div class="space-y-1">
              <label
                for="about-contact"
                class="block text-xs font-semibold text-gray-900 dark:text-gray-100"
                >Contacto <span class="text-gray-500 font-normal"
                  >(Opcional)</span
                ></label
              >
              <input
                type="text"
                id="about-contact"
                bind:value={contactInfo}
                class="w-full bg-white dark:bg-[#0d1117] border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-[#0969da] dark:focus:ring-[#58a6ff] focus:border-[#0969da] dark:focus:border-[#58a6ff] placeholder-gray-400"
                placeholder="ejemplo@correo.com"
              />
            </div>

            <!-- Submit -->
            <div class="pt-2">
              <button
                type="submit"
                disabled={isSending || !selectedTipo}
                class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-[#2da44e] hover:bg-[#2c974b] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#2da44e] disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                {#if isSending}
                  <Loader2 class="animate-spin w-4 h-4 mr-2" />
                  Enviando...
                {:else}
                  Enviar Reporte
                {/if}
              </button>
            </div>
          </form>

          <!-- Success/Error Message -->
          {#if lastResult}
            <div
              class="mt-4 p-3 rounded-md border {lastResult.estado === 'enviado'
                ? 'bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800/30'
                : 'bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-800/30'}"
            >
              <div class="flex items-start gap-2">
                {#if lastResult.estado === "enviado"}
                  <Check class="w-4 h-4 text-green-600 mt-0.5" />
                  <div class="text-sm text-green-800 dark:text-green-300">
                    <strong>Reporte enviado</strong>
                    <p class="text-xs mt-1 opacity-80">
                      Tu reporte ha sido enviado exitosamente.
                    </p>
                  </div>
                {:else}
                  <AlertCircle class="w-4 h-4 text-yellow-600 mt-0.5" />
                  <div class="text-sm text-yellow-800 dark:text-yellow-300">
                    <strong>Reporte guardado</strong>
                    <p class="text-xs mt-1 opacity-80">
                      El reporte fue guardado pero no se pudo enviar.
                    </p>
                  </div>
                {/if}
              </div>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
</div>
