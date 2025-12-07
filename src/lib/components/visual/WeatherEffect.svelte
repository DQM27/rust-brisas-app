<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { generalSettings, type Season } from "$lib/stores/settingsStore";
  import { currentSeason } from "$lib/utils/season";

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null;
  let animationFrameId: number;

  interface Particle {
    x: number;
    y: number;
    radius: number;
    speedX: number;
    speedY: number;
    opacity: number;
    color: string;
    rotation?: number;
    rotationSpeed?: number;
    size?: number;
  }

  let particles: Particle[] = [];
  const COUNT = 100;

  // Watch for season changes to reset particles
  let lastSeason: Season | null = null;
  $: if ($currentSeason !== lastSeason) {
    lastSeason = $currentSeason;
    if (canvas) createParticles(); // Only recreate if mounted
  }

  function resize() {
    if (canvas) {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    }
  }

  function createParticles() {
    particles = [];
    for (let i = 0; i < COUNT; i++) {
      resetParticle(i, true);
    }
  }

  function resetParticle(index: number, initial = false) {
    const w = window.innerWidth;
    const h = window.innerHeight;

    let p: Particle = {
      x: Math.random() * w,
      y: initial ? Math.random() * h : -20,
      radius: 0,
      speedX: 0,
      speedY: 0,
      opacity: Math.random() * 0.5 + 0.5,
      color: "#fff",
    };

    switch ($currentSeason) {
      case "winter": // Nieve
        p.radius = Math.random() * 3 + 2;
        p.speedY = Math.random() * 1 + 0.5;
        p.speedX = Math.random() * 0.5 - 0.25;
        p.color = "#CDE4FF";
        break;
      case "spring": // Pétalos
        p.radius = Math.random() * 4 + 2;
        p.speedY = Math.random() * 0.8 + 0.2;
        p.speedX = Math.random() * 1 - 0.5;
        p.color = Math.random() > 0.5 ? "#FFB7C5" : "#FFE4E1";
        p.rotation = Math.random() * 360;
        p.rotationSpeed = Math.random() * 2 - 1;
        break;
      case "summer": // Luciérnagas
        p.y = Math.random() * h;
        p.radius = Math.random() * 2 + 1;
        p.speedY = Math.random() * 0.2 - 0.1;
        p.speedX = Math.random() * 0.2 - 0.1;
        p.color = "#FFD700";
        p.opacity = Math.random();
        break;
      case "autumn": // Hojas
        p.radius = Math.random() * 4 + 3;
        p.speedY = Math.random() * 1 + 0.5;
        p.speedX = Math.random() * 2 - 1;
        const colors = ["#CD5C5C", "#D2691E", "#DAA520", "#8B4513"];
        p.color = colors[Math.floor(Math.random() * colors.length)];
        p.rotation = Math.random() * 360;
        p.rotationSpeed = Math.random() * 3 - 1.5;
        break;
    }

    if (particles[index]) {
      particles[index] = p;
    } else {
      particles.push(p);
    }
  }

  function loop() {
    animationFrameId = requestAnimationFrame(loop);

    if (!ctx || !canvas) return;

    // 1. Check if disabled
    if (!$generalSettings.enableWeatherEffects) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      return; // Skip update/draw
    }

    // 2. Logic & Draw
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    particles.forEach((p, i) => {
      // Logic
      if ($currentSeason === "summer") {
        p.opacity += (Math.random() - 0.5) * 0.05;
        if (p.opacity < 0) p.opacity = 0;
        if (p.opacity > 1) p.opacity = 1;
      }

      // Draw
      ctx!.beginPath();
      ctx!.fillStyle = p.color;
      ctx!.globalAlpha = p.opacity;

      if ($currentSeason === "winter" || $currentSeason === "summer") {
        ctx!.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
        ctx!.fill();
      } else {
        ctx!.save();
        ctx!.translate(p.x, p.y);
        ctx!.rotate(((p.rotation || 0) * Math.PI) / 180);
        ctx!.ellipse(0, 0, p.radius, p.radius * 0.6, 0, 0, Math.PI * 2);
        ctx!.fill();
        ctx!.restore();
        if (p.rotation !== undefined && p.rotationSpeed !== undefined) {
          p.rotation += p.rotationSpeed;
        }
      }

      // Move
      p.x += p.speedX;
      p.y += p.speedY;

      // Bounds
      const outOfBounds =
        p.y > canvas.height + 10 || p.x > canvas.width + 10 || p.x < -10;
      const resetSummer =
        $currentSeason === "summer" && (outOfBounds || Math.random() < 0.001);

      if (outOfBounds || resetSummer) {
        resetParticle(i);
      }
    });
    ctx!.globalAlpha = 1;
  }

  onMount(() => {
    ctx = canvas.getContext("2d");
    resize();
    createParticles();
    window.addEventListener("resize", resize);

    // SINGLE ENTRY POINT FOR LOOP
    loop();
  });

  onDestroy(() => {
    if (typeof window !== "undefined") {
      window.removeEventListener("resize", resize);
      cancelAnimationFrame(animationFrameId);
    }
  });
</script>

<canvas
  bind:this={canvas}
  class="fixed inset-0 pointer-events-none"
  style="z-index: 50;"
></canvas>
