<script lang="ts">
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";

  export let name: string = "Usuario";

  // Elements for background pattern
  const decorations = Array.from({ length: 20 }).map((_, i) => ({
    id: i,
    x: Math.random() * 100,
    y: Math.random() * 100,
    scale: 0.5 + Math.random() * 0.5,
    rotation: Math.random() * 360,
    type: ["🎂", "🎈", "🎁", "✨"][Math.floor(Math.random() * 4)],
    delay: Math.random() * 5,
    duration: 10 + Math.random() * 20,
  }));

  // Simple Confetti particle system
  let particles: Array<{
    x: number;
    y: number;
    color: string;
    speed: number;
    wobble: number;
  }> = [];

  onMount(() => {
    // Initialize simple confetti loop if we wanted canvas, but let's stick to CSS for simplicity and performance
  });
</script>

<div
  class="absolute inset-0 overflow-hidden bg-linear-to-br from-sky-400 via-indigo-400 to-teal-400 selection:bg-indigo-600"
>
  <!-- Animated Background Gradient Mesh (Fake) -->
  <div class="absolute inset-0 opacity-30">
    <div
      class="absolute top-0 left-0 w-full h-full bg-gradient-radial from-yellow-300 to-transparent scale-150 animate-pulse"
    ></div>
  </div>

  <!-- Floating Decorations (Pure CSS parallax-ish) -->
  {#each decorations as deco (deco.id)}
    <div
      class="absolute text-4xl select-none"
      style="
        left: {deco.x}%; 
        top: {deco.y}%; 
        transform: scale({deco.scale}) rotate({deco.rotation}deg);
        animation: float {deco.duration}s infinite ease-in-out alternate;
        animation-delay: -{deco.delay}s;
        opacity: 0.6;
      "
    >
      {deco.type}
    </div>
  {/each}

  <!-- Big Central Cake (SVG) -->
  <div
    class="absolute inset-0 flex flex-col items-center justify-center opacity-30 pointer-events-none"
  >
    <svg
      viewBox="0 0 512 512"
      class="w-64 h-64 animate-bounce-slow text-white fill-current drop-shadow-lg"
    >
      <!-- Simplified Cake Icon Path -->
      <path
        d="M416 277.333V490.667H96V277.333H64V512H448V277.333H416ZM266.666 0H245.333V42.6667H266.666V0ZM298.666 0H277.333V42.6667H298.666V0ZM330.666 0H309.333V42.6667H330.666V0ZM181.333 42.6667H330.666V85.3333H181.333V42.6667ZM181.333 106.667H330.666V149.333H181.333V106.667ZM149.333 170.667H362.666V213.333H149.333V170.667ZM96 234.667H416V256H96V234.667Z"
      />
    </svg>
  </div>
</div>

<style>
  @keyframes float {
    0% {
      transform: translateY(0px) rotate(0deg);
    }
    100% {
      transform: translateY(-50px) rotate(10deg);
    }
  }
  .animate-bounce-slow {
    animation: bounce 3s infinite;
  }
  @keyframes bounce {
    0%,
    100% {
      transform: translateY(-5%);
    }
    50% {
      transform: translateY(5%);
    }
  }
</style>
