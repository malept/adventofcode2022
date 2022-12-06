use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy)]
struct Crate(char);

struct CrateRow(Vec<Option<Crate>>);

impl FromStr for CrateRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("CRATE ROW: {}", s);
        let mut crates = vec![];
        for (idx, chunk) in s.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if chunk[0] == '[' {
                println!("* crate: {}", chunk[1]);
                crates.push(Some(Crate(chunk[1])))
            } else {
                println!("* no crate");
                crates.push(None)
            }
        }
        Ok(CrateRow(crates))
    }
}

struct Stack(Vec<Crate>);

struct Stacks(HashMap<u32, Stack>);

impl Stacks {
    fn move_crates(&mut self, count: usize, from_id: u32, to_id: u32) {
        let from: &mut Stack = self.0.get_mut(&from_id).expect("Could not find stack");
        let mut to_move = Vec::with_capacity(count);
        for _n in 0..count {
            if let Some(c) = from.0.pop() {
                to_move.push(c);
            } else {
                panic!("** WARNING ** NO MORE CRATES IN {from_id}, FAILING");
            }
        }

        let to: &mut Stack = self.0.get_mut(&to_id).expect("Could not find stack");
        for c in to_move.iter().rev() {
            to.0.push(*c);
        }
        println!("* to stack: {}", to.0.len())
    }

    pub fn follow_instructions(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            println!("Instruction: {:?}", instruction);
            self.move_crates(instruction.count, instruction.from_id, instruction.to_id);
            self.ordered();
        }
    }

    fn ordered(&self) {
        let mut keys: Vec<&u32> = self.0.keys().collect();
        keys.sort();
        for key in keys {
            let stack = self.0.get(key).expect("Inexplicably cannot find Stack");
            let stack_len = stack.0.len();
            print!("{key}: {stack_len}, ");
        }
        println!("");
    }

    pub fn top_crates(&self) -> String {
        let mut keys: Vec<&u32> = self.0.keys().collect();
        keys.sort();
        let mut crate_values = String::with_capacity(keys.len());
        for key in keys {
            let stack = self.0.get(key).expect("Inexplicably cannot find Stack");
            let crate_value = if let Some(c) = stack.0.last() {
                c.0
            } else {
                '?'
            };
            crate_values.push(crate_value)
        }

        crate_values
    }
}

impl FromStr for Stacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<&str>>();
        let stack_height: usize = lines.len() - 1;
        let mut stacks: HashMap<u32, Stack> = HashMap::with_capacity(stack_height);

        let mut crates: Vec<CrateRow> = vec![];
        let mut ids: Vec<u32> = vec![];
        for (idx, line) in lines.iter().enumerate() {
            if idx == stack_height {
                lazy_static! {
                    static ref IDS_SPLIT_RE: Regex =
                        Regex::new(" +").expect("Could not compile split regex");
                }
                ids = IDS_SPLIT_RE
                    .split(line.trim())
                    .map(|id| id.parse::<u32>().expect("Could not parse number"))
                    .collect();
                for id in &ids {
                    stacks.insert(*id, Stack(vec![]));
                }
            } else {
                let crate_line = line.parse().expect("could not parse crate line");
                crates.push(crate_line);
            }
        }

        for row in crates.iter().rev() {
            for (idx, maybe_crate) in row.0.iter().enumerate() {
                let stack: &mut Stack = stacks
                    .get_mut(&ids[idx])
                    .expect("Could not find stack in stacks");
                if let Some(c) = maybe_crate {
                    stack.0.push(*c);
                }
            }
        }

        Ok(Stacks(stacks))
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    count: usize,
    from_id: u32,
    to_id: u32,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCTION_RE: Regex =
                Regex::new("^move (?P<count>\\d+) from (?P<from>\\d+) to (?P<to>\\d+)$")
                    .expect("Could not compile instruction regex");
        }

        let caps = INSTRUCTION_RE.captures(s).expect("Could not match");
        let count = caps
            .name("count")
            .expect("Could not find count")
            .as_str()
            .parse()?;
        let from_id = caps
            .name("from")
            .expect("Could not find count")
            .as_str()
            .parse()?;
        let to_id = caps
            .name("to")
            .expect("Could not find to")
            .as_str()
            .parse()?;

        Ok(Instruction {
            count,
            from_id,
            to_id,
        })
    }
}

fn parse_instructions(manual: &str) -> Vec<Instruction> {
    manual
        .split("\n")
        .map(|line| {
            line.parse::<Instruction>()
                .expect("Could not parse instruction")
        })
        .collect()
}

pub fn top_stacked_crates(stacks_and_instructions: String) -> String {
    let parts: Vec<&str> = stacks_and_instructions.splitn(2, "\n\n").collect();
    let mut stacks: Stacks = parts[0].parse().expect("Could not parse stacks");
    let instructions = parse_instructions(parts[1]);
    stacks.ordered();
    stacks.follow_instructions(instructions);
    stacks.top_crates()
}

mod tests {
    const CRATES: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_instructions() {
        let parts: Vec<&str> = CRATES.splitn(2, "\n\n").collect();
        let instructions: Vec<super::Instruction> = super::parse_instructions(parts[1]);
        assert_eq!(instructions.len(), 4);
        assert_eq!(
            instructions[0],
            super::Instruction {
                count: 1,
                from_id: 2,
                to_id: 1,
            }
        )
    }

    #[test]
    fn test_top_stacked_crates() {
        let top = super::top_stacked_crates(CRATES.to_string());
        assert_eq!(top, "MCD".to_string())
    }
}
