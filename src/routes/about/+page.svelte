<script lang="ts">
  import { Check, Mail, Info, Loader2, Link } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "svelte-5-french-toast";

  let subject = "";
  let message = "";
  let contactInfo = "";
  let isSending = false;

  const subjects = [
    "Sugerencia General",
    "Reporte de Error",
    "Mejora de Funcionalidad",
    "Problema de Rendimiento",
    "Otro",
  ];

  async function handleSubmit() {
    if (!message.trim()) {
      toast.error("Por favor escribe un mensaje");
      return;
    }

    if (!subject) {
      toast.error("Por favor selecciona un asunto");
      return;
    }

    isSending = true;
    try {
      await invoke("send_suggestion", {
        subject,
        message,
        contactInfo: contactInfo || undefined,
      });

      toast.success("¡Sugerencia enviada con éxito!");
      message = "";
      subject = "";
      contactInfo = "";
    } catch (error) {
      console.error("Error sending suggestion:", error);
      toast.error(
        "Error al enviar sugerencia. Verifica tu conexión o configuración.",
      );
    } finally {
      isSending = false;
    }
  }
</script>

<div
  class="p-6 max-w-5xl mx-auto space-y-8 font-sans text-gray-900 dark:text-gray-100"
>
  <!-- Header Section -->
  <div class="text-center space-y-4 mb-10">
    <h1 class="text-4xl font-extrabold tracking-tight">Acerca de Brisas App</h1>
    <p class="text-gray-500 dark:text-gray-400 text-lg max-w-2xl mx-auto">
      Gestión integral de accesos y control de contratistas.
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
          Información del Sistema
        </h3>
      </div>

      <!-- Body -->
      <div class="p-4 space-y-4 text-sm">
        <p class="text-gray-600 dark:text-gray-300 leading-relaxed">
          Brisas App es una plataforma robusta construida con tecnologías de
          alto rendimiento para garantizar la eficiencia operativa.
        </p>

        <div
          class="border rounded-md border-gray-200 dark:border-gray-700 divide-y divide-gray-200 dark:divide-gray-700"
        >
          <div class="flex justify-between items-center px-3 py-2">
            <span class="text-gray-500 dark:text-gray-400 font-medium"
              >Versión</span
            >
            <span
              class="bg-gray-100 dark:bg-[#21262d] text-gray-700 dark:text-gray-300 px-2 py-0.5 rounded-full text-xs font-mono border border-gray-200 dark:border-gray-600"
              >v1.2.0</span
            >
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
              >Rust • Tauri • Svelte</span
            >
          </div>
        </div>

        <div class="text-xs text-center text-gray-400 pt-2 italic">
          "Seguridad sin compromisos"
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
          Contactar Soporte
        </h3>
      </div>

      <!-- Body -->
      <div class="p-4">
        <form class="space-y-4" on:submit|preventDefault={handleSubmit}>
          <!-- Subject Dropdown -->
          <div class="space-y-1 relative">
            <label
              for="subject"
              class="block text-xs font-semibold text-gray-900 dark:text-gray-100"
              >Asunto</label
            >

            <button
              type="button"
              class="w-full text-left bg-white dark:bg-[#0d1117] border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 text-sm shadow-sm hover:bg-gray-50 dark:hover:bg-[#161b22] focus:outline-none focus:ring-2 focus:ring-[#0969da] dark:focus:ring-[#58a6ff] focus:border-[#0969da] dark:focus:border-[#58a6ff] transition-all flex justify-between items-center group"
              on:click={() => {
                document
                  .getElementById("gh-dropdown")
                  ?.classList.toggle("hidden");
              }}
            >
              <span
                class={!subject
                  ? "text-gray-500"
                  : "text-gray-900 dark:text-gray-100"}
              >
                {subject || "Selecciona una opción"}
              </span>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-4 w-4 text-gray-500 group-focus:text-[#0969da] dark:group-focus:text-[#58a6ff]"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </button>

            <!-- GitHub Style Menu -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
            <div
              id="gh-dropdown"
              class="hidden absolute right-0 left-0 z-10 mt-1 origin-top-right bg-white dark:bg-[#161b22] border border-gray-200 dark:border-gray-700 rounded-md shadow-lg outline-none max-h-60 overflow-y-auto"
            >
              <div class="py-1">
                {#each subjects as s}
                  <div
                    class="px-4 py-2 text-sm hover:bg-[#0969da] hover:text-white dark:hover:bg-[#1f6feb] dark:hover:text-white cursor-pointer flex items-center group/item transition-colors
                    {subject === s
                      ? 'text-gray-900 dark:text-gray-100 font-semibold bg-gray-50 dark:bg-gray-800'
                      : 'text-gray-700 dark:text-gray-300'}"
                    on:click={() => {
                      subject = s;
                      document
                        .getElementById("gh-dropdown")
                        ?.classList.add("hidden");
                    }}
                  >
                    {#if subject === s}
                      <Check class="w-4 h-4 mr-2" />
                    {:else}
                      <div class="w-4 h-4 mr-2"></div>
                    {/if}
                    {s}
                  </div>
                {/each}
              </div>
            </div>
          </div>

          <!-- Message -->
          <div class="space-y-1">
            <label
              for="message"
              class="block text-xs font-semibold text-gray-900 dark:text-gray-100"
              >Mensaje</label
            >
            <textarea
              id="message"
              bind:value={message}
              rows="5"
              class="w-full bg-white dark:bg-[#0d1117] border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-[#0969da] dark:focus:ring-[#58a6ff] focus:border-[#0969da] dark:focus:border-[#58a6ff] resize-none placeholder-gray-400"
              placeholder="Describe tu sugerencia o problema..."
            ></textarea>
          </div>

          <!-- Contact -->
          <div class="space-y-1">
            <label
              for="contact"
              class="block text-xs font-semibold text-gray-900 dark:text-gray-100"
              >Contacto <span class="text-gray-500 font-normal">(Opcional)</span
              ></label
            >
            <input
              type="text"
              id="contact"
              bind:value={contactInfo}
              class="w-full bg-white dark:bg-[#0d1117] border border-gray-300 dark:border-gray-600 rounded-md px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-[#0969da] dark:focus:ring-[#58a6ff] focus:border-[#0969da] dark:focus:border-[#58a6ff] placeholder-gray-400"
              placeholder="ejemplo@correo.com"
            />
          </div>

          <!-- Submit -->
          <div class="pt-2">
            <button
              type="submit"
              disabled={isSending}
              class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-[#2da44e] hover:bg-[#2c974b] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#2da44e] disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {#if isSending}
                <Loader2 class="animate-spin w-4 h-4 mr-2" />
                Enviando...
              {:else}
                Enviar Comentario
              {/if}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</div>
