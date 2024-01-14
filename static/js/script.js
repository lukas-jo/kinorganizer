const filmSearchBar = document.querySelector("#film-search");
const filmList = document.querySelector("#search-results");
const selectedFilm = document.querySelector("#selected-film");
const tmdbId = document.querySelector("#tmdb-id");

filmSearchBar.addEventListener("keyup", listSearchResult);
selectedFilm.addEventListener("click", removeFilmId);

function listSearchResult(event) {
  const query = event.currentTarget.value;
  if (query.length >= 3) {
    fetch(`/api/search/${query}`)
      .then((res) => res.text())
      .then((html) => {
        // filmList.setHTML(html);
        filmList.innerHTML = html;
        Array.from(filmList.children).forEach(c => c.addEventListener("click", setFilmId));
      });
  } else {
    filmList.setHTML("");
  }
}

function setFilmId(event) {
  tmdbId.value = event.currentTarget.dataset.id;

  filmList.classList.add("hidden");
  // filmList.setHTML("");
  filmList.innerHTML = "";

  filmSearchBar.classList.add("hidden");
  filmSearchBar.value = "";

  selectedFilm.classList.remove("hidden");
  selectedFilm.innerHTML = event.currentTarget.innerHTML;
}

function removeFilmId(_) {
  tmdbId.value = "";

  filmList.classList.remove("hidden");
  // filmList.setHTML("");
  filmList.innerHTML = "";

  filmSearchBar.classList.remove("hidden");
  filmSearchBar.value = "";
  filmSearchBar.focus();

  selectedFilm.classList.add("hidden");
  // selectedFilm.setHTML("");
  selectedFilm.innerHTML = "";
}
