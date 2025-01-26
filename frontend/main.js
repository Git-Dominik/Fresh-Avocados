import {listenToTorrentBtn} from "./js/torrent.js";

const { invoke } = window.__TAURI__.core;

// Stopt hele program
//await invoke("get_games");

listenToTorrentBtn();