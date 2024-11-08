use std::fs::File;
use ultron::turn;

fn main() {
    let file_name = "output.txt";
    let _ = File::create(file_name).unwrap();
    let mut p = 1;
    let (mut width, mut height) = (0usize, 0usize);
    loop {
        turn(&file_name, &mut p, &mut width, &mut height);
    }
}
