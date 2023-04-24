/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  daisyui: {
    themes: [
      'synthwave', 'cyberpunk'
    ],
  },
  plugins: [require("daisyui")],
  theme: {
    extend: {},
  },
}

