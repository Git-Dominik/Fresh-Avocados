const { invoke } = window.__TAURI__.core;

const search = document.getElementById("search");
const results = document.getElementById("gameResults");

function clearResults() {
    results.innerHTML = "";
}

search.onchange = async () => {
    clearResults();
    console.log(search.value);
    
    let games = await invoke("search_igdb", {
        query: search.value
    });

    console.log(games);

    games.forEach(async game => {
        let gameElement = document.createElement("img");

        gameElement.src = await invoke("cover_igdb", {
            coverId: game.cover
        });

        results.appendChild(gameElement);
    });
};
