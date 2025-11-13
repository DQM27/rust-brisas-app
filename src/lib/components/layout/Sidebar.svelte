
<script lang="ts">
  import { activeView } from '$lib/stores/ui';
  import { get } from 'svelte/store';
  import { isAuthenticated } from '$lib/stores/auth';
  import { resetTabs, openTab } from '$lib/stores/tabs';
  import { getComponent } from '$lib/components/registry';
  import { logout } from '$lib/stores/auth';

  import {
    User,
    Lock,
    FileText,
    Settings,
    LogIn,
    UserPlus,
  } from 'lucide-svelte';

  const items = [
    { id: 'users', icon: User, label: 'Usuarios' },
    { id: 'access', icon: Lock, label: 'Accesos' },
    { id: 'logs', icon: FileText, label: 'Logs' },
    { id: 'settings', icon: Settings, label: 'Configuración' },
  ];

  function select(view: string) {
    activeView.set(view);
  }

  function openUserRegistration() {
    openTab({
      componentKey: 'user-register',
      title: 'Registrar Usuario',
      focusOnOpen: true
    });
  }

  function logout() {
    isAuthenticated.set(false);
    resetTabs();
    activeView.set('');
  }

  const user = {
    name: 'Daniel',
    initials: 'DQ'
  };
</script>

<div class="sidebar">
  <div class="top">
    {#each items as item}
      <button
        class:selected={get(activeView) === item.id}
        on:click={() => select(item.id)}
      >
        <svelte:component this={item.icon} size={22} />
        <span class="tooltip">{item.label}</span>
      </button>
    {/each}

    <!-- Botón para abrir panel de registro como tab -->
    <button on:click={openUserRegistration} title="Registrar nuevo usuario">
      <UserPlus size={22} />
      <span class="tooltip">Registrar Usuario</span>
    </button>
  </div>

  <div class="bottom">
    <div class="avatar" title={user.name}>
      {user.initials}
    </div>

    <button on:click={logout}>Cerrar sesión</button>
      <LogIn size={24} />
    </button>
  </div>
</div>

<style>
  .sidebar {
    width: 52px;
    background: #2d2d2d;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 6px 0;
    border-right: 1px solid #1f1f1f;
  }

  .top {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  button {
    background: none;
    border: none;
    color: #bbb;
    width: 100%;
    height: 42px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    cursor: pointer;
  }

  button.selected {
    color: #fff;
    background: #3c3c3c;
  }

  button:hover {
    background: #3a3a3a;
    color: #fff;
  }

  .tooltip {
    position: absolute;
    left: 52px;
    background: #3a3a3a;
    white-space: nowrap;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 11.5px;
    display: none;
    z-index: 1000;
  }

  button:hover .tooltip {
    display: block;
  }

  .bottom {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding-bottom: 8px;
  }

  .avatar {
    width: 32px;
    height: 32px;
    background: #764ba2;
    border-radius: 50%;
    font-size: 13px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    color: white;
    cursor: pointer;
  }

  .avatar:hover {
    background: #8c5fc3;
  }

  .logout {
    width: 80%;
    border-radius: 6px;
    height: 28px;
  }
</style>