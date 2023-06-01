use std::fs::File;
use std::io::{self, BufRead};

fn assemble(instruction: &str) -> u8 {
    let inst = instruction.split_whitespace().nth(0).unwrap();
    match inst {
       "noop" => 0, 
        invalid => panic!("Invalid instruction: {:?}", invalid),
    }
}

fn main() {
    // TODO: use Clap
    let path = std::env::args().nth(1).expect("no file to assemble");

    let file = File::open(path).expect("error opening file");

    for line in io::BufReader::new(file).lines() {
        if let Ok(l) = line {
            if l == "" {
                continue
            }
            let inst = assemble(l.as_str());
            print!("{}", inst);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn noop() {
        assert_eq!(assemble("noop"), 0);
    }
}
