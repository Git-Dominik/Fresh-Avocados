const { invoke } = window.__TAURI__.core;

await invoke("get_games");
