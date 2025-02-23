use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: u32,
    pub cover: Option<u32>,
    pub genres: Option<Vec<u32>>,
    pub name: String,
    pub rating: Option<f32>,
    pub storyline: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cover {
    pub url: String
}

pub struct IGDB {
    client: reqwest::Client,
    client_id: String,
    client_secret: String,
}

impl IGDB {
    pub fn new() -> IGDB {
        IGDB {
            client: reqwest::Client::new(),
            client_id: "ikjpiuua1mns8fbz83ccwtsxk8p3aw".into(),
            client_secret: "xts363ulha6ldje8a8l56pqhcnn36x".into(),
        }
    }

    pub async fn search_games(&self, query: &str) -> Vec<Game> {
        let res = self.client
            .post("https://api.igdb.com/v4/games")
            .header("Client-ID", self.client_id.as_str())
            .header("Authorization", format!("Bearer {}", self.client_secret))
            .body(format!("search \"{}\"; fields id, name, storyline, rating, genres, cover; limit 5;", query))
            .send()
            .await
            .expect("failed to send request");

        res.json::<Vec<Game>>().await.unwrap()
    }
    
    pub async fn get_game_by_id(&self, game_id: u32) -> Option<Game> {
        let res = self.client
            .post("https://api.igdb.com/v4/games")
            .header("Client-ID", self.client_id.as_str())
            .header("Authorization", format!("Bearer {}", self.client_secret))
            .body(format!(
                "fields id, name, storyline, rating, genres, cover; 
                where id = {}; 
                limit 1;", 
                game_id
            ))
            .send()
            .await
            .expect("failed to send request");

        res.json::<Vec<Game>>().await.unwrap().into_iter().next()
    }

    pub async fn get_cover(&self, cover_id: u32) -> Cover {
        let res = self.client
            .post("https://api.igdb.com/v4/covers")
            .header("Client-ID", self.client_id.as_str())
            .header("Authorization", format!("Bearer {}", self.client_secret))
            .body(format!("fields url; where id = {}; limit 1;", cover_id))
            .send()
            .await
            .expect("failed to send request");

        let mut cover: Cover = res.json::<Vec<Cover>>().await.unwrap().first().unwrap().clone();
        cover.url = cover.url.replace("t_thumb", "t_cover_big");

        cover
    }
}