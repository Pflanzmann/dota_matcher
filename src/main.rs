use std::cmp::Ordering;
use std::env;
use std::error::Error;

use dotenv::dotenv;
use eframe::egui;
use egui::*;
use serde::{Deserialize, Serialize};

use crate::calender_bar_2_visualizer::calender_bar_2;
use crate::calender_visualizer::calender;

mod line_visualizer;
mod calender_visualizer;
mod crack_visualizer;
mod calender_bar_visualizer;
mod calender_bar_2_visualizer;

struct MyApp {
    games: Vec<Game>,
    image: Option<ColorImage>,
}

#[tokio::main]
async fn main() {
    let games_result = get_matches().await;

    let mut games = match games_result {
        Ok(games) => games,
        Err(_) => vec![]
    };

    games.sort_by(|x1, x2| {
        if x1.start_time > x2.start_time {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let mut app = MyApp::new();
    app.games = games.clone();

    calender_bar_2(&mut app.games.clone());

    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Tatooine",
        options,
        Box::new(|cc| Box::new(app)),
    );
}

async fn get_matches() -> Result<Vec<Game>, Box<dyn Error>> {
    dotenv().ok();

    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => return Err("No API key provided. Please set the API_KEY environment variable.".into()),
    };

    let player_id = match env::var("PLAYER_ID") {
        Ok(key) => key,
        Err(_) => return Err("No player id provided. Please set the PLAYER_ID environment variable.".into()),
    };

    let url = format!("https://api.opendota.com/api/players/{}/matches?api_key={}", player_id, api_key);

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let text = response.text().await?;

        match serde_json::from_str(&text) {
            Ok(games) => Ok(games),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(format!("HTTP Error: {}", response.status()).into())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        Window::new("Do you want to quit?")
            .collapsible(false)
            .resizable(false)
            .auto_sized()
            .show(ctx, |ui| {
                ctx.set_visuals(Visuals::light());

                ui.vertical(|ui| {
                    if ui.button("Recalculate").clicked() {
                        self.image = Some(calender(&mut self.games));
                    }

                    if let Some(image) = &self.image {
                        let texture = ui.ctx().load_texture("image_name", image.clone(), Default::default());

                        ui.image(texture.id(), Vec2::new(5000.0, 210.0));
                    }
                });
            });
    }
}

impl MyApp {
    fn new() -> Self {
        Self { games: Vec::new(), image: None }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Game {
    match_id: Option<i64>,
    player_slot: Option<i64>,
    radiant_win: Option<bool>,
    duration: Option<i64>,
    game_mode: Option<i64>,
    lobby_type: Option<i64>,
    hero_id: Option<i64>,
    start_time: Option<i64>,
    version: Option<i64>,
    kills: Option<i64>,
    deaths: Option<i64>,
    assists: Option<i64>,
    skill: Option<i64>,
    average_rank: Option<i64>,
    leaver_status: Option<i64>,
    party_size: Option<i64>,
}
