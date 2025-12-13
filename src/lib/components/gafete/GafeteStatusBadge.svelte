<script lang="ts">
  /**
   * Componente para mostrar estado del Gafete
   * Props: status (string)
   */
  export let status: string; // 'disponible' | 'en_uso' | 'perdido' | 'danado' | 'extraviado'

  const baseClass =
    "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border uppercase tracking-wide";

  let classes = "";
  let icon = "";
  let label = "";

  $: {
    switch (status) {
      case "disponible":
      case "activo":
        classes =
          "bg-emerald-50 text-emerald-700 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800";
        icon = "✔";
        label = "Disponible";
        break;
      case "en_uso":
        classes =
          "bg-blue-50 text-blue-700 border-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:border-blue-800";
        icon = "◉";
        label = "En Uso";
        break;
      case "perdido":
        classes =
          "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
        icon = "✖";
        label = "Perdido";
        break;
      case "danado":
        classes =
          "bg-rose-100 text-rose-800 border-rose-300 dark:bg-rose-900/50 dark:text-rose-200 dark:border-rose-700";
        icon = "⚡";
        label = "Dañado";
        break;
      case "extraviado":
        classes =
          "bg-amber-100 text-amber-800 border-amber-300 dark:bg-amber-900/50 dark:text-amber-200 dark:border-amber-700";
        icon = "❓";
        label = "Extraviado";
        break;
      default:
        classes =
          "bg-gray-100 text-gray-800 border-gray-300 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600";
        icon = "-";
        label = status || "Desconocido";
    }
  }
</script>

<span class="{baseClass} {classes}">
  <span class="mr-1 opacity-75">{icon}</span>
  {label}
</span>
