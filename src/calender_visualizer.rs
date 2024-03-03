use std::ops::Div;

use bmp::{Image, Pixel};
use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use egui::{Color32, ColorImage};

use crate::Game;

pub fn calender(games: &mut Vec<Game>) -> ColorImage {
    let first_match_time = games.first().unwrap().start_time.unwrap();
    let last_match_time = games.last().unwrap().start_time.unwrap();

    let start_date = Utc.timestamp_opt(first_match_time, 0).unwrap();
    let end_date = Utc.timestamp_opt(Utc::now().timestamp(), 0).unwrap();

    let days_played = (end_date - start_date).num_days();
    let weeks = days_played.div(7) + 1;

    let height_multiplier: u32 = 30;
    let width_multiplier: u32 = 30;
    let height = 7 * height_multiplier;
    let width = (weeks * width_multiplier as i64) as u32;

    let mut bitmap = Image::new(width, height);
    let mut color_image = ColorImage::new([width as usize, height as usize], Color32::TRANSPARENT);

    let mut date_time = start_date;
    let mut x = 0;
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
            if player_won {
                games_won += 1;
            }
        }

        let weekday = date_time.weekday();

        if games_played != 0 {
            let green_color = 255 / games_played * games_won;
            let played_color = Pixel::new(255 - green_color, green_color, 0);
            let black = Pixel::new(0, 0, 0);

            for local_x in 0..width_multiplier {
                for local_y in 0..height_multiplier {
                    bitmap.set_pixel(x * width_multiplier + local_x, weekday.num_days_from_monday() * height_multiplier + local_y, black);
                    color_image[((x * width_multiplier + local_x) as usize, (weekday.num_days_from_monday() * height_multiplier + local_y) as usize)] = Color32::BLACK;
                }
            }
        } else {
            let not_played_color = Pixel::new(255, 255, 255);

            for local_x in 0..width_multiplier {
                for local_y in 0..height_multiplier {
                    bitmap.set_pixel(x * width_multiplier + local_x, weekday.num_days_from_monday() * height_multiplier + local_y, not_played_color);
                    color_image[((x * width_multiplier + local_x) as usize, (weekday.num_days_from_monday() * height_multiplier + local_y) as usize)] = Color32::TRANSPARENT;
                }
            }
        }

        if weekday == Weekday::Sun {
            x += 1;
        }

        date_time += Duration::days(1);
    }

    bitmap.save("visuals/calender.bmp");
    return color_image;
}
