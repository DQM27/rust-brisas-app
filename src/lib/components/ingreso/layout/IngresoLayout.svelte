<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";

  // Tabs configuration
  const tabs = [
    {
      id: "dashboard",
      label: "Dashboard",
      path: "/ingreso/dashboard",
      icon: "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z",
    },
    {
      id: "contratistas",
      label: "Contratistas",
      path: "/ingreso/contratistas",
      icon: "M13 10V3L4 14h7v7l9-11h-7z",
    }, // Example icon
    {
      id: "visitas",
      label: "Visitas",
      path: "/ingreso/visitas",
      icon: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z",
    },
    {
      id: "proveedores",
      label: "Proveedores",
      path: "/ingreso/proveedores",
      icon: "M9 17a2 2 0 11-4 0 2 2 0 014 0zM19 17a2 2 0 11-4 0 2 2 0 014 0z M13 16V6a1 1 0 00-1-1H4a1 1 0 00-1 1v10a1 1 0 001 1h1m8-1a1 1 0 01-1 1H9m4-1V8a1 1 0 011-1h2.586a1 1 0 01.707.293l3.414 3.414a1 1 0 01.293.707V16a1 1 0 01-1 1h-1m-6-1a1 1 0 001 1h1M5 17a2 2 0 104 0m-4 0a2 2 0 114 0m6 0a2 2 0 104 0m-4 0a2 2 0 114 0",
    },
  ];

  /* 
    Current path detection logic would go here if we were using real routing.
    Since this might be a component rendered inside a main page, we manage active tab here
    or valid its props if controlled by parent.
    For this layout, let's assume it wraps slots.
  */

  export let activeTab: string = "dashboard";

  function navigate(tab: any) {
    activeTab = tab.id;
    // goto(tab.path); // If using router
    // dispatch?
  }
</script>

<div class="flex flex-col h-full bg-base-100">
  <!-- Header / Navbar specific for Ingresos -->
  <div class="navbar bg-base-100 border-b border-base-200">
    <div class="flex-1">
      <h1
        class="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-primary to-secondary ml-2"
      >
        Control de Ingresos
      </h1>
    </div>
    <div class="flex-none">
      <!-- Global Actions could go here (e.g. "Reportar Emergencia") -->
    </div>
  </div>

  <!-- Tabs Navigation -->
  <div class="bg-base-100 pt-2 px-4 shadow-sm z-10">
    <div class="tabs tabs-boxed bg-transparent p-0 gap-2">
      {#each tabs as tab}
        <button
          class="tab tab-lg h-12 px-6 rounded-t-lg transition-all duration-200
                           {activeTab === tab.id
            ? 'tab-active bg-primary text-primary-content shadow-lg transform -translate-y-1'
            : 'hover:bg-base-200 text-base-content/70'}"
          on:click={() => navigate(tab)}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5 mr-2"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d={tab.icon}
            />
          </svg>
          <span class="font-medium text-sm tracking-wide">{tab.label}</span>
        </button>
      {/each}
    </div>
  </div>

  <!-- Main Content Area -->
  <main class="flex-1 overflow-auto p-6 bg-base-200/50 relative">
    <div class="container mx-auto">
      <slot />
    </div>
  </main>
</div>
