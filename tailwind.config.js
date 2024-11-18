/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js}", "./templates/**/*.html"],
  theme: {
    fontFamily: {
      josefin_slab: ["Josefin Slab"],
      poppins: ["Poppins", "sans-serif"],
    },
    fontSize: {
      sm: 'clamp(0.8rem, 0.17vi + 0.76rem, 0.89rem)',
      base: 'clamp(1rem, 0.34vi + 0.91rem, 1.19rem)',
      xl: 'clamp(1.25rem, 0.61vi + 1.1rem, 1.58rem)',
      '2xl': 'clamp(1.56rem, 1vi + 1.31rem, 2.11rem)',
      '3xl': 'clamp(1.95rem, 1.56vi + 1.56rem, 2.81rem)',
      '4xl': 'clamp(2.44rem, 2.38vi + 1.85rem, 3.75rem)',
      '5xl': 'clamp(3.05rem, 3.54vi + 2.17rem, 5rem)',
    },
    extend: {},
  },
  plugins: [],
}
