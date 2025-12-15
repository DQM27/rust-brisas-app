<script lang="ts">
  // VisitaFormFields.svelte - GitHub Style
  import type { VisitaFormData } from "$lib/logic/ingreso/visitaService";

  export let formData: VisitaFormData;
  export let errors: Record<string, string> = {};
  export let onChange: (field: keyof VisitaFormData, value: any) => void;

  function handleInput(field: keyof VisitaFormData) {
    return (e: Event) => {
      const target = e.target as HTMLInputElement | HTMLTextAreaElement;
      onChange(field, target.value);
    };
  }
</script>

<div class="flex flex-col gap-4">
  <!-- Visitor Info Row -->
  <div class="grid grid-cols-3 gap-3">
    <div class="flex flex-col gap-1.5">
      <label for="cedula" class="text-xs font-medium text-[#8d96a0]"
        >Cédula</label
      >
      <input
        id="cedula"
        type="text"
        value={formData.cedula}
        on:input={handleInput("cedula")}
        class="w-full px-3 py-2 bg-[#0d1117] border rounded-md text-sm text-[#f0f6fc] placeholder-[#484f58] focus:ring-2 focus:ring-[#388bfd] focus:border-transparent outline-none transition-all {errors.cedula
          ? 'border-[#f85149]'
          : 'border-[#30363d]'}"
        placeholder="1-1234-5678"
      />
      {#if errors.cedula}
        <span class="text-xs text-[#f85149]">{errors.cedula}</span>
      {/if}
    </div>

    <div class="flex flex-col gap-1.5">
      <label for="nombre" class="text-xs font-medium text-[#8d96a0]"
        >Nombre</label
      >
      <input
        id="nombre"
        type="text"
        value={formData.nombre}
        on:input={handleInput("nombre")}
        class="w-full px-3 py-2 bg-[#0d1117] border rounded-md text-sm text-[#f0f6fc] placeholder-[#484f58] focus:ring-2 focus:ring-[#388bfd] focus:border-transparent outline-none transition-all {errors.nombre
          ? 'border-[#f85149]'
          : 'border-[#30363d]'}"
        placeholder="Nombre"
      />
      {#if errors.nombre}
        <span class="text-xs text-[#f85149]">{errors.nombre}</span>
      {/if}
    </div>

    <div class="flex flex-col gap-1.5">
      <label for="apellido" class="text-xs font-medium text-[#8d96a0]"
        >Apellido</label
      >
      <input
        id="apellido"
        type="text"
        value={formData.apellido}
        on:input={handleInput("apellido")}
        class="w-full px-3 py-2 bg-[#0d1117] border rounded-md text-sm text-[#f0f6fc] placeholder-[#484f58] focus:ring-2 focus:ring-[#388bfd] focus:border-transparent outline-none transition-all {errors.apellido
          ? 'border-[#f85149]'
          : 'border-[#30363d]'}"
        placeholder="Apellido"
      />
      {#if errors.apellido}
        <span class="text-xs text-[#f85149]">{errors.apellido}</span>
      {/if}
    </div>
  </div>

  <!-- Visit Details -->
  <div class="pt-3 border-t border-[#21262d]">
    <p
      class="text-xs font-semibold text-[#8d96a0] uppercase tracking-wider mb-3"
    >
      Detalles de Visita
    </p>

    <div class="grid grid-cols-2 gap-3">
      <div class="flex flex-col gap-1.5">
        <label for="anfitrion" class="text-xs font-medium text-[#8d96a0]"
          >Anfitrión</label
        >
        <input
          id="anfitrion"
          type="text"
          value={formData.anfitrion}
          on:input={handleInput("anfitrion")}
          class="w-full px-3 py-2 bg-[#0d1117] border rounded-md text-sm text-[#f0f6fc] placeholder-[#484f58] focus:ring-2 focus:ring-[#388bfd] focus:border-transparent outline-none transition-all {errors.anfitrion
            ? 'border-[#f85149]'
            : 'border-[#30363d]'}"
          placeholder="Persona que recibe"
        />
        {#if errors.anfitrion}
          <span class="text-xs text-[#f85149]">{errors.anfitrion}</span>
        {/if}
      </div>

      <div class="flex flex-col gap-1.5">
        <label for="areaVisitada" class="text-xs font-medium text-[#8d96a0]"
          >Área</label
        >
        <input
          id="areaVisitada"
          type="text"
          value={formData.areaVisitada}
          on:input={handleInput("areaVisitada")}
          class="w-full px-3 py-2 bg-[#0d1117] border rounded-md text-sm text-[#f0f6fc] placeholder-[#484f58] focus:ring-2 focus:ring-[#388bfd] focus:border-transparent outline-none transition-all {errors.areaVisitada
            ? 'border-[#f85149]'
            : 'border-[#30363d]'}"
          placeholder="Administración, Producción..."
        />
        {#if errors.areaVisitada}
          <span class="text-xs text-[#f85149]">{errors.areaVisitada}</span>
        {/if}
      </div>
    </div>

    <div class="flex flex-col gap-1.5 mt-3">
      <label for="motivoVisita" class="text-xs font-medium text-[#8d96a0]"
        >Motivo</label
      >
      <textarea
        id="motivoVisita"
        value={formData.motivoVisita}
        on:input={handleInput("motivoVisita")}
        class="w-full px-3 py-2 bg-[#0d1117] border rounded-md text-sm text-[#f0f6fc] placeholder-[#484f58] focus:ring-2 focus:ring-[#388bfd] focus:border-transparent outline-none transition-all resize-none {errors.motivoVisita
          ? 'border-[#f85149]'
          : 'border-[#30363d]'}"
        placeholder="Describa brevemente el motivo de la visita"
        rows="2"
      ></textarea>
      {#if errors.motivoVisita}
        <span class="text-xs text-[#f85149]">{errors.motivoVisita}</span>
      {/if}
    </div>
  </div>
</div>
