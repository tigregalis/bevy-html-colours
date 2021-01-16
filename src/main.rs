#![feature(format_args_capture)]

use bevy::render::color::Color;
use change_case::{constant_case, sentence_case, title_case};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const COLOURS: &str = r#"AliceBlue|#F0F8FF
AntiqueWhite|#FAEBD7
Aqua|#00FFFF
Aquamarine|#7FFFD4
Azure|#F0FFFF
Beige|#F5F5DC
Bisque|#FFE4C4
Black|#000000
BlanchedAlmond|#FFEBCD
Blue|#0000FF
BlueViolet|#8A2BE2
Brown|#A52A2A
BurlyWood|#DEB887
CadetBlue|#5F9EA0
Chartreuse|#7FFF00
Chocolate|#D2691E
Coral|#FF7F50
CornflowerBlue|#6495ED
Cornsilk|#FFF8DC
Crimson|#DC143C
Cyan|#00FFFF
DarkBlue|#00008B
DarkCyan|#008B8B
DarkGoldenRod|#B8860B
DarkGray|#A9A9A9
DarkGrey|#A9A9A9
DarkGreen|#006400
DarkKhaki|#BDB76B
DarkMagenta|#8B008B
DarkOliveGreen|#556B2F
DarkOrange|#FF8C00
DarkOrchid|#9932CC
DarkRed|#8B0000
DarkSalmon|#E9967A
DarkSeaGreen|#8FBC8F
DarkSlateBlue|#483D8B
DarkSlateGray|#2F4F4F
DarkSlateGrey|#2F4F4F
DarkTurquoise|#00CED1
DarkViolet|#9400D3
DeepPink|#FF1493
DeepSkyBlue|#00BFFF
DimGray|#696969
DimGrey|#696969
DodgerBlue|#1E90FF
FireBrick|#B22222
FloralWhite|#FFFAF0
ForestGreen|#228B22
Fuchsia|#FF00FF
Gainsboro|#DCDCDC
GhostWhite|#F8F8FF
Gold|#FFD700
GoldenRod|#DAA520
Gray|#808080
Grey|#808080
Green|#008000
GreenYellow|#ADFF2F
HoneyDew|#F0FFF0
HotPink|#FF69B4
IndianRed|#CD5C5C
Indigo|#4B0082
Ivory|#FFFFF0
Khaki|#F0E68C
Lavender|#E6E6FA
LavenderBlush|#FFF0F5
LawnGreen|#7CFC00
LemonChiffon|#FFFACD
LightBlue|#ADD8E6
LightCoral|#F08080
LightCyan|#E0FFFF
LightGoldenRodYellow|#FAFAD2
LightGray|#D3D3D3
LightGrey|#D3D3D3
LightGreen|#90EE90
LightPink|#FFB6C1
LightSalmon|#FFA07A
LightSeaGreen|#20B2AA
LightSkyBlue|#87CEFA
LightSlateGray|#778899
LightSlateGrey|#778899
LightSteelBlue|#B0C4DE
LightYellow|#FFFFE0
Lime|#00FF00
LimeGreen|#32CD32
Linen|#FAF0E6
Magenta|#FF00FF
Maroon|#800000
MediumAquaMarine|#66CDAA
MediumBlue|#0000CD
MediumOrchid|#BA55D3
MediumPurple|#9370DB
MediumSeaGreen|#3CB371
MediumSlateBlue|#7B68EE
MediumSpringGreen|#00FA9A
MediumTurquoise|#48D1CC
MediumVioletRed|#C71585
MidnightBlue|#191970
MintCream|#F5FFFA
MistyRose|#FFE4E1
Moccasin|#FFE4B5
NavajoWhite|#FFDEAD
Navy|#000080
OldLace|#FDF5E6
Olive|#808000
OliveDrab|#6B8E23
Orange|#FFA500
OrangeRed|#FF4500
Orchid|#DA70D6
PaleGoldenRod|#EEE8AA
PaleGreen|#98FB98
PaleTurquoise|#AFEEEE
PaleVioletRed|#DB7093
PapayaWhip|#FFEFD5
PeachPuff|#FFDAB9
Peru|#CD853F
Pink|#FFC0CB
Plum|#DDA0DD
PowderBlue|#B0E0E6
Purple|#800080
RebeccaPurple|#663399
Red|#FF0000
RosyBrown|#BC8F8F
RoyalBlue|#4169E1
SaddleBrown|#8B4513
Salmon|#FA8072
SandyBrown|#F4A460
SeaGreen|#2E8B57
SeaShell|#FFF5EE
Sienna|#A0522D
Silver|#C0C0C0
SkyBlue|#87CEEB
SlateBlue|#6A5ACD
SlateGray|#708090
SlateGrey|#708090
Snow|#FFFAFA
SpringGreen|#00FF7F
SteelBlue|#4682B4
Tan|#D2B48C
Teal|#008080
Thistle|#D8BFD8
Tomato|#FF6347
Turquoise|#40E0D0
Violet|#EE82EE
Wheat|#F5DEB3
White|#FFFFFF
WhiteSmoke|#F5F5F5
Yellow|#FFFF00
YellowGreen|#9ACD32"#;

fn main() {
    let colours = COLOURS
        .trim()
        .split_whitespace()
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
