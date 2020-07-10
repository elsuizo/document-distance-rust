//-------------------------------------------------------------------------
// @file main.rs
//
// @date 07/10/20 11:35:41
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
//
// @detail
//
// Licence MIT:
// Copyright <year> <Martin Noblia>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.  THE SOFTWARE IS PROVIDED
// "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT
// LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
// PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//-------------------------------------------------------------------------
/// This program compute the "distance" between two files as the angle between
/// their word frequency vectors (in radians)
// TODO(elsuizo:2020-07-10): list of
// - [X] split the document into words
//      - [X] first read the file
// - [X] compute the frequencies of the words
// - [  ] compute the dot product

extern crate regex;

use std::error::Error;
use std::path::Path;
use std::ops::{Mul};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Debug, Clone)]
struct DocumentVector<'a> {
    file_name: &'a str,
    statistics: BTreeMap<String, usize>
}

impl<'a> DocumentVector<'a> {
    fn new(file_name: &'a str, statistics: BTreeMap<String, usize>) -> Self {
        DocumentVector{file_name, statistics}
    }
}

impl<'a> Mul for DocumentVector<'a> {
    type Output = usize;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut sum:usize = 0;
        for (k, v) in &self.statistics {
            if rhs.statistics.contains_key(k) {
                sum += v * rhs.statistics[k];
            }
        }
        sum
    }
}

// TODO(elsuizo:2020-07-09): no era que con AsRef<Path> ya se solucionaba todo???
fn list_files<P>(folder_path: P) -> std::io::Result<Vec<String>>
    where P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>
{

    let mut result: Vec<String> = Vec::new();
    let path = Path::new(&folder_path);

    for file_result in path.read_dir()? {
        let file = file_result?;
        result.push(file.file_name().into_string().unwrap());
    }

    Ok(result)
}

fn read_text_file(path: &Path) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("error reading the line"))
        .collect();
    Ok(lines)
}

fn count_words(file_input: Vec<String>) -> std::io::Result<BTreeMap<String, usize>> {

    let mut counts = BTreeMap::new();
    // NOTE(elsuizo:2020-04-30): estaria bueno saber bien que es lo que filtra este regex
    let word_regex = Regex::new(r"(?i)[a-z']+").expect("Could not compile regex");

    for line in file_input {
        let words = word_regex
            .find_iter(&line)
            .map(|m| &line[m.start()..m.end()]);
        for word in words.map(|s| s.to_lowercase()) {
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    Ok(counts)
}

fn main() -> Result<(), Box<dyn Error>> {

    let files_dir = "files_dot_test";
    // read all the files in the given folder
    let files = list_files(files_dir)?;

    let mut docs: Vec<DocumentVector> = Vec::new();
    for file_name in &files {
        let path = Path::new(files_dir);
        let file = read_text_file(&path.join(file_name))?;
        let statistics = count_words(file)?;
        docs.push(DocumentVector::new(&file_name, statistics));
    }

    let d1 = &docs[0];
    let d2 = &docs[1];

    println!("sum: {}", d1.clone() * d2.clone());

    Ok(())
}
