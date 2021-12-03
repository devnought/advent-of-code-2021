use std::error::Error;

use common::read_input;

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = read_input::<_, i32>("input.txt")?;

    let mut last = if let Some(d) = data.next() {
        d
    } else {
        return Ok(());
    };

    let mut count = 0;

    for d in data {
        if d > last {
            count += 1;
        }

        last = d;
    }

    println!("{}", count);

    Ok(())
}
