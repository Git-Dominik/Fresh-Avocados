const { invoke } = window.__TAURI__.core;

const search = document.getElementById("search");
const results = document.getElementById("gameResults");
const library = document.getElementById("library");

const installedGames = await invoke("get_installed_games");

installedGames.forEach(async game => {
    console.log(game);

    let gameData = await invoke("get_game_by_id", {
        gameId: game.igdb_id
    });
    console.log(gameData);

    let gameElement = document.createElement("img");
    gameElement.title = game.name;
    gameElement.src = await invoke("cover_igdb", {
        coverId: gameData.cover
    });

    gameElement.onclick = async () => {
        invoke("launch_game", {
            gameId: game.igdb_id
        });
    };

    library.appendChild(gameElement);
});

search.onchange = async () => {
    results.innerHTML = "";
    console.log(search.value);
    
    let games = await invoke("search_igdb", {
        query: search.value
    });

    console.log(games);

    games.forEach(async game => {
        let gameElement = document.createElement("img");
        gameElement.title = game.name;
        gameElement.src = await invoke("cover_igdb", {
            coverId: game.cover
        });

        gameElement.onclick = async () => {
            invoke("add_game", {
                id: game.id
            });
        };

        results.appendChild(gameElement);
    });
};
