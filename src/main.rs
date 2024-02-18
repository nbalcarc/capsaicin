use std::{fs::{self, File}, collections::HashSet, io::{Write, self}, hash, mem::size_of};
use rayon::prelude::*;

mod minHash;


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
    //let file_loc = "/run/media/terrior/BeutelratteDrive/Genomes/isolated/sepiaLycidas.fna";
    //let file_loc = "input_files/sepiaLycidas.fna";
    let all_files = [
        "droseraCapensis.fna",
        "psilocybeMexicana.fna",
        //"ariolimaxColumbianus.fna",
        //"physcomitriumPatens.fna",
        //"sepiaLycidas.fna",
        //"bidensHawaiiensis.fna",
        //"eublepharisMacularius.fna",
        //"porcellioScaber.fna",
        //"sphagnumFallax.fna",
        //"bombusTerrestris.fna",
        //"hemileiaVastatrix.fna",
        //"takakiaLepidozioides.fna",
        //"crassostreaVirginica.fna",
        //"malasseziaRestricta.fna",
        //"sarraceniaLeucophylla.fna",
        //"cryptoproctaFerox.fna",
        //"pardosaPseudoannulata.fna",
        //"scomberScrombus.fna",
    ];
    //let file_header = "input_files/";
    let file_header = "/run/media/terrior/BeutelratteDrive/Genomes/isolated/";
    let _ = fs::create_dir("output_files"); //create output directory

    //all_files.par_iter().for_each(|file_path| {
    //    println!("starting {}", file_path);
    //    let contents = fs::read_to_string(String::from(file_header) + file_path).expect("welp").to_ascii_uppercase();
    //    let kmers = kmers_from_filestring(&contents);
    //    let hashed: HashSet<&str> = HashSet::from_iter(kmers.into_iter());

    //    let mut file = File::create(String::from("output_files/") + file_path).expect("SOMETHING WENT WRONG AAAAAA");
    //    for hash in hashed {
    //        file.write_all(hash.as_bytes()).expect("SSSSSSSSSSSS");
    //        file.write_all(b"\n").expect("NOTHING");
    //    }
    //    println!("finished {}", file_path);
    //});
    for file_path in all_files {
        let contents = fs::read_to_string(String::from(file_header) + file_path).expect("welp").to_ascii_uppercase();
        let kmers = kmers_from_filestring(&contents);
        let hashed: HashSet<&str> = HashSet::from_iter(kmers.into_iter());

        let mut file = File::create(String::from("output_files/") + file_path).expect("SOMETHING WENT WRONG AAAAAA");
        for hash in hashed {
            file.write_all(hash.as_bytes()).expect("SSSSSSSSSSSS");
            file.write_all(b"\n").expect("NOTHING");
        }
    }

    //let preprocessed = kmers_preprocess(&contents);
    //let kmers = kmers_from_filestring_append(&preprocessed);
    //let hashed: HashSet<&str> = HashSet::from_iter(kmers.into_iter());

    //println!("{}", hashed.len());
}


fn test_kmers1() {
    let all_files = [
        "droseraCapensis.fna",
        "psilocybeMexicana.fna",
        //"ariolimaxColumbianus.fna",
        //"physcomitriumPatens.fna",
        "sepiaLycidas.fna",
        //"bidensHawaiiensis.fna",
        //"eublepharisMacularius.fna",
        //"porcellioScaber.fna",
        //"sphagnumFallax.fna",
        //"bombusTerrestris.fna",
        //"hemileiaVastatrix.fna",
        //"takakiaLepidozioides.fna",
        //"crassostreaVirginica.fna",
        //"malasseziaRestricta.fna",
        //"sarraceniaLeucophylla.fna",
        //"cryptoproctaFerox.fna",
        //"pardosaPseudoannulata.fna",
        //"scomberScrombus.fna",
    ];
    //let file_header = "input_files/";
    let file_header = "/run/media/terrior/BeutelratteDrive/Genomes/isolated/";
    let _ = fs::create_dir("output_files"); //create output directory

    for file_path in all_files {
        println!("starting {}", file_path);
        //let contents = fs::read(String::from(file_header) + file_path).expect("welp").to_ascii_uppercase();
        let contents = fs::read_to_string(String::from(file_header) + file_path).expect("welp");
        println!("done reading");
        let lines = contents.split('\n');
        //let mut blocks = vec![String::from("")];

        let mut set = HashSet::new();
        let mut block = String::new();
        for line in lines {
            if line.starts_with('>') { //whenever we hit a header just push a new string
                set.insert(block);
                block = String::new();
                continue;
            }
            block += &line.to_ascii_uppercase(); //if no header then expand the last string
        }
        //let set = HashSet::new();
        println!("finished {}", file_path);
    }

}


//fn test_levenshtein() {
//    let drosera_file = "/run/media/terrior/BeutelratteDrive/Genomes/isolated/droseraCapensis.fna";
//    let psilocybe_file = "/run/media/terrior/BeutelratteDrive/Genomes/isolated/psilocybeMexicana.fna";
//    let drosera = fs::read_to_string(drosera_file).expect("waaaaaaaaaaaaaa");
//    let psilocybe = fs::read_to_string(psilocybe_file).expect("wbbbbbbbbbbbbbb");
//    let similarity = levenshtein::levenshtein(&drosera, &psilocybe);
//    println!("{}", similarity);
//}



fn get_all_file_paths(header: &str) -> Vec<String> {
    let all_files = [
        "droseraCapensis.fna",
        "psilocybeMexicana.fna",
        //"ariolimaxColumbianus.fna",
        //"physcomitriumPatens.fna",
        "sepiaLycidas.fna",
        //"bidensHawaiiensis.fna",
        //"eublepharisMacularius.fna",
        //"porcellioScaber.fna",
        //"sphagnumFallax.fna",
        //"bombusTerrestris.fna",
        //"hemileiaVastatrix.fna",
        //"takakiaLepidozioides.fna",
        //"crassostreaVirginica.fna",
        //"malasseziaRestricta.fna",
        //"sarraceniaLeucophylla.fna",
        //"cryptoproctaFerox.fna",
        //"pardosaPseudoannulata.fna",
        //"scomberScrombus.fna",
    ];
    all_files
        .par_iter()
        .map(|x| String::from(header) + x)
        .collect()
}



fn compare_kmers(kmers: &HashSet<String>, kmers1: &HashSet<String>) -> f32 {
    let inter = kmers.intersection(&kmers1).count();
    2 as f32 * inter as f32 / (kmers.len() + kmers1.len()) as f32

}


fn retrieve_kmers(file_path: &str) -> HashSet<String> {
    fs::read_to_string(file_path)
        .expect("welp")
        .split('\n')
        .map(|x| String::from(x))
        .collect()
}


//fn retrieve_kmers_compare(file_path: &str, set: &HashSet<String>) -> HashSet<String> {
//    let file = File::open(file_path).unwrap();
//    io::BufReader::new(file).lines();
//    fs::read_to_string(file_path)
//        .expect("welp")
//        .split('\n')
//        .map(|x| String::from(x))
//        .filter(|x| set.contains(x))
//        .collect()
//}


fn test_compare() {
    //let all_file_paths = get_all_file_paths("/run/media/terrior/BeutelratteDrive/Genomes/isolated/");
    let all_file_paths = get_all_file_paths("output_files/");
    println!("{}", all_file_paths[0]);
    let kmers = retrieve_kmers(&all_file_paths[0]);
    let kmers1 = retrieve_kmers(&all_file_paths[1]);
    let res = compare_kmers(&kmers, &kmers1);
    println!("COMPARE RESULT: {}%", res);
}


fn main() {
    //test_levenshtein();
    let mut thing_thing = HashSet::new();
    let x = "hi";
    let y = "hi";
    thing_thing.insert(x);
    thing_thing.insert(y);
    println!("size: {}", thing_thing.len());

    //test_compare();
    //test_kmers();
    test_kmers();
}


