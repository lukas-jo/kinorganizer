/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/*.html.tera"],
  darkMode: 'class',
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

