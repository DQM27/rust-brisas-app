/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class', // activa modo oscuro vía clase 'dark'
  content: [
    './src/**/*.{html,svelte,ts,js}', // Tailwind procesará estos archivos
  ],
  theme: {
    extend: {}, // aquí puedes extender colores, fuentes, etc.
  },
  plugins: [],
}
