static WORDS: &str = include_str!("./words_alpha.txt");

use std::hash::{Hash, Hasher};
use std::collections::hash_map::{HashMap, Entry};
use fnv::FnvHasher;
use fxhash::FxHasher;

fn collider<H: Hasher + Default>() {
    let mut hashes: HashMap<u64, &str> = HashMap::with_capacity(370100);
    let mut collisions: Vec<(&str, &str)> = Vec::new();

    for word in WORDS.lines() {
        let mut hasher = H::default();
        hasher.write(word.as_bytes());
        let hash = hasher.finish();

        match hashes.entry(hash) {
            Entry::Occupied(occupied) => {
                collisions.push((occupied.get(), word));
            },
            Entry::Vacant(vacant) => {
                vacant.insert(word);
            },
        }
    }

    println!("Got {} collisions!", collisions.len());

    for (a, b) in collisions.iter() {
        println!("{} collides with {}", a, b);
    }
}

fn main() {
    println!("\n\nFNV-1a:");
    collider::<FnvHasher>();
    println!("\n\nFxHash:");
    collider::<FxHasher>();
}
