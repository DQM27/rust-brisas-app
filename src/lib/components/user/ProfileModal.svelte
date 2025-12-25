<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import {
    X,
    User,
    Mail,
    Phone,
    MapPin,
    Calendar,
    CreditCard,
    Briefcase,
    Shield,
    PenSquare,
  } from "lucide-svelte";
  import type { UserResponse } from "$lib/types/user";

  interface Props {
    show: boolean;
    user: UserResponse | null;
    onClose: () => void;
    onEdit: () => void;
  }

  let { show, user, onClose, onEdit }: Props = $props();

  function formatDate(dateString?: string | null) {
    if (!dateString) return "N/A";
    return new Date(dateString).toLocaleDateString("es-ES", {
      year: "numeric",
      month: "long",
      day: "numeric",
    });
  }

  const userInitials = $derived(
    user
      ? `${user.nombre?.[0] || ""}${user.apellido?.[0] || ""}`.toUpperCase()
      : "",
  );

  const fullName = $derived(
    user ? `${user.nombre || ""} ${user.apellido || ""}`.trim() : "",
  );
</script>

{#if show && user}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-black/50 dark:bg-black/70 backdrop-blur-sm z-[9999] flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
    onclick={onClose}
    role="dialog"
    aria-modal="true"
  >
    <!-- Modal Container -->
    <div
      class="bg-white dark:bg-[#0d1117] w-full max-w-2xl rounded-xl shadow-2xl flex flex-col max-h-[90vh] border border-gray-200 dark:border-gray-800"
      transition:fly={{ y: 20, duration: 300 }}
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Header with Cover & Avatar -->
      <div class="relative">
        <div
          class="h-32 bg-gray-100 dark:bg-[#161b22] rounded-t-xl overflow-hidden border-b border-gray-200 dark:border-gray-800"
        >
          <!-- Abstract Pattern or Gradient could go here -->
          <div
            class="w-full h-full opacity-10 bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0IiBoZWlnaHQ9IjQiPgo8cmVjdCB3aWR0aD0iNCIgaGVpZ2h0PSI0IiBmaWxsPSIjZmZmIi8+CjxyZWN0IHdpZHRoPSIxIiBoZWlnaHQ9IjEiIGZpbGw9IiMwMDAiLz4KPC9zdmc+')]"
          ></div>
        </div>

        <button
          onclick={onClose}
          class="absolute top-4 right-4 p-2 rounded-full bg-black/20 hover:bg-black/40 text-white transition-colors backdrop-blur-md"
        >
          <X size={20} />
        </button>

        <div class="absolute -bottom-12 left-8">
          <div
            class="w-24 h-24 rounded-full border-4 border-white dark:border-[#0d1117] bg-blue-500 flex items-center justify-center text-3xl font-bold text-white shadow-md"
          >
            {userInitials}
          </div>
        </div>

        <div class="absolute bottom-2 right-4">
          <button
            onclick={onEdit}
            class="flex items-center gap-2 px-4 py-2 bg-white dark:bg-[#21262d] hover:bg-gray-50 dark:hover:bg-[#30363d] text-sm font-medium text-gray-700 dark:text-gray-200 border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm transition-all"
          >
            <PenSquare size={16} />
            Editar Perfil
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="pt-16 px-8 pb-8 overflow-y-auto">
        <!-- Main Info -->
        <div class="mb-6">
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
            {fullName}
          </h1>
          <div
            class="flex items-center gap-2 text-gray-500 dark:text-gray-400 mt-1"
          >
            <span class="inline-flex items-center gap-1">
              <Shield size={14} />
              {user.roleName || "N/A"}
            </span>
            <span>•</span>
            <span>{user.email}</span>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <!-- Section: Personal Info -->
          <section>
            <h3
              class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-4 pb-2 border-b border-gray-100 dark:border-gray-800"
            >
              Información Personal
            </h3>
            <dl class="space-y-4">
              <div class="group">
                <dt
                  class="flex items-center gap-2 text-sm font-medium text-gray-500 dark:text-gray-400 mb-1"
                >
                  <CreditCard size={14} /> Cédula
                </dt>
                <dd class="text-sm text-gray-900 dark:text-gray-200 ml-6">
                  {user.cedula}
                </dd>
              </div>
              <div>
                <dt
                  class="flex items-center gap-2 text-sm font-medium text-gray-500 dark:text-gray-400 mb-1"
                >
                  <Calendar size={14} /> Fecha Nacimiento
                </dt>
                <dd class="text-sm text-gray-900 dark:text-gray-200 ml-6">
                  {formatDate(user.fechaNacimiento)}
                </dd>
              </div>
            </dl>
          </section>

          <!-- Section: Contact Info -->
          <section>
            <h3
              class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-4 pb-2 border-b border-gray-100 dark:border-gray-800"
            >
              Contacto
            </h3>
            <dl class="space-y-4">
              <div>
                <dt
                  class="flex items-center gap-2 text-sm font-medium text-gray-500 dark:text-gray-400 mb-1"
                >
                  <Phone size={14} /> Teléfono
                </dt>
                <dd class="text-sm text-gray-900 dark:text-gray-200 ml-6">
                  {user.telefono || "No registrado"}
                </dd>
              </div>
              <div>
                <dt
                  class="flex items-center gap-2 text-sm font-medium text-gray-500 dark:text-gray-400 mb-1"
                >
                  <MapPin size={14} /> Dirección
                </dt>
                <dd class="text-sm text-gray-900 dark:text-gray-200 ml-6">
                  {user.direccion || "No registrada"}
                </dd>
              </div>
            </dl>
          </section>

          <!-- Section: Work Info -->
          <section>
            <h3
              class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-4 pb-2 border-b border-gray-100 dark:border-gray-800"
            >
              Información Laboral
            </h3>
            <dl class="space-y-4">
              <div>
                <dt
                  class="flex items-center gap-2 text-sm font-medium text-gray-500 dark:text-gray-400 mb-1"
                >
                  <Briefcase size={14} /> Inicio Labores
                </dt>
                <dd class="text-sm text-gray-900 dark:text-gray-200 ml-6">
                  {formatDate(user.fechaInicioLabores)}
                </dd>
              </div>
              <div>
                <dt
                  class="flex items-center gap-2 text-sm font-medium text-gray-500 dark:text-gray-400 mb-1"
                >
                  <Shield size={14} /> Gafete
                </dt>
                <dd
                  class="text-sm text-gray-900 dark:text-gray-200 ml-6 group-hover:text-blue-500 transition-colors"
                >
                  {user.numeroGafete || "No asignado"}
                </dd>
              </div>
            </dl>
          </section>

          <!-- Section: Emergency Contact -->
          {#if user.contactoEmergenciaNombre}
            <section>
              <h3
                class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-4 pb-2 border-b border-gray-100 dark:border-gray-800"
              >
                Emergencia
              </h3>
              <dl class="space-y-4">
                <div>
                  <dt
                    class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-1 ml-6"
                  >
                    Contacto
                  </dt>
                  <dd class="text-sm text-gray-900 dark:text-gray-200 ml-6">
                    {user.contactoEmergenciaNombre}
                    {#if user.contactoEmergenciaTelefono}
                      <span class="text-gray-400 mx-1">•</span>
                      {user.contactoEmergenciaTelefono}
                    {/if}
                  </dd>
                </div>
              </dl>
            </section>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}
