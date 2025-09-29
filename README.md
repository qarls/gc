# gc

Computes and returns the max GC content from a file of Rosalind's FASTA format.
- Solution for Rosalind's [Computing GC Content](https://rosalind.info/problems/fib/).
- This time, our code accepts a **file input** directly.

Regarding the input file, I'm unable to guarantee this will run standard and other FASTA formats.
Please contact me if you have any questions.

Uses clap.rs.

# Usage

```
gc <INPUT_FILE>
```

- INPUT_FILE must be of ROSALIND's FASTA spec

## Examples

### Basic Input
```
$ gc test.txt
Rosalind_9598
50.663717
```

### Redirect output to file
Please note this overwrites a file if pre-existing.
```
$ gc test.txt > test_output.txt
```

# Updates and Comments

## [2025-09-29]
While unnecessary, the first part of main.rs parses the input into a hashmap pairing the DNA ID and its strands.
My main goal is to make this part of the code more modular (for some performance cost),
knowing the FASTA format is quite ubiquitous for DNA analysis.
It's as likely Rosalind will re-use these types of files as inputs.

In storing the GC-content, I was planning to use BTreeMaps.
BTreeMaps, a collection like HashMaps, instead allows you to store keys that are then ordered.
Unfortunately, Rust doesn't implement Ord for f32 and f64's.
