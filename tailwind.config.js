/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
    "!./assets/styling/octo_corner.css",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
  safelist: [
    {
      pattern:
        /(bg|text|border)-(slate|gray|zinc|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose|black|white)-[0-9]{3}/,
    },
    "[&::-webkit-progress-value]:bg-green-400",
    "[&::-webkit-progress-value]:bg-orange-400",
    "[&::-webkit-progress-value]:bg-red-400",
    "[&::-moz-progress-bar]:bg-green-400",
    "[&::-moz-progress-bar]:bg-orange-400",
    "[&::-moz-progress-bar]:bg-red-400",
  ],
};
