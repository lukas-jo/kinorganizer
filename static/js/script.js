const tmdbSearch = document.querySelector("#filmSearchBar");
tmdbSearch.addEventListener(
  "keydown",
  (e) =>
    fetch(`/api/search-tmdb/${tmdbSearch.value}`)
      .then((res) => res.json())
      .then((json) => json.map(filmToHtml).reduce((p, c) => p + c))
      .then((html) => { document.querySelector('#filmList').innerHTML = html} )
  ,
);

function filmToHtml(film) {
  return `<div id="film-${film.tmdb_id}" class="hover:bg-red-100" onclick="setFilmId(${film.tmdb_id})">
            <img class="" src="https://image.tmdb.org/t/p/w92${film.poster_path}"
            <span class="">${film.title}</span>
            <span class="">${film.year}</span>
          </div>`
}

function setFilmId(id) {
  document.querySelector('#tmdb_id').value = id;
  const html = document.querySelector(`#film-${id}`).outerHTML;
  document.querySelector('#filmList').innerHTML = '';
  document.querySelector('#filmSearchBar').value = '';
  document.querySelector('#selectedFilm').innerHTML = html;
}
