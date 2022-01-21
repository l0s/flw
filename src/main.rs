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

use std::cmp::Ordering;
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
fn score(word: &str, scores: &HashMap<char, f32>) -> f32 {
    let chars = word.chars().collect::<HashSet<char>>();
    let mut result = 0f32;
    for c in chars {
        result += scores[&c];
    }
    result
}

fn main() {
    // letter frequencies from: https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html
    let letter_scores: HashMap<char, f32> = HashMap::from([
        ('e', 56.88),
        ('a', 43.31),
        ('r', 38.64),
        ('i', 38.45),
        ('o', 36.51),
        ('t', 35.43),
        ('n', 33.92),
        ('s', 29.23),
        ('l', 27.98),
        ('c', 23.13),
        ('u', 18.51),
        ('d', 17.25),
        ('p', 16.14),
        ('m', 15.36),
        ('h', 15.31),
        ('g', 12.59),
        ('b', 10.56),
        ('f', 9.24),
        ('y', 9.06),
        ('w', 6.57),
        ('k', 5.61),
        ('v', 5.13),
        ('x', 1.48),
        ('z', 1.39),
        ('j', 1.00),
        ('q', 1.00),
    ]);
    let file = File::open("/usr/share/dict/words").expect("Unable to open dictionary");
    let lines = BufReader::new(file).lines();
    let mut candidates = lines
        .filter_map(|result| result.ok())
        .filter(|word| word.len() == 5)
        .filter(|word| word.to_lowercase().eq(word))
        .map(|word| (word.clone(), score(&word, &letter_scores)))
        .collect::<Vec<(String, f32)>>();
    candidates.sort_unstable_by(|x, y| -> Ordering {
        if x.1 > y.1 {
            Ordering::Less
        } else if y.1 > x.1 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let output = stdout();
    let mut output = output.lock();
    for candidate in candidates.iter() {
        match writeln!(output, "{}: {}", candidate.1, candidate.0) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}
