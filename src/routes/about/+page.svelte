<script lang="ts">
  import { Check, Mail, AlertTriangle, Loader2 } from "lucide-svelte";
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

<div class="p-6 max-w-4xl mx-auto space-y-8">
  <!-- Header Section -->
  <div class="text-center space-y-4">
    <h1
      class="text-4xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent"
    >
      Acerca de Brisas App
    </h1>
    <p class="text-surface-content/70 max-w-2xl mx-auto text-lg">
      Tu herramienta integral para la gestión y control de accesos.
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
    <!-- Info Card -->
    <div
      class="h-full border border-surface-200/50 dark:border-surface-700/50 shadow-lg backdrop-blur-sm bg-surface-100/50 dark:bg-surface-800/20 rounded-xl overflow-hidden"
    >
      <div class="p-6 space-y-6">
        <div class="flex items-center gap-3 mb-4">
          <div class="p-3 rounded-xl bg-primary/10 text-primary">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="lucide lucide-info"
            >
              <circle cx="12" cy="12" r="10" />
              <line x1="12" x2="12" y1="16" y2="12" />
              <line x1="12" x2="12.01" y1="8" y2="8" />
            </svg>
          </div>
          <h2
            class="text-2xl font-semibold text-surface-900 dark:text-surface-50"
          >
            Información
          </h2>
        </div>

        <div class="space-y-4 text-surface-content/80">
          <p>
            Brisas App es una solución moderna diseñada para optimizar el
            control de acceso y la seguridad en el residencial.
          </p>

          <div
            class="py-4 border-t border-b border-surface-200 dark:border-surface-700"
          >
            <div class="flex justify-between items-center py-2">
              <span class="font-medium">Versión</span>
              <span
                class="px-2 py-1 rounded-md bg-surface-200 dark:bg-surface-700 text-sm font-mono"
                >v1.2.0</span
              >
            </div>
            <div class="flex justify-between items-center py-2">
              <span class="font-medium">Desarrollado por</span>
              <span>Fempro Brisas</span>
            </div>
            <div class="flex justify-between items-center py-2">
              <span class="font-medium">Tecnología</span>
              <span class="text-accent">Rust + Svelte + Tauri</span>
            </div>
          </div>

          <p class="text-sm italic opacity-70">
            "Seguridad y eficiencia en un solo lugar."
          </p>
        </div>
      </div>
    </div>

    <!-- Feedback Form Card -->
    <div
      class="h-full border border-surface-200/50 dark:border-surface-700/50 shadow-lg backdrop-blur-sm bg-surface-100/50 dark:bg-surface-800/20 rounded-xl"
    >
      <div class="p-6 space-y-6">
        <div class="flex items-center gap-3 mb-4">
          <div class="p-3 rounded-xl bg-accent/10 text-accent">
            <Mail />
          </div>
          <h2
            class="text-2xl font-semibold text-surface-900 dark:text-surface-50"
          >
            Buzón de Sugerencias
          </h2>
        </div>

        <p class="text-sm text-surface-content/70 mb-4">
          Tu opinión es importante. Envíanos tus sugerencias, reportes de
          errores o comentarios de forma anónima o con contacto.
        </p>

        <form class="space-y-4" on:submit|preventDefault={handleSubmit}>
          <!-- Subject -->
          <div class="space-y-2 relative">
            <label
              for="subject"
              class="text-sm font-medium text-surface-700 dark:text-surface-300"
              >Asunto</label
            >

            <button
              type="button"
              class="w-full px-4 py-2.5 rounded-lg border border-surface-200 dark:border-surface-700 bg-surface-50 dark:bg-surface-900 text-left text-sm flex justify-between items-center focus:ring-2 focus:ring-primary/50 outline-none transition-all group"
              on:click={() => {
                const el = document.getElementById("subject-dropdown");
                if (el) {
                  el.classList.toggle("hidden");
                  // Optional: handle overlay
                }
              }}
            >
              <span
                class={!subject
                  ? "text-surface-400"
                  : "text-surface-900 dark:text-surface-50"}
              >
                {subject || "Selecciona un asunto..."}
              </span>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="lucide lucide-chevron-down text-surface-400 transition-transform group-focus:rotate-180"
              >
                <path d="m6 9 6 6 6-6" />
              </svg>
            </button>

            <!-- Dropdown Menu -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
            <div
              id="subject-dropdown"
              class="hidden absolute z-50 mt-1 w-full rounded-lg border border-surface-200 dark:border-surface-700 bg-surface-50 dark:bg-surface-900 shadow-xl overflow-hidden animate-in fade-in zoom-in-95 duration-100"
            >
              <div class="py-1 max-h-60 overflow-auto custom-scrollbar">
                {#each subjects as s}
                  <div
                    class="px-4 py-2.5 text-sm cursor-pointer hover:bg-primary/10 hover:text-primary transition-colors flex items-center justify-between
                      {subject === s
                      ? 'text-primary font-medium bg-primary/5'
                      : 'text-surface-600 dark:text-surface-300'}"
                    on:click|stopPropagation={() => {
                      subject = s;
                      document
                        .getElementById("subject-dropdown")
                        ?.classList.add("hidden");
                    }}
                    role="option"
                    aria-selected={subject === s}
                  >
                    {s}
                    {#if subject === s}
                      <Check class="w-4 h-4" />
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          </div>

          <!-- Message -->
          <div class="space-y-2">
            <label
              for="message"
              class="text-sm font-medium text-surface-700 dark:text-surface-300"
              >Mensaje</label
            >
            <textarea
              id="message"
              bind:value={message}
              rows="4"
              placeholder="Escribe tu mensaje aquí..."
              class="w-full px-4 py-2 rounded-lg border border-surface-200 dark:border-surface-700 bg-surface-50 dark:bg-surface-900 focus:ring-2 focus:ring-primary/50 outline-none transition-all resize-none"
            ></textarea>
          </div>

          <!-- Contact Info (Optional) -->
          <div class="space-y-2">
            <label
              for="contact"
              class="text-sm font-medium text-surface-700 dark:text-surface-300"
            >
              Contacto (Opcional)
            </label>
            <input
              type="text"
              id="contact"
              bind:value={contactInfo}
              placeholder="Email o teléfono (si deseas respuesta)"
              class="w-full px-4 py-2 rounded-lg border border-surface-200 dark:border-surface-700 bg-surface-50 dark:bg-surface-900 focus:ring-2 focus:ring-primary/50 outline-none transition-all"
            />
          </div>

          <div class="pt-2">
            <button
              type="submit"
              class="w-full py-2.5 font-medium flex justify-center items-center gap-2 group bg-primary text-black dark:text-white rounded-lg hover:brightness-110 transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-md"
              disabled={isSending}
            >
              {#if isSending}
                <Loader2 class="animate-spin w-5 h-5" />
                <span>Enviando...</span>
              {:else}
                <span>Enviar Sugerencia</span>
                <Check
                  class="w-5 h-5 opacity-0 group-hover:opacity-100 transition-opacity"
                />
              {/if}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</div>
