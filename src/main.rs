use clap::Parser;
use std::collections::HashMap;
use std::{fs, path::PathBuf};

const ABOUT_MESSAGE: &str = "Compute the GC content of Rosalind FASTA format datasets.";

#[derive(Parser)]
#[command(version, about = ABOUT_MESSAGE, long_about = None)]
struct Cli {
    /// Input file following Rosalind's FASTA spec
    input_file: PathBuf,
}

//reads multi-line base &str portion of fasta
fn to_gc(strands: &str) -> f64 {
    let mut count_cg = 0;
    let mut count_total = 0;

    for line in strands.split("\n") {
        for base in line.chars() {
            match base {
                'A' | 'T' => (),
                'C' | 'G' => count_cg += 1,
                error => panic!("Invalid character read: {}", error),
            }
            count_total += 1;
        }
    }

    count_cg as f64 / count_total as f64 * 100.
}

fn main() {
    let args = Cli::parse();
    let input = fs::read_to_string(&args.input_file).expect("Could not read this file.");

    // This section of code is made modular for Rosalind-FASTA inputs in other future challenges.
    // We define a hashmay to hold an individual DNA string ID and the value strands (multi-line).
    let mut dna_strings = HashMap::new();
    {
        let iter: Vec<&str> = input.split('>').collect();

        // This separates the DNA strings into the ID and its strands.
        // 0 index is an empty item due to split.
        for i in 1..iter.len() {
            let (dna_id, strands) = iter[i].split_once("\n").unwrap();
            dna_strings.insert(dna_id, strands);
        }
    }

    // Apparently, Rust doesn't allow Ord for floats, so we can't use BTreeMaps.
    // So, here's one way to find keep the max GC for a dna_id.
    let mut dna_id_max = String::new();
    let mut gc_max: f64 = 0.;
    for (dna_id, strands) in dna_strings {
        let gc = to_gc(strands);
        if gc > gc_max {
            dna_id_max = dna_id.to_string();
            gc_max = gc;
        }
    }

    println!("{}\n{:.6}", dna_id_max, gc_max);
}
