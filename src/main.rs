extern crate csv;
extern crate itertools;
extern crate tfidf;

use std::char;
use itertools::Itertools;
use std::collections::HashMap;
use tfidf::{TfIdf, TfIdfDefault};

fn non_word(c: char) -> bool {
    !char::is_alphabetic(c) && !char::is_whitespace(c) && c != '-'
}

fn wf(s: String) -> HashMap<String, u32> {
    s.replace(non_word, "")
        .split_whitespace()
        .filter(|s| *s != "-")
        .map(|s| s.chars().flat_map(char::to_lowercase).collect::<String>())
        .fold(HashMap::new(), |mut acc, i| {
            *acc.entry(i).or_insert(0) += 1;
            acc
        })
}

fn main() {
    let mut rdr = csv::Reader::from_file("./data/eig.csv").unwrap();
    let mut docs = Vec::new();
    for record in rdr.decode() {
        let (title, desc, _): (String, String, String) = record.unwrap();
        let mut doc = Vec::new();
        let word_freq = wf(desc);
        for (word, freq) in word_freq.iter() {
            doc.push((String::from(word.clone()), freq.clone() as usize));
            println!("({}, {})", word, freq);
        }
        docs.push(doc);
    }
    TfIdfDefault::tfidf("a", &docs[0], docs.iter());
}
