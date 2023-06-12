use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Arrays {
    prefixes: Vec<String>,
    suffixes: Vec<String>,
    appellations: Vec<String>,
    super_uniques: Vec<String>,
}

fn main() {
    let probability = rand::thread_rng().gen_range(0..100);
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Please provide a path");

    let arrays = read_arrays_from_file(file_path).unwrap_or_else(|err| {
        panic!("Error while reading the file: {}", err);
    });

    match probability {
        98..=99 => {
            println!("{}", rand_name(&arrays.super_uniques));
        }
        60..=97 => {
            println!(
                "{} {} {}",
                rand_name(&arrays.prefixes),
                rand_name(&arrays.suffixes),
                rand_name(&arrays.appellations)
            );
        }
        _ => {
            println!(
                "{} {}",
                rand_name(&arrays.prefixes),
                rand_name(&arrays.suffixes)
            );
        }
    }
}

fn read_arrays_from_file<P>(file_path: P) -> io::Result<Arrays>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut prefixes = Vec::new();
    let mut suffixes = Vec::new();
    let mut appellations = Vec::new();
    let mut super_uniques = Vec::new();

    let mut current_array: Option<&mut Vec<String>> = None;

    for line_result in reader.lines() {
        let line = line_result?;
        if line.starts_with("[PREFIXES]") {
            current_array = Some(&mut prefixes);
        } else if line.starts_with("[SUFFIXES]") {
            current_array = Some(&mut suffixes);
        } else if line.starts_with("[APPELLATIONS]") {
            current_array = Some(&mut appellations);
        } else if line.starts_with("[SUPER_UNIQUES]") {
            current_array = Some(&mut super_uniques);
        } else if let Some(ref mut array) = current_array {
            array.push(line);
        }
    }

    Ok(Arrays {
        prefixes,
        suffixes,
        appellations,
        super_uniques,
    })
}

fn rand_name(input: &[String]) -> &str {
    let index = rand::thread_rng().gen_range(0..input.len());
    &input[index]
}
