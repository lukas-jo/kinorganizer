/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/*.html"],
  darkMode: "class",
  theme: {
    colors: {
      transparent: "transparent",
      current: "currentColor",
      base: {
        black: "#100F0F",
        950: "#1C1B1A",
        900: "#282726",
        850: "#343331",
        800: "#403E3C",
        700: "#575653",
        600: "#6F6E69",
        500: "#878580",
        300: "#B7B5AC",
        200: "#CECDC3",
        150: "#DAD8CE",
        100: "#E6E4D9",
        50: "#F2F0E5",
        paper: "#FFFCF0",
      },
      red: {
        600: "#AF3029",
        400: "#D14D41",
      },
      orange: {
        600: "#BC5215",
        400: "#DA702C",
      },
      yellow: {
        600: "#AD8301",
        400: "#D0A215",
      },
      green: {
        600: "#66800B",
        400: "#879A39",
      },
      cyan: {
        600: "#24837B",
        400: "#3AA99F",
      },
      blue: {
        600: "#205EA6",
        400: "#4385BE",
      },
      purple: {
        600: "#5E409D",
        400: "#8B7EC8",
      },
      magenta: {
        600: "#A02F6F",
        400: "#CE5D97",
      },
    },
    extend: {
      fontFamily: {
        display: ["Atkinson Hyperlegible", "sans-serif"],
      },
    },
  },
  plugins: [
    require("@tailwindcss/forms"),
  ],
};
