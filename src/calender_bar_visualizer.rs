use std::ops::Div;

use bmp::{Image, Pixel};
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use egui::{Color32, ColorImage};

use crate::Game;

pub fn calender_bar_visualizer(games: &Vec<Game>) -> Image {
    let mut games = games.clone();

    let first_match_time = games.first().unwrap().start_time.unwrap();
    let last_match_time = games.last().unwrap().start_time.unwrap();

    let start_date = Utc.timestamp_opt(first_match_time, 0).unwrap();
    let end_date = Utc.timestamp_opt(last_match_time, 0).unwrap();

    let days_played = (end_date - start_date).num_days();
    let weeks = days_played.div(7) + 1;

    let height = 100;
    let half_height = height / 2;
    let width = (weeks as i64) as u32;

    let mut bitmap = Image::new(width, height);

    let mut date_time = start_date;
    let mut x = 0;
    let mut games_played_per_week = 0;

    while end_date > date_time {
        let mut games_played = 0;
        let mut games_won = 0;

        let mut current_game = *games.first().unwrap();
        let mut current_game_day = Utc.timestamp_opt(current_game.start_time.unwrap(), 0).unwrap();
        let mut player_radiant = current_game.player_slot.unwrap() < 100;
        let mut player_won = (current_game.radiant_win.unwrap() && player_radiant) || (!current_game.radiant_win.unwrap() && !player_radiant);

        while current_game_day.num_days_from_ce() == date_time.num_days_from_ce() && games.len() > 2 {
            games.remove(0);
            current_game = *games.get(0).unwrap();
            current_game_day = Utc.timestamp_opt(current_game.start_time.unwrap(), 0).unwrap();
            player_radiant = current_game.player_slot.unwrap() < 100;
            player_won = (current_game.radiant_win.unwrap() && player_radiant) || (!current_game.radiant_win.unwrap() && !player_radiant);

            games_played += 1;
            games_played_per_week += 1;

            if player_won {
                games_won += 1;
            }
        }

        let weekday = date_time.weekday();

        if weekday == Weekday::Sun {
            let not_played_color = Pixel::new(255, 255, 255);
            for y in 0..height {
                bitmap.set_pixel(x, y, not_played_color);
                bitmap.set_pixel(x, y, not_played_color);
            }

            if games_played_per_week != 0 {
                let red = Pixel::new(255, 0, 0);
                let black = Pixel::new(0, 0, 0);

                for y in 0..games_played_per_week {
                    bitmap.set_pixel(x, y, red);
                    // bitmap.set_pixel(x, half_height - y, red);
                }
            }

            x += 1;
            games_played_per_week = 0;
        }

        date_time += Duration::days(1);
    }

    bitmap.save("visuals/calender_bar_games_played.bmp");
    bitmap
}
