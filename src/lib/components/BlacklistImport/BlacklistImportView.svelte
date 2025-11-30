<!-- $lib/components/BlacklistImportView.svelte -->
<script lang="ts">
  import BlacklistImportContainer from "./BlacklistImportContainer.svelte";
  import { currentUser } from "$lib/stores/auth";

  const userId = $derived($currentUser?.id || "");

  // Si no hay usuario, mostrar mensaje
  const hasUser = $derived(!!$currentUser);
</script>

{#if hasUser}
  <BlacklistImportContainer {userId} />
{:else}
  <div class="error-container">
    <p class="error-message">Error: Usuario no autenticado</p>
  </div>
{/if}

<style>
  .error-container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background-color: #1e1e1e;
  }

  .error-message {
    padding: 20px 32px;
    background-color: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.2);
    border-radius: 8px;
    color: #f87171;
    font-size: 14px;
  }
</style>
