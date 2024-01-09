/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/*.html.tera", "./static/js/*.js"],
  darkMode: 'class',
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

