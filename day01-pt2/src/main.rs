use std::error::Error;

use common::read_input;

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_input("input.txt")?.collect::<Vec<_>>();
    let mut iter = data.windows(3).map(|x| x.iter().sum::<i32>());

    let mut last = if let Some(d) = iter.next() {
        d
    } else {
        return Ok(());
    };

    let mut count = 0;

    for d in iter {
        if d > last {
            count += 1;
        }

        last = d;
    }

    println!("{}", count);

    Ok(())
}
