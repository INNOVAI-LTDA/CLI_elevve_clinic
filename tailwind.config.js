/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./*.html", "./public/js/**/*.js"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        "secondary": "#735c00",
        "surface-container": "#edefe7",
        "error": "#ba1a1a",
        "on-secondary": "#ffffff",
        "secondary-container": "#fed65b",
        "surface-dim": "#d9dbd3",
        "on-secondary-fixed-variant": "#574500",
        "error-container": "#ffdad6",
        "on-secondary-fixed": "#241a00",
        "primary-fixed-dim": "#a5d0b9",
        "on-tertiary-container": "#65b68e",
        "on-tertiary-fixed": "#002113",
        "secondary-fixed": "#ffe088",
        "surface-variant": "#e2e3db",
        "secondary-fixed-dim": "#e9c349",
        "surface-bright": "#f9faf2",
        "tertiary-container": "#00452d",
        "surface-container-lowest": "#ffffff",
        "outline-variant": "#c1c8c2",
        "surface-tint": "#3f6653",
        "primary-container": "#1b4332",
        "inverse-primary": "#a5d0b9",
        "on-primary": "#ffffff",
        "on-primary-container": "#86af99",
        "on-background": "#1a1c18",
        "on-primary-fixed-variant": "#274e3d",
        "on-secondary-container": "#745c00",
        "primary": "#012d1d",
        "background": "#f9faf2",
        "tertiary": "#002d1b",
        "tertiary-fixed": "#a1f4c8",
        "on-surface": "#1a1c18",
        "primary-fixed": "#c1ecd4",
        "surface-container-low": "#f3f4ec",
        "surface-container-highest": "#e2e3db",
        "inverse-on-surface": "#f0f1e9",
        "on-tertiary": "#ffffff",
        "on-tertiary-fixed-variant": "#005236",
        "surface": "#f9faf2",
        "on-error": "#ffffff",
        "inverse-surface": "#2f312c",
        "tertiary-fixed-dim": "#86d7ad",
        "on-error-container": "#93000a",
        "outline": "#717973",
        "surface-container-high": "#e8e9e1",
        "on-primary-fixed": "#002114",
        "on-surface-variant": "#414844"
      },
      fontFamily: {
        "headline": ["Noto Serif", "serif"],
        "body": ["Manrope", "sans-serif"],
        "label": ["Manrope", "sans-serif"]
      },
      borderRadius: {
        "DEFAULT": "0.25rem",
        "lg": "0.5rem",
        "xl": "0.75rem",
        "full": "9999px"
      }
    }
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/container-queries')
  ]
}
