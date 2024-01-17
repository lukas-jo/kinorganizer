document.querySelector("#colorscheme").addEventListener("change", (event) => {
  switch (event.currentTarget.value) {
    case "dark":
      localStorage.theme = "dark";
      document.documentElement.classList.add("dark");
      break;
    case "light":
      localStorage.theme = "light";
      document.documentElement.classList.remove("dark");
      break;
    default:
      localStorage.removeItem("theme");
      if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
        document.documentElement.classList.add("dark");
      } else {
        document.documentElement.classList.remove("dark");
      }
  }
});
