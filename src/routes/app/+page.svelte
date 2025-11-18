<script lang="ts">
  import { onMount } from 'svelte';
  import { openTab, tabsStore } from '$lib/stores/tabs';
  import { get } from 'svelte/store';
  import TabsContainer from '$lib/components/TabsContainer.svelte';
  
  onMount(() => {
    const tabs = get(tabsStore);
    if (tabs.length === 0) {
      openTab({
        componentKey: 'welcome',
        title: 'Bienvenida',
        id: 'welcome'
      });
    }
  });
</script>

<TabsContainer />