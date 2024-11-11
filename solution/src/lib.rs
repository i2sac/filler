use std::cmp;
use std::io::stdin;
use std::io::BufRead;

use rust_linalg::Matrix;

#[derive(Debug, Clone)]
pub struct Game {
    pub my_player: usize,
    pub map_width: usize,
    pub map_height: usize,
    pub turn_number: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            my_player: 1,
            map_width: 0,
            map_height: 0,
            turn_number: 0,
        }
    }
}

pub fn turn(game: &mut Game) {
    let input = stdin().lock().lines(); // Lecture des entrées
    let msg = "PARSING ERROR";
    let mut map_lines: Vec<String> = vec![];
    let mut piece_lines: Vec<String> = vec![];
    let mut map_recording = false;
    let mut piece_recording = -1;
    let (mut piece_w, mut piece_h) = (0, 0);

    // Boucle de traitement des lignes
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

    if map_lines.len() > 0 {
        let (player_chars, foe_chars) = match game.my_player {
            1 => (['@', 'a'], ['$', 's']),
            _ => (['$', 's'], ['@', 'a']),
        };

        // let corner_pos = match game.turn_number {
        //     0 => match game.my_player {
        //         1 => (game.map_width - 1, game.map_height - 1),
        //         _ => (0, 0),
        //     },
        //     1 => (game.map_width - 1, 0),
        //     _ => (0, game.map_height - 1),
        // };

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
                            if x <= min_x {
                                min_x = x;
                            }
                            if x >= max_x {
                                max_x = x;
                            }
                            if y <= min_y {
                                min_y = y;
                            }
                            if y >= max_y {
                                max_y = y;
                            }
                            return 1;
                        } else if foe_chars.contains(&c) {
                            return 3;
                        }
                        0
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let piece_vecs = piece_lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == 'O' {
                            return 1;
                        }
                        0
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let mut pos_ok: Vec<(usize, usize)> = vec![];
        let mut _mat: Matrix<usize> = Matrix::new(vec![vec![]]);

        min_x = cmp::max(0, min_x as i32 - piece_w as i32) as usize;
        min_y = cmp::max(0, min_y as i32 - piece_h as i32) as usize;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x + piece_w <= game.map_width && y + piece_h <= game.map_height {
                    let piece_matrix = Matrix::new(piece_vecs.clone());
                    let map_window = Matrix::new(
                        map_vecs[y..(y + piece_h)]
                            .into_iter()
                            .map(|line| line[x..(x + piece_w)].to_vec())
                            .collect(),
                    );
                    _mat = piece_matrix + map_window;
                    let good_overlay = _mat.data.iter().flatten().filter(|v| **v == 2).count();
                    let bad_overlay = _mat.data.iter().flatten().filter(|v| **v > 2).count();
                    if good_overlay == 1 && bad_overlay == 0 {
                        pos_ok.push((x, y));
                    }
                }
            }
        }

        if pos_ok.len() > 0 {
            // Utiliser la stratégie appropriée en fonction du tour
            // let output_pos = match game.turn_number {
            //     0 => closer_to_enemy(&pos_ok, &foe_pos),
            //     1 => closer_to_corner(&pos_ok, corner_pos),
            //     _ => closer_to_center(&pos_ok, game.map_width, game.map_height),
            // };
            let output_pos = closer_to_enemy(&pos_ok, &foe_pos);

            println!("{} {}", output_pos.0, output_pos.1);
        } else {
            println!("0 0");
        }
    }
}

fn closer_to_enemy(pos_ok: &[(usize, usize)], foe_pos: &[(usize, usize)]) -> (usize, usize) {
    let mut min_dist = usize::MAX;
    let mut output_pos = pos_ok[0];

    for &pos1 in pos_ok.iter() {
        for &pos2 in foe_pos.iter() {
            let dist = ((pos1.0 as i32 - pos2.0 as i32).abs()
                + (pos1.1 as i32 - pos2.1 as i32).abs()) as usize;
            if dist < min_dist {
                min_dist = dist;
                output_pos = pos1;
            }
        }
    }

    output_pos
}


// fn closer_to_corner(pos_ok: &[(usize, usize)], corner_pos: (usize, usize)) -> (usize, usize) {
//     let mut min_dist = usize::MAX;
//     let mut output_pos = pos_ok[0];

//     for &pos in pos_ok.iter() {
//         let dist = ((pos.0 as i32 - corner_pos.0 as i32).abs()
//             + (pos.1 as i32 - corner_pos.1 as i32).abs()) as usize;
//         if dist < min_dist {
//             min_dist = dist;
//             output_pos = pos;
//         }
//     }

//     output_pos
// }

// fn closer_to_center(
//     pos_ok: &[(usize, usize)],
//     map_width: usize,
//     map_height: usize,
// ) -> (usize, usize) {
//     let center_pos = (map_width / 2, map_height / 2);
//     let mut min_dist = usize::MAX;
//     let mut output_pos = pos_ok[0];

//     for &pos in pos_ok.iter() {
//         let dist = ((pos.0 as i32 - center_pos.0 as i32).abs()
//             + (pos.1 as i32 - center_pos.1 as i32).abs()) as usize;
//         if dist < min_dist {
//             min_dist = dist;
//             output_pos = pos;
//         }
//     }

//     output_pos
// }
