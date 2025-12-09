<script lang="ts">
  import SceneRenderer from "./SceneRenderer.svelte";
  
  interface Props {
    name?: string;
  }
  
  let { name = "Usuario" }: Props = $props();
</script>

<!-- 
  Birthday mode wrapper.
  Uses SceneRenderer in birthday mode with animated gradient background,
  confetti rain, and fireworks.
-->
<SceneRenderer isBirthday={true} />

<!-- Name sparkle overlay could go here if needed -->
