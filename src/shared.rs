pub fn choose_random_from_filename(name: &str) -> String {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    use rand::prelude::IteratorRandom;


    let f = File::open(name).unwrap();

    let result = BufReader::new(f)
        .lines()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .unwrap();
    
    result
}
