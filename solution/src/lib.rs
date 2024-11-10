use std::io::{BufRead, Write};
use std::{fs::File, io::stdin};

use regex::Regex;

pub fn turn(file_name: &str, p: &mut i32, width: &mut usize, height: &mut usize) {
    let mut f = File::options().append(true).open(file_name).unwrap();
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
            *p = p_cmd.split_at(1).0.parse::<i32>().expect(&msg);
        }
        else if let Some(size_raw) = txt.strip_prefix("Anfield ") {
            let size = size_raw.replace(":", "")
                .split(" ")
                .map(|val| val.parse::<usize>().expect(&msg))
                .collect::<Vec<usize>>();
            (*width, *height) = (size[0], size[1]);
            map_recording = true;
        } else if let Some(piece_size) = txt.strip_prefix("Piece ") {
            let size = piece_size.replace(":", "")
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

    if map_lines.len() > 0 {
        let mut f = File::options().append(true).open(file_name).unwrap();
        writeln!(&mut f, "=============================================================").unwrap();
        writeln!(&mut f, "Map:").unwrap();
        for line in map_lines {
            writeln!(&mut f, "{}", line).unwrap();
        }
        writeln!(&mut f, "=============================================================").unwrap();
    }

    if piece_lines.len() > 0 {
        let mut f = File::options().append(true).open(file_name).unwrap();
        writeln!(&mut f, "=============================================================").unwrap();
        writeln!(&mut f, "Piece:").unwrap();
        for line in piece_lines {
            writeln!(&mut f, "{}", line).unwrap();
        }
        writeln!(&mut f, "=============================================================").unwrap();
    }

    // Affichage de ma réponse. J'ai simulé 3 3 pour passer le tour
    println!("3 3");
}
