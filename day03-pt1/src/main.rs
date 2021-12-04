use std::{collections::HashMap, error::Error};

use common::read_input;

fn main() -> Result<(), Box<dyn Error>> {
    let data =
        read_input::<_, String>("input.txt")?.filter_map(|x| u16::from_str_radix(&x, 2).ok());

    const MASK: [u16; 12] = [
        0b0000_0000_0000_0001,
        0b0000_0000_0000_0010,
        0b0000_0000_0000_0100,
        0b0000_0000_0000_1000,
        0b0000_0000_0001_0000,
        0b0000_0000_0010_0000,
        0b0000_0000_0100_0000,
        0b0000_0000_1000_0000,
        0b0000_0001_0000_0000,
        0b0000_0010_0000_0000,
        0b0000_0100_0000_0000,
        0b0000_1000_0000_0000,
    ];

    let mut ones_count = HashMap::new();
    let mut rows = 0;

    for line in data {
        for m in MASK {
            if line & m == m {
                let count = ones_count.entry(m).or_insert(0);
                *count += 1;
            }
        }

        rows += 1;
    }

    let half_rows = rows / 2;

    let (gamma, epsilon) = MASK
        .iter()
        .enumerate()
        .map(|(i, mask)| {
            let count = ones_count.get(mask).unwrap();

            if count > &half_rows {
                (1 << i, 0)
            } else {
                (0, 1 << i)
            }
        })
        .fold((0u16, 0u16), |(gamma, epsilon), (x, y)| {
            (gamma | x, epsilon | y)
        });

    let result = gamma as usize * epsilon as usize;

    println!("{}", result);

    Ok(())
}
