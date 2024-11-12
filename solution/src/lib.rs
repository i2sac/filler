use std::cmp;
use std::io::{stdin, BufRead};
use rayon::prelude::*;
use rust_linalg::Matrix;

#[derive(Debug, Clone)]
pub struct Game {
    pub my_player: usize,
    pub map_width: usize,
    pub map_height: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            my_player: 1,
            map_width: 0,
            map_height: 0,
        }
    }
}

pub fn turn(game: &mut Game) {
    let input = stdin().lock().lines();
    let msg = "PARSING ERROR";
    let mut map_lines: Vec<String> = vec![];
    let mut piece_lines: Vec<String> = vec![];
    let mut map_recording = false;
    let mut piece_recording = -1;
    let (mut piece_w, mut piece_h) = (0, 0);

    for line in input {
        let txt = line.unwrap();
        if let Some(p_cmd) = txt.strip_prefix("$$$ exec p") {
            game.my_player = p_cmd.split_at(1).0.parse::<usize>().expect(&msg);
        } else if let Some(size_raw) = txt.strip_prefix("Anfield ") {
            let size = size_raw
                .replace(":", "")
                .split(" ")
                .map(|val| val.parse::<usize>().expect(&msg))
                .collect::<Vec<usize>>();
            (game.map_width, game.map_height) = (size[0], size[1]);
            map_recording = true;
        } else if let Some(piece_size) = txt.strip_prefix("Piece ") {
            let size = piece_size
                .replace(":", "")
                .split(" ")
                .map(|val| val.parse::<usize>().expect(&msg))
                .collect::<Vec<usize>>();
            (piece_w, piece_h) = (size[0], size[1]);
            map_recording = false;
            piece_recording = piece_h as i32 + 1;
        }

        if piece_recording > 0 {
            piece_lines.push(txt.clone());
            piece_recording -= 1;
        }

        if map_recording && !txt.starts_with("Anfield ") && !txt.starts_with("   ") {
            map_lines.push(txt.clone().split_at(4).1.to_string());
        }

        if piece_recording == 0 {
            break;
        }
    }

    piece_lines = piece_lines
        .into_iter()
        .filter(|line| !line.starts_with("Piece "))
        .collect();

    if !map_lines.is_empty() {
        let (player_chars, foe_chars) = match game.my_player {
            1 => (['@', 'a'], ['$', 's']),
            _ => (['$', 's'], ['@', 'a']),
        };

        let mut foe_pos: Vec<(usize, usize)> = vec![];

        for (y, line) in map_lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if foe_chars.contains(&c) {
                    foe_pos.push((x, y));
                }
            }
        }

        let (mut min_x, mut max_x, mut min_y, mut max_y) = (game.map_width, 0, game.map_height, 0);

        let map_vecs = map_lines
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if player_chars.contains(&c) {
                            min_x = cmp::min(min_x, x);
                            max_x = cmp::max(max_x, x);
                            min_y = cmp::min(min_y, y);
                            max_y = cmp::max(max_y, y);
                            1
                        } else if foe_chars.contains(&c) {
                            3
                        } else {
                            0
                        }
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let piece_vecs = piece_lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| if c == 'O' { 1 } else { 0 })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        min_x = cmp::max(0, min_x as i32 - piece_w as i32) as usize;
        min_y = cmp::max(0, min_y as i32 - piece_h as i32) as usize;

        let pos_ok: Vec<(usize, usize)> = (min_y..=max_y)
            .into_par_iter()
            .flat_map(|y| {
                (min_x..=max_x)
                    .filter_map(|x| {
                        if x + piece_w <= game.map_width && y + piece_h <= game.map_height {
                            let piece_matrix = Matrix::new(piece_vecs.clone());
                            let map_window = Matrix::new(
                                map_vecs[y..(y + piece_h)]
                                    .iter()
                                    .map(|line| line[x..(x + piece_w)].to_vec())
                                    .collect(),
                            );
                            let _mat = piece_matrix + map_window;
                            let good_overlay = _mat.data.iter().flatten().filter(|&&v| v == 2).count();
                            let bad_overlay = _mat.data.iter().flatten().filter(|&&v| v > 2).count();
                            if good_overlay == 1 && bad_overlay == 0 {
                                Some((x, y))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        if !pos_ok.is_empty() {
            let output_pos = closer_to_enemy(&pos_ok, &foe_pos);
            println!("{} {}", output_pos.0, output_pos.1);
        } else {
            println!("0 0");
        }
    }
}

fn closer_to_enemy(pos_ok: &[(usize, usize)], foe_pos: &[(usize, usize)]) -> (usize, usize) {
    pos_ok
        .par_iter()
        .min_by_key(|&&pos| {
            foe_pos
                .iter()
                .map(|&foe| (pos.0 as i32 - foe.0 as i32).abs() + (pos.1 as i32 - foe.1 as i32).abs())
                .min()
                .unwrap_or(usize::MAX as i32)
        })
        .copied()
        .unwrap_or(pos_ok[0])
}
