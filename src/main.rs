#![feature(format_args_capture)]

use bevy::render::color::Color;
use change_case::{constant_case, sentence_case, title_case};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let colours =
        std::fs::read_to_string("colours.txt").expect("Something went wrong reading the file");

    let colours = colours
        .trim()
        .lines()
        .map(|line| {
            let mut splitter = line.trim().splitn(2, '|');
            let first = splitter.next().unwrap();
            let second = splitter.next().unwrap();
            let color = Color::hex(&second[1..=6]).unwrap();
            (
                second,
                title_case(&sentence_case(first)),
                constant_case(first),
                color.r_linear(),
                color.g_linear(),
                color.b_linear(),
            )
        })
        .collect::<Vec<_>>();

    println!("{} colours", colours.len());

    let mut colour_counts = colours.iter().fold(
        {
            let map: HashMap<&str, (usize, Vec<&str>)> = HashMap::new();
            map
        },
        |mut map, (hex, title, _, _, _, _)| {
            map.entry(hex)
                .and_modify(|(count, vec)| {
                    *count += 1;
                    vec.push(title)
                })
                .or_insert((1, vec![title]));
            map
        },
    );

    println!("{} unique colours", colour_counts.len());

    colour_counts.retain(|_, (count, _)| *count > 1);

    let duplicate_colours = colour_counts
        .into_iter()
        .map(|(hex, (_count, names))| (hex, names))
        .collect::<HashMap<_, _>>();

    println!("{duplicate_colours:#?}");

    let colours = colours.iter()
        .map(|(hex, title, constant, r, g, b)| {
            format!("/// {hex} {title}\npub const {constant}: Color = Color::rgb_linear({r:.64}, {g:.64}, {b:.64});\n")
        })
        .collect::<String>();

    // this regex removes superfluous zeroes at the end of each rgb value
    let colours = regex::Regex::new("0{2,}([,)])")
        .unwrap()
        .replace_all(&colours, "$1");

    // print!("{colours}");

    let path = Path::new("colours.rs");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the string to `file`, returns `io::Result<()>`
    match file.write_all(colours.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
