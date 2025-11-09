import { writable } from 'svelte-local-storage-store';

export const panelLayout = writable('panelLayout', {
  topHeight: 60,
  bottomHeight: 250,
  sidebarWidth: 260
});
