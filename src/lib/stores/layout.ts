import { persisted } from 'svelte-persisted-store';

export const panelLayout = persisted('panelLayout', {
  topHeight: 60,
  bottomHeight: 250,
  sidebarWidth: 260
});