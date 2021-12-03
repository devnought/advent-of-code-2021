use std::{error::Error, fmt};

use common::read_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_input::<_, String>("input.txt")?;
    let mut x = 0;
    let mut y = 0;

    for line in data {
        match movement(&line)? {
            Movement::Forward(dx) => x += dx,
            Movement::Up(dy) => y -= dy,
            Movement::Down(dy) => y += dy,
        }
    }

    println!("{}", x * y);

    Ok(())
}

#[derive(Debug)]
enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
enum ParseError {
    Parse,
    RemainingInput,
}

impl Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn movement(input: &str) -> Result<Movement, ParseError> {
    let (remaining, movement) = parse_movement(input).map_err(|_| ParseError::Parse)?;

    if !remaining.is_empty() {
        Err(ParseError::RemainingInput)
    } else {
        Ok(movement)
    }
}

fn parse_movement(input: &str) -> IResult<&str, Movement> {
    let (input, (direction, _, amount)) = tuple((direction, space1, numeric))(input)?;

    Ok((
        input,
        match direction {
            Movement::Forward(_) => Movement::Forward(amount),
            Movement::Up(_) => Movement::Up(amount),
            Movement::Down(_) => Movement::Down(amount),
        },
    ))
}

fn numeric(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn forward(input: &str) -> IResult<&str, Movement> {
    let (input, _) = tag("forward")(input)?;

    Ok((input, Movement::Forward(0)))
}

fn up(input: &str) -> IResult<&str, Movement> {
    let (input, _) = tag("up")(input)?;

    Ok((input, Movement::Up(0)))
}

fn down(input: &str) -> IResult<&str, Movement> {
    let (input, _) = tag("down")(input)?;

    Ok((input, Movement::Down(0)))
}

fn direction(input: &str) -> IResult<&str, Movement> {
    let (input, dir) = alt((forward, up, down))(input)?;

    Ok((input, dir))
}
