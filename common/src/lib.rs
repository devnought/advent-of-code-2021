use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub fn read_input<P, T>(path: P) -> Result<impl Iterator<Item = T>, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: FromStr,
{
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let iter = lines
        .filter_map(|x| x.ok())
        .filter_map(|x| x.trim().parse::<T>().ok());

    Ok(iter)
}
