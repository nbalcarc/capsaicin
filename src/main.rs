use levenshtein;

use std::{fs, collections::HashSet};


fn kmers_from_filestring<'a>(contents: &'a str) -> Vec<&'a str> {
    let k = 32;
    let blocks: Vec<&str> = contents.split('\n').filter(|x| !x.starts_with('>')).collect();
    let kmers_size = blocks.iter().map(|x| x.len() - k + 1).sum();
    let mut kmers: Vec<&str> = Vec::with_capacity(kmers_size); //initialize

    for block in blocks {
        if block.len() < k {
            continue;
        }
        for i in 0..block.len() - k + 1 {
            kmers.push(&block[i..i+k]);
        }
    }

    kmers
}


//fn kmers_preprocess(contents: &str) -> String {
//    contents
//        .split('\n')
//        .filter(|x| !x.starts_with('>'))
//        .fold(String::new(), |mut a, x| {
//            a.push_str(x);
//            a
//        })
//}


//fn kmers_from_filestring_append<'a>(contents: &'a str) -> Vec<&'a str> {
//    let k = 32;
//    let kmers_size = contents.len() - k + 1;
//    let mut kmers: Vec<&str> = Vec::with_capacity(kmers_size); //initialize
//
//    for i in 0..contents.len() - k + 1 {
//        kmers.push(&contents[i..i+k]);
//    }
//
//    kmers
//}


fn test_kmers() {
    //let file_loc = "/run/media/terrior/BeutelratteDrive/Genomes/isolated/droseraCapensis.fna";
    let file_loc = "/run/media/terrior/BeutelratteDrive/Genomes/isolated/sepiaLycidas.fna";
    let contents = fs::read_to_string(file_loc).expect("welp");
    let kmers = kmers_from_filestring(&contents);
    let hashed: HashSet<&str> = HashSet::from_iter(kmers.into_iter());

    //let preprocessed = kmers_preprocess(&contents);
    //let kmers = kmers_from_filestring_append(&preprocessed);
    //let hashed: HashSet<&str> = HashSet::from_iter(kmers.into_iter());

    println!("{}", hashed.len());
}


//fn test_levenshtein() {
//    let drosera_file = "/run/media/terrior/BeutelratteDrive/Genomes/isolated/droseraCapensis.fna";
//    let psilocybe_file = "/run/media/terrior/BeutelratteDrive/Genomes/isolated/psilocybeMexicana.fna";
//    let drosera = fs::read_to_string(drosera_file).expect("waaaaaaaaaaaaaa");
//    let psilocybe = fs::read_to_string(psilocybe_file).expect("wbbbbbbbbbbbbbb");
//    let similarity = levenshtein::levenshtein(&drosera, &psilocybe);
//    println!("{}", similarity);
//}


fn main() {
    //test_levenshtein();
    test_kmers();
}


