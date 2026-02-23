extern crate qdrant_rust_stemmers;

use qdrant_rust_stemmers::{Algorithm, Stemmer};

fn main() {
    let en_stemmer = Stemmer::create(Algorithm::English);
    assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");    
}
