
<script lang="ts">
  import type { User } from '$lib/types/user';
  import { createEventDispatcher } from 'svelte';
  
  export let user: User;
  
  const dispatch = createEventDispatcher();
  
  function getInitials(user: User): string {
    return user.nombre.charAt(0).toUpperCase() + user.apellido.charAt(0).toUpperCase();
  }
</script>

<div class="card">
  <div class="user-info">
    <div class="user-avatar">
      {getInitials(user)}
    </div>
    <div class="user-details">
      <h3>{user.nombre} {user.apellido}</h3>
      <p>{user.email}</p>
      <span class="badge {user.role === 'admin' ? 'badge-admin' : 'badge-user'}">
        {user.role}
      </span>
      {#if !user.isActive}
        <span class="badge badge-inactive">Inactivo</span>
      {/if}
    </div>
    <button on:click={() => dispatch('logout')} class="btn btn-secondary btn-sm">
      Cerrar Sesión
    </button>
  </div>
</div>
