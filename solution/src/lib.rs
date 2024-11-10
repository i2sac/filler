use std::cmp;
use std::io::{BufRead, Write};
use std::{fs::File, io::stdin};

use rust_linalg::Matrix;

#[derive(Debug, Clone)]
pub struct Game {
    pub output_file: String,
    pub my_player: usize,
    pub map_width: usize,
    pub map_height: usize,
    pub turn_number: usize,
}

impl Game {
    pub fn new(file_name: &str) -> Self {
        Game {
            output_file: file_name.to_string(),
            my_player: 1,
            map_width: 0,
            map_height: 0,
            turn_number: 0,
        }
    }
}

pub fn turn(game: &mut Game) {
    let mut f = File::options()
        .append(true)
        .open(&game.output_file)
        .unwrap();
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

        writeln!(&mut f, "{}", txt).unwrap();

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

        let mut foe_pos: Vec<(usize, usize)> = vec![];

        for (y, line) in map_lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if foe_chars.contains(&c) {
                    foe_pos.push((x, y));
                }
            }
        }

        let map_vecs = map_lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if player_chars.contains(&c) {
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

        for y in 0..game.map_height {
            for x in 0..game.map_width {
                if x + piece_w < game.map_width && y + piece_h < game.map_height {
                    let piece_matrix = Matrix::new(piece_vecs.clone());
                    let map_window = Matrix::new(
                        map_vecs[y..(y + piece_h)]
                            .into_iter()
                            .map(|line| line[x..(x + piece_w)].to_vec())
                            .collect(),
                    );
                    _mat = piece_matrix + map_window;
                    writeln!(&mut f, "Conv res: {:?}", _mat.data.iter().flatten().collect::<Vec<&usize>>()).unwrap();
                    let good_overlay = _mat.data.iter().flatten().filter(|v| **v == 2).count();
                    let bad_overlay = _mat.data.iter().flatten().filter(|v| **v > 2).count();
                    if good_overlay == 1 && bad_overlay == 0 {
                        pos_ok.push((x, y));
                    }
                }
            }
        }

        // let mut f = File::options()
        //     .append(true)
        //     .open(&game.output_file)
        //     .unwrap();
        // writeln!(&mut f, "=================================================").unwrap();
        // writeln!(&mut f, "Map:").unwrap();
        // for line in map_vecs {
        //     writeln!(&mut f, "{:?}", line).unwrap();
        // }
        // writeln!(&mut f, "=================================================").unwrap();

        // if piece_lines.len() > 0 {
        //     let mut f = File::options()
        //         .append(true)
        //         .open(&game.output_file)
        //         .unwrap();
        //     writeln!(&mut f, "=================================================").unwrap();
        //     writeln!(&mut f, "Piece {} {}:", piece_w, piece_h).unwrap();
        //     for line in piece_vecs {
        //         writeln!(&mut f, "{:?}", line).unwrap();
        //     }
        //     writeln!(&mut f, "=================================================").unwrap();
        //     writeln!(&mut f, "=================================================").unwrap();
        //     writeln!(&mut f, "Pos OK:").unwrap();
        //     for pos in pos_ok.iter() {
        //         writeln!(&mut f, "{:?}", pos).unwrap();
        //     }
        //     writeln!(&mut f, "=================================================").unwrap();
        // }

        if pos_ok.len() > 0 {
            let mut min_dist = 2 * cmp::min(game.map_width, game.map_height);
            let mut ouput_pos = pos_ok[0];
            for pos1 in pos_ok.iter() {
                for pos2 in foe_pos.iter() {
                    let dist = ((pos1.0 as i32 - pos2.0 as i32).abs() + (pos1.1 as i32 - pos2.1 as i32).abs()) as usize;
                    if dist <= min_dist {
                        min_dist = dist;
                        ouput_pos = *pos1;
                    }
                }
            }
            println!("{} {}", ouput_pos.0, ouput_pos.1);
            return;
        }
    }

    // Affichage de ma réponse. J'ai simulé 3 3 pour passer le tour
    println!("0 0");
}
