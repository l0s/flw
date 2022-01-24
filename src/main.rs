/*
MIT License

Copyright (c) 2022 Carlos Macasaet

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use clap::Parser;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::stdout;
use std::io::Write;
use std::io::{BufRead, BufReader};

/// Determine how likely a word is to narrow the possible target words
///
/// Parameters
/// - `word` - a candidate to score
/// - `scores` - the proportions of each letter. The lowest value is 1.0. Other values indicate how
///              much more common that letter is than a letter with a score of 1.0.
///
/// Return: A value greater than or equal to 1.0. The higher the number, the more likely it is to
///         narrow the list of possible target words.
fn score(word: &str, scores: &HashMap<char, Decimal>) -> Decimal {
    let chars = word.chars().collect::<HashSet<char>>();
    let mut result = dec!(0.00);
    for c in chars {
        result += scores[&c];
    }
    result
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(short, long, default_value = "/usr/share/dict/words")]
    word_list: String,

    #[clap(short, long, default_value_t = 5)]
    length: u8,
}

fn main() {
    let arguments = Arguments::parse();
    // letter frequencies from: https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html
    let letter_scores: HashMap<char, Decimal> = HashMap::from([
        ('e', dec!(56.88)),
        ('a', dec!(43.31)),
        ('r', dec!(38.64)),
        ('i', dec!(38.45)),
        ('o', dec!(36.51)),
        ('t', dec!(35.43)),
        ('n', dec!(33.92)),
        ('s', dec!(29.23)),
        ('l', dec!(27.98)),
        ('c', dec!(23.13)),
        ('u', dec!(18.51)),
        ('d', dec!(17.25)),
        ('p', dec!(16.14)),
        ('m', dec!(15.36)),
        ('h', dec!(15.31)),
        ('g', dec!(12.59)),
        ('b', dec!(10.56)),
        ('f', dec!(9.24)),
        ('y', dec!(9.06)),
        ('w', dec!(6.57)),
        ('k', dec!(5.61)),
        ('v', dec!(5.13)),
        ('x', dec!(1.48)),
        ('z', dec!(1.39)),
        ('j', dec!(1.00)),
        ('q', dec!(1.00)),
    ]);
    let file = File::open(arguments.word_list.clone())
        .unwrap_or_else(|_| panic!("Unable to open dictionary: {}", arguments.word_list));
    let lines = BufReader::new(file).lines();
    let mut candidates = lines
        .filter_map(|result| result.ok())
        .filter(|word| word.len() == arguments.length as usize)
        .filter(|word| word.to_lowercase().eq(word))
        .map(|word| (word.clone(), score(&word, &letter_scores)))
        .collect::<Vec<(String, Decimal)>>();
    candidates.sort_unstable_by_key(|tuple| -tuple.1);
    let output = stdout();
    let mut output = output.lock();
    for candidate in candidates.iter() {
        if writeln!(output, "{}: {}", candidate.1, candidate.0).is_err() {
            break;
        }
    }
}
