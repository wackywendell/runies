use std::io::{self, Read};

use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let graphemes = UnicodeSegmentation::graphemes(input.as_str(), true).collect::<Vec<&str>>();

    let char_indices = input.char_indices().collect::<Vec<_>>();

    println!("Input:      {}", input);
    println!(
        "Length:     {} grapheme clusters, {} chars, {} bytes",
        graphemes.len(),
        char_indices.len(),
        input.len(),
    );

    let unknown = String::from("Unknown");
    println!("Clusters: ",);

    for cluster in graphemes {
        let char_indices = cluster.char_indices().collect::<Vec<_>>();
        let mut start = "  ";
        if char_indices.len() > 1 {
            println!(
                "  \"{}\" : {} bytes, {} chars",
                cluster,
                cluster.len(),
                char_indices.len()
            );
            start = "    ";
        }

        for ((ix, c), (eix, _)) in char_indices.iter().copied().circular_tuple_windows() {
            let eix = if eix == 0 { cluster.len() } else { eix };
            let name = unicode_names2::name(c).map(|n| n.to_string());
            let name_str = name.as_ref().unwrap_or(&unknown);

            println!(
                "{}\"{}\" : 0x{:x} ({}), bytes {:x?}, name: {}",
                start,
                c,
                c as u32,
                c as u32,
                &cluster.as_bytes()[ix..eix],
                name_str,
            )
        }
    }

    Ok(())
}
