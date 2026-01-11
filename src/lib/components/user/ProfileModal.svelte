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
  import * as userService from "$lib/logic/user/userService";
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

  // Avatar Logic
  let avatarUrl = $state<string | null>(null);

  async function loadUserAvatar(userId: string) {
    const result = await userService.getUserAvatar(userId);
    if (result.ok) {
      avatarUrl = `data:image/webp;base64,${result.data}`;
    } else {
      avatarUrl = null;
    }
  }

  $effect(() => {
    if (show && user) {
      loadUserAvatar(user.id);
    } else {
      avatarUrl = null;
    }
  });

  const fullName = $derived(
    user
      ? [user.nombre, user.segundoNombre, user.apellido, user.segundoApellido]
          .filter(Boolean)
          .join(" ")
      : "",
  );
</script>

{#if show && user}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 bg-black/50 dark:bg-black/70 backdrop-blur-sm z-[9999] flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
    onclick={onClose}
    onkeydown={(e) => e.key === "Escape" && onClose()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="bg-surface-2 w-full max-w-3xl rounded-xl shadow-2xl flex flex-col max-h-[90vh] border border-surface overflow-hidden"
      transition:fly={{ y: 20, duration: 300 }}
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <!-- Header with Cover & Avatar -->
      <div class="relative flex-none">
        <!-- Sober Banner with Dot Pattern -->
        <div
          class="h-32 bg-surface-3 relative overflow-hidden border-b border-surface"
        >
          <div
            class="absolute inset-0 opacity-[0.03] bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0IiBoZWlnaHQ9IjQiPgo8cmVjdCB3aWR0aD0iNCIgaGVpZ2h0PSI0IiBmaWxsPSIjZmZmIi8+CjxyZWN0IHdpZHRoPSIxIiBoZWlnaHQ9IjEiIGZpbGw9IiMwMDAiLz4KPC9zdmc+')]"
          ></div>
          <!-- Subtle Gradient Overlay -->
          <div
            class="absolute inset-0 bg-gradient-to-b from-transparent to-black/20"
          ></div>
        </div>

        <!-- Close Button -->
        <button
          onclick={onClose}
          class="absolute top-4 right-4 p-2 rounded-full bg-black/20 hover:bg-black/40 text-white transition-colors backdrop-blur-md z-30"
        >
          <X size={20} />
        </button>

        <!-- Avatar & Basic Info Container -->
        <div
          class="px-8 -mt-24 relative z-20 flex items-center justify-between gap-4"
        >
          <div class="flex items-center gap-5 flex-1 min-w-0">
            <div
              class="flex-none w-28 h-28 rounded-full border-4 border-[#0d1117] bg-surface-2 flex items-center justify-center text-3xl font-bold text-white shadow-xl overflow-hidden relative group"
            >
              {#if avatarUrl}
                <img
                  src={avatarUrl}
                  alt="Avatar"
                  class="w-full h-full object-cover transition-transform group-hover:scale-110 duration-500"
                />
              {:else}
                <div
                  class="w-full h-full bg-blue-600 flex items-center justify-center text-white"
                >
                  {userInitials}
                </div>
              {/if}
            </div>

            <div class="pb-2 flex-1 min-w-0">
              <h1
                class="text-2xl font-bold text-primary tracking-tight leading-tight"
                title={fullName}
              >
                {fullName}
              </h1>
              <div
                class="flex items-center gap-2 text-secondary text-sm mt-0.5"
              >
                <span
                  class="inline-flex items-center gap-1.5 px-2 py-0.5 bg-surface-3 rounded-md border border-surface text-[10px] font-bold uppercase tracking-wider text-blue-400"
                >
                  <Shield size={12} />
                  {user.roleName || "N/A"}
                </span>
                <span class="text-surface border-l h-3 mx-1"></span>
                <span
                  class="flex items-center gap-1.5 opacity-80 decoration-white/20 hover:decoration-white/40 transition-all cursor-default"
                >
                  <Mail size={14} />
                  {user.email}
                </span>
              </div>
            </div>
          </div>

          <div class="pb-2">
            <button
              onclick={onEdit}
              class="flex items-center gap-2 px-4 py-2 bg-surface-3 hover:bg-surface-hover text-sm font-medium text-primary border border-surface rounded-lg shadow-sm transition-all active:scale-95"
            >
              <PenSquare size={16} class="text-blue-400" />
              Editar Perfil
            </button>
          </div>
        </div>
      </div>

      <!-- Content Grid: Consolidated into a single intelligent container -->
      <div class="flex-1 overflow-y-auto p-8 pt-10">
        <div
          class="bg-surface-1 p-6 rounded-xl border border-surface shadow-inner-surface"
        >
          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-8">
            <!-- Personal Info -->
            {#if user.cedula || user.fechaNacimiento}
              <div class="space-y-6">
                <h3
                  class="text-[10px] font-bold text-primary/40 uppercase tracking-[0.2em] mb-4"
                >
                  Información Personal
                </h3>
                <div class="space-y-5">
                  {#if user.cedula}
                    <div class="flex items-start gap-4 group">
                      <div
                        class="p-2 bg-surface-2 rounded-lg text-blue-400 border border-surface group-hover:border-blue-500/30 transition-colors shadow-sm"
                      >
                        <CreditCard size={16} />
                      </div>
                      <div>
                        <dt
                          class="text-[10px] font-bold uppercase tracking-wider text-secondary/50"
                        >
                          Cédula
                        </dt>
                        <dd class="text-sm font-medium text-primary mt-0.5">
                          {user.cedula}
                        </dd>
                      </div>
                    </div>
                  {/if}
                  {#if user.fechaNacimiento}
                    <div class="flex items-start gap-4 group">
                      <div
                        class="p-2 bg-surface-2 rounded-lg text-blue-400 border border-surface group-hover:border-blue-500/30 transition-colors shadow-sm"
                      >
                        <Calendar size={16} />
                      </div>
                      <div>
                        <dt
                          class="text-[10px] font-bold uppercase tracking-wider text-secondary/50"
                        >
                          Fecha Nacimiento
                        </dt>
                        <dd class="text-sm font-medium text-primary mt-0.5">
                          {formatDate(user.fechaNacimiento)}
                        </dd>
                      </div>
                    </div>
                  {/if}
                </div>
              </div>
            {/if}

            <!-- Contact Info -->
            {#if user.telefono || user.direccion}
              <div class="space-y-6">
                <h3
                  class="text-[10px] font-bold text-primary/40 uppercase tracking-[0.2em] mb-4"
                >
                  Canales de Contacto
                </h3>
                <div class="space-y-5">
                  {#if user.telefono}
                    <div class="flex items-start gap-4 group">
                      <div
                        class="p-2 bg-surface-2 rounded-lg text-blue-400 border border-surface group-hover:border-blue-500/30 transition-colors shadow-sm"
                      >
                        <Phone size={16} />
                      </div>
                      <div>
                        <dt
                          class="text-[10px] font-bold uppercase tracking-wider text-secondary/50"
                        >
                          Teléfono Personal
                        </dt>
                        <dd class="text-sm font-medium text-primary mt-0.5">
                          {user.telefono}
                        </dd>
                      </div>
                    </div>
                  {/if}
                  {#if user.direccion}
                    <div class="flex items-start gap-4 group">
                      <div
                        class="p-2 bg-surface-2 rounded-lg text-blue-400 border border-surface group-hover:border-blue-500/30 transition-colors shadow-sm"
                      >
                        <MapPin size={16} />
                      </div>
                      <div>
                        <dt
                          class="text-[10px] font-bold uppercase tracking-wider text-secondary/50"
                        >
                          Dirección Residencial
                        </dt>
                        <dd
                          class="text-sm font-medium text-primary mt-0.5 leading-relaxed"
                        >
                          {user.direccion}
                        </dd>
                      </div>
                    </div>
                  {/if}
                </div>
              </div>
            {/if}

            <!-- Work Info -->
            {#if user.fechaInicioLabores || user.numeroGafete}
              <div class="space-y-6">
                <h3
                  class="text-[10px] font-bold text-primary/40 uppercase tracking-[0.2em] mb-4"
                >
                  Información Laboral
                </h3>
                <div class="space-y-5">
                  {#if user.fechaInicioLabores}
                    <div class="flex items-start gap-4 group">
                      <div
                        class="p-2 bg-surface-2 rounded-lg text-blue-400 border border-surface group-hover:border-blue-500/30 transition-colors shadow-sm"
                      >
                        <Briefcase size={16} />
                      </div>
                      <div>
                        <dt
                          class="text-[10px] font-bold uppercase tracking-wider text-secondary/50"
                        >
                          Fecha de Ingreso
                        </dt>
                        <dd class="text-sm font-medium text-primary mt-0.5">
                          {formatDate(user.fechaInicioLabores)}
                        </dd>
                      </div>
                    </div>
                  {/if}
                  {#if user.numeroGafete}
                    <div class="flex items-start gap-4 group">
                      <div
                        class="p-2 bg-surface-2 rounded-lg text-blue-400 border border-surface group-hover:border-blue-500/30 transition-colors shadow-sm"
                      >
                        <Shield size={16} />
                      </div>
                      <div>
                        <dt
                          class="text-[10px] font-bold uppercase tracking-wider text-secondary/50"
                        >
                          Número de Gafete
                        </dt>
                        <dd class="text-sm font-medium text-primary mt-0.5">
                          {user.numeroGafete}
                        </dd>
                      </div>
                    </div>
                  {/if}
                </div>
              </div>
            {/if}

            <!-- Emergency Contact -->
            {#if user.contactoEmergenciaNombre || user.contactoEmergenciaTelefono}
              <div class="space-y-6">
                <h3
                  class="text-[10px] font-bold text-primary/40 uppercase tracking-[0.2em] mb-4"
                >
                  🔒 Contacto de Emergencia
                </h3>
                <div class="space-y-5">
                  {#if user.contactoEmergenciaNombre}
                    <div class="flex items-start gap-4 group">
                      <div
                        class="p-2 bg-surface-2 rounded-lg text-blue-400 border border-surface group-hover:border-blue-500/30 transition-colors shadow-sm"
                      >
                        <User size={16} />
                      </div>
                      <div>
                        <dt
                          class="text-[10px] font-bold uppercase tracking-wider text-secondary/50"
                        >
                          Nombre del Contacto
                        </dt>
                        <dd class="text-sm font-medium text-primary mt-0.5">
                          {user.contactoEmergenciaNombre}
                        </dd>
                      </div>
                    </div>
                  {/if}
                  {#if user.contactoEmergenciaTelefono}
                    <div class="flex items-start gap-4 group">
                      <div
                        class="p-2 bg-surface-2 rounded-lg text-blue-400 border border-surface group-hover:border-blue-500/30 transition-colors shadow-sm"
                      >
                        <Phone size={16} />
                      </div>
                      <div>
                        <dt
                          class="text-[10px] font-bold uppercase tracking-wider text-secondary/50"
                        >
                          Teléfono Emergencia
                        </dt>
                        <dd class="text-sm font-medium text-primary mt-0.5">
                          {user.contactoEmergenciaTelefono}
                        </dd>
                      </div>
                    </div>
                  {/if}
                </div>
              </div>
            {/if}
          </div>

          <!-- Empty State if no fields are defined (Safety Check) -->
          {#if !user.cedula && !user.fechaNacimiento && !user.telefono && !user.direccion && !user.fechaInicioLabores && !user.numeroGafete && !user.contactoEmergenciaNombre}
            <div class="text-center py-10">
              <p class="text-secondary/40 text-sm italic">
                No hay información adicional registrada para este perfil.
              </p>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}
