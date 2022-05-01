pub fn choose_random_from_filename(name: &str) -> String {
    use rand::prelude::IteratorRandom;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let f = File::open(name).unwrap();

    let result = BufReader::new(f)
        .lines()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .unwrap();

    result
}
