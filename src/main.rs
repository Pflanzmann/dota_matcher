use std::cmp::Ordering;
use std::env;
use std::error::Error;

use dotenv::dotenv;
use eframe::egui;
use egui::*;
use serde::{Deserialize, Serialize};

use crate::calender_bar_2_visualizer::calender_bar_2_visualizer;
use crate::calender_bar_visualizer::calender_bar_visualizer;
use crate::calender_visualizer::calender_visualizer;
use crate::crack_visualizer::crack_visualizer;
use crate::line_visualizer::line_visualizer;

mod line_visualizer;
mod calender_visualizer;
mod crack_visualizer;
mod calender_bar_visualizer;
mod calender_bar_2_visualizer;

struct MyApp {
    games: Vec<Game>,
    images: Vec<Diagram>,
}

impl MyApp {
    fn new(games: Vec<Game>) -> Self {
        Self { games, images: vec![] }
    }
}

fn convert_bmp_to_egui_image(bitmap: &bmp::Image) -> ColorImage {
    let mut color_image = ColorImage::new([bitmap.get_width() as usize, bitmap.get_height() as usize], Color32::TRANSPARENT);
    for x in 0..bitmap.get_width() {
        for y in 0..bitmap.get_height() {
            let pixel = bitmap.get_pixel(x, y);
            color_image[(x as usize, y as usize)] = Color32::from_rgb(pixel.r.clone(), pixel.g.clone(), pixel.b.clone());
        }
    }
    return color_image;
}

struct Diagram {
    name: String,
    image: ColorImage,
}

impl Diagram {
    pub fn new(name: String, image: ColorImage) -> Self {
        Self { name, image }
    }
}

#[tokio::main]
async fn main() {
    let games_result = get_matches().await;

    let mut games = games_result.unwrap_or_else(|_| vec![]);

    games.sort_by(|x1, x2| {
        if x1.start_time > x2.start_time {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let mut app = MyApp::new(games);

    let bmp = calender_visualizer(&app.games);
    app.images.push(Diagram::new("Calender".to_string(), convert_bmp_to_egui_image(&bmp)));

    let bmp = crack_visualizer(&app.games);
    app.images.push(Diagram::new("Crack".to_string(), convert_bmp_to_egui_image(&bmp)));

    let bmp = calender_bar_visualizer(&app.games);
    app.images.push(Diagram::new("Calender Bar".to_string(), convert_bmp_to_egui_image(&bmp)));

    let bmp = line_visualizer(&app.games);
    app.images.push(Diagram::new("Line".to_string(), convert_bmp_to_egui_image(&bmp)));

    let bmp = calender_bar_2_visualizer(&app.games);
    app.images.push(Diagram::new("Calender bar 2".to_string(), convert_bmp_to_egui_image(&bmp)));

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
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(Visuals::light());

            ui.vertical_centered(|ui| {
                let available_size = ui.available_size() * 0.9f32;

                ScrollArea::both()
                    .show(ui, |ui| {
                        ui.vertical_centered_justified(|ui| {
                            for (index, diagram) in self.images.iter().enumerate() {
                                let image: &ColorImage = &diagram.image;
                                let texture = ui.ctx().load_texture(format!("image_{}", index), image.clone(), Default::default());

                                let aspect_ratio = image.width() as f32 / image.height() as f32;

                                let base_scaled_height = available_size.x / aspect_ratio;
                                let scaled_height = base_scaled_height;
                                let image_size = if scaled_height > available_size.y {
                                    Vec2::new(available_size.y * aspect_ratio, available_size.y)
                                } else {
                                    Vec2::new(available_size.x, scaled_height)
                                };

                                ui.add_space(available_size.y * 0.05f32);
                                ui.label(diagram.name.clone());
                                ui.image(texture.id(), image_size);
                            }
                        });
                    });
            });
        });
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
