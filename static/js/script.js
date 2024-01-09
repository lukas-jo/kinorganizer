const filmSearchBar = document.querySelector("#filmSearchBar");
const filmList = document.querySelector("#filmList");
const selectedFilm = document.querySelector("#selectedFilm");
const tmdbId = document.querySelector("#tmdb_id");

filmSearchBar.addEventListener("keyup", listSearchResult);

function listSearchResult(event) {
  const query = filmSearchBar.value;
  if (query.length > 2) {
    fetch(`/api/search-tmdb/${query}`)
      .then((res) => res.json())
      .then((json) => json.map(filmToHtml).reduce((p, c) => p + c))
      .then((html) => {
        filmList.innerHTML = html;
      });
  } else {
    filmList.innerHTML = "";
  }
}

function filmToHtml(film) {
  return `<li data-id="${film.tmdb_id}" class="flex items-center gap-x-6 hover:bg-indigo-200 p-4" onclick="setFilmId(this)">
            <img class="rounded-lg" src="https://image.tmdb.org/t/p/w92${film.poster_path}"></img>
            <div class="grid">
              <span class="font-semibold">${film.title}</span>
              <span class="font-normal">${film.year}</span>
            </div>
          </li>`;
}

function setFilmId(film) {
  tmdbId.value = film.dataset.id;

  filmList.classList.add("hidden");
  filmList.innerHTML = "";

  filmSearchBar.classList.add("hidden");
  filmSearchBar.value = "";

  selectedFilm.classList.remove("hidden");
  selectedFilm.innerHTML = film.innerHTML;
}

function removeFilmId() {
  tmdbId.value = "";

  filmList.classList.remove("hidden");
  filmList.innerHTML = "";

  filmSearchBar.classList.remove("hidden");
  filmSearchBar.value = "";
  filmSearchBar.focus();

  selectedFilm.classList.add("hidden");
  selectedFilm.innerHTML = "";
}
