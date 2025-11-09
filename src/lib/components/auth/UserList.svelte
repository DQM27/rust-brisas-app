
<script lang="ts">
  import type { User } from '$lib/types';
  
  export let users: User[] = [];
  
  function getInitial(user: User): string {
    return user.nombre.charAt(0).toUpperCase();
  }
</script>

{#if users.length > 0}
  <div class="card">
    <div class="card-header">
      <h2>Usuarios Registrados ({users.length})</h2>
    </div>
    <div class="user-list">
      {#each users as user}
        <div class="user-item">
          <div class="user-item-avatar">
            {getInitial(user)}
          </div>
          <div class="user-item-info">
            <div class="user-item-name">{user.nombre} {user.apellido}</div>
            <div class="user-item-email">{user.email}</div>
          </div>
          <div class="user-item-badges">
            <span class="badge {user.role === 'admin' ? 'badge-admin' : 'badge-user'}">
              {user.role}
            </span>
            {#if !user.isActive}
              <span class="badge badge-inactive">Inactivo</span>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}
