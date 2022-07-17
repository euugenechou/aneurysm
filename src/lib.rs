use anyhow::{anyhow, Context, Result};
use byteorder::ReadBytesExt;
use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, multispace0, none_of},
    combinator::{recognize, value},
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const DEFAULT_TAPE_LENGTH: usize = 30000;

#[derive(Clone, Debug)]
enum Instruction {
    Advance,
    Retreat,
    Increment,
    Decrement,
    Output,
    Accept,
    Forward,
    Backward,
}

#[derive(Debug)]
pub struct Aneurysm;

impl Aneurysm {
    pub fn interpret<P: AsRef<Path>>(path: P) -> Result<()> {
        let file = fs::read_to_string(path)?;
        let instructions = tokenize(&file)?;

        let mut stack = Vec::new();
        let mut braces = HashMap::new();
        for (i, instruction) in instructions.iter().enumerate() {
            match *instruction {
                Instruction::Forward => {
                    stack.push(i);
                }
                Instruction::Backward => {
                    let partner = stack.pop().context("imbalanced braces")?;
                    braces.insert(i, partner);
                    braces.insert(partner, i);
                }
                _ => {}
            }
        }
        if !stack.is_empty() {
            return Err(anyhow!("imbalanced braces"));
        }

        let mut i = 0;
        let mut head = 0;
        let mut tape = vec![0; DEFAULT_TAPE_LENGTH];

        while i < instructions.len() {
            match instructions[i] {
                Instruction::Advance => {
                    head += 1;
                    if head >= tape.len() {
                        tape.append(&mut vec![0; tape.len()]);
                    }
                }
                Instruction::Retreat => {
                    head -= 1;
                }
                Instruction::Increment => {
                    tape[head] += 1;
                }
                Instruction::Decrement => {
                    tape[head] -= 1;
                }
                Instruction::Output => {
                    print!("{}", tape[head] as u8 as char);
                }
                Instruction::Accept => {
                    print!("> ");
                    io::stdout().flush()?;
                    tape[head] = io::stdin().lock().read_i8()?;
                }
                Instruction::Forward => {
                    if tape[head] == 0 {
                        i = *braces.get(&i).unwrap();
                        continue;
                    }
                }
                Instruction::Backward => {
                    if tape[head] != 0 {
                        i = *braces.get(&i).unwrap();
                        continue;
                    }
                }
            }
            i += 1;
        }

        Ok(())
    }
}

fn tokenize(s: &str) -> Result<Vec<Instruction>> {
    match many0(parse)(s) {
        Ok((_, instructions)) => Ok(instructions),
        Err(_) => Err(anyhow!("syntax error")),
    }
}

fn parse(s: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Advance, delimited(junk, char('>'), junk)),
        value(Instruction::Retreat, delimited(junk, char('<'), junk)),
        value(Instruction::Increment, delimited(junk, char('+'), junk)),
        value(Instruction::Decrement, delimited(junk, char('-'), junk)),
        value(Instruction::Output, delimited(junk, char('.'), junk)),
        value(Instruction::Accept, delimited(junk, char(','), junk)),
        value(Instruction::Forward, delimited(junk, char('['), junk)),
        value(Instruction::Backward, delimited(junk, char(']'), junk)),
    ))(s)
}

fn junk(s: &str) -> IResult<&str, &str> {
    alt((
        delimited(
            pair(multispace0, char('#')),
            is_not("\n"),
            pair(char('\n'), multispace0),
        ),
        recognize(many0(none_of("><+-.,[]"))),
    ))(s)
}
