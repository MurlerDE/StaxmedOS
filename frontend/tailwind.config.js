/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,ts}"],
  theme: {
    extend: {
      colors: {
        // Wir verknüpfen die Tailwind-Namen mit den CSS-Variablen
        bg: 'var(--color-bg)',
        surface: 'var(--color-surface)',
        primary: {
          DEFAULT: 'var(--color-primary)',
          hover: 'var(--color-primary-hover)',
        },
        secondary: 'var(--color-secondary)',
        accent: 'var(--color-accent)',
        main: 'var(--color-text-main)',
        muted: 'var(--color-text-muted)',
        border: 'var(--color-border)',
        success: 'var(--color-success)',
        error: 'var(--color-error)',
        warning: 'var(--color-warning)',
      },
    },
  },
  plugins: [],
};
