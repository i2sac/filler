use std::io::{BufRead, Write};
use std::{fs::File, io::stdin};

pub fn turn(file_name: &str, p: &mut i32, width: &mut usize, height: &mut usize) {
    let input = stdin().lock().lines();
    let mut map_lines: Vec<String> = vec![];
    let mut piece_lines: Vec<String> = vec![];
    let mut map_recording = false;
    let mut piece_recording = false;
    
    for line in input {
        let mut f = File::options().append(true).open(file_name).unwrap();
        let txt = line.unwrap();
        let msg = "PARSING ERROR";
        let (mut piece_w, mut piece_h) = (0, 0);

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
            piece_recording = true;
        }

        if map_recording && !txt.starts_with("Anfield ") && !txt.starts_with("   ") {
            map_lines.push(txt.clone().split_at(4).1.to_string());
        } else if piece_recording && !txt.starts_with("Piece ") && !txt.starts_with("   ") {
            piece_lines.push(txt.clone());
        }

        writeln!(&mut f, "{}", txt).unwrap();
        println!("3 3");
    }

    piece_recording = false;

    if map_lines.len() > 0 {
        let mut f = File::options().append(true).open(file_name).unwrap();
        writeln!(&mut f, "Map:").unwrap();
        for line in map_lines {
            writeln!(&mut f, "{}", line).unwrap();
        }
        writeln!(&mut f, "=============================================================").unwrap();
    }

    if piece_lines.len() > 0 {
        let mut f = File::options().append(true).open(file_name).unwrap();
        writeln!(&mut f, "Piece:").unwrap();
        for line in piece_lines {
            writeln!(&mut f, "{}", line).unwrap();
        }
        writeln!(&mut f, "=============================================================").unwrap();
    }
}
