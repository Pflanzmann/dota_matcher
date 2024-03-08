use bmp::{Image, Pixel};
use chrono::{Datelike, Duration, TimeZone, Utc};

use crate::Game;

pub fn line_visualizer(games: &Vec<Game>) -> Image {
    let mut games = games.clone();

    let first_match_time = games.first().unwrap().start_time.unwrap();
    let last_match_time = games.last().unwrap().start_time.unwrap();

    let start_date = Utc.timestamp_opt(first_match_time, 0).unwrap();
    let end_date = Utc.timestamp_opt(last_match_time, 0).unwrap();

    let days_played = (end_date - start_date).num_days();

    let height = 1000;
    let mut bitmap = Image::new(
        days_played as u32 + 1,
        height,
    );

    let mut x = 0;

    let mut dt = start_date;
    while end_date > dt {
        let mut games_played = 0;
        let mut games_won = 0;

        let mut current_game = *games.first().unwrap();
        let mut current_game_day = Utc.timestamp_opt(current_game.start_time.unwrap(), 0).unwrap();
        let mut player_radiant = current_game.player_slot.unwrap() < 100;
        let mut player_won = (current_game.radiant_win.unwrap() && player_radiant) || (!current_game.radiant_win.unwrap() && !player_radiant);

        while current_game_day.num_days_from_ce() == dt.num_days_from_ce() && games.len() > 2 {
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

        if games_played > 0 {
            let green_color = 255 / games_played * games_won;
            let played_color = Pixel::new(0, 0, 0);

            for y in 0..height {
                bitmap.set_pixel(x, y, played_color);
            }
        } else {
            let not_played_color = Pixel::new(255, 255, 255);

            for y in 0..height {
                bitmap.set_pixel(x, y, not_played_color);
            }
        }

        dt += Duration::days(1);
        x += 1;
    }

    bitmap.save("visuals/barcode.bmp");
    bitmap
}
