/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: "#2563EB",
        "primary-dark": "#1D4ED8",
        secondary: "#64748B",
        accent: "#10B981",
        dark: "#0F172A",
        "dark-card": "#1E293B",
      },
    },
  },
  plugins: [],
}
