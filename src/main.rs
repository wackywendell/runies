use std::io::{self, Read};

use itertools::Itertools;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let char_indices = input.char_indices().collect::<Vec<_>>();

    println!("Input:      {}", input);
    println!(
        "Length:     {} bytes, {} chars",
        input.len(),
        char_indices.len()
    );
    println!("Characters: ",);

    let unknown = String::from("Unknown");

    for ((ix, c), (eix, _)) in char_indices.iter().copied().circular_tuple_windows() {
        let eix = if eix == 0 { input.len() } else { eix };
        let name = unicode_names2::name(c).map(|n| n.to_string());
        let name_str = name.as_ref().unwrap_or(&unknown);

        println!(
            "  \"{}\" : 0x{:x} ({}), bytes {:x?}, name: {}",
            c,
            c as u32,
            c as u32,
            &input.as_bytes()[ix..eix],
            name_str,
        )
    }

    Ok(())
}
