use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Item(char);

impl Item {
    pub fn priority(&self) -> u32 {
        let lowercase = 'a'..='z';
        let uppercase = 'A'..='Z';
        if lowercase.contains(&self.0) {
            self.0 as u32 - 'a' as u32 + 1
        } else if uppercase.contains(&self.0) {
            self.0 as u32 - 'A' as u32 + 27
        } else {
            0
        }
    }
}

struct Compartment(Vec<Item>);

impl Compartment {
    fn to_set(&self) -> HashSet<Item> {
        let mut set: HashSet<Item> = HashSet::new();
        for item in self.0.iter() {
            set.insert(item.clone());
        }

        set
    }

    pub fn shared_items(&self, other: &Compartment) -> Vec<Item> {
        let mut shared = vec![];
        for item in self.to_set().intersection(&other.to_set()) {
            shared.push(item.clone())
        }

        shared
    }
}

struct Rucksack {
    pub first_compartment: Compartment,
    pub second_compartment: Compartment,
}

impl std::str::FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 == 1 {
            println!("Bad line length (not even)");
            return Err(());
        }

        let split_idx = s.len() / 2;
        let (first, second) = s.split_at(split_idx);

        Ok(Rucksack {
            first_compartment: Compartment(first.chars().map(|c| Item(c)).collect()),
            second_compartment: Compartment(second.chars().map(|c| Item(c)).collect()),
        })
    }
}

impl Rucksack {
    pub fn duplicates(&self) -> Vec<Item> {
        self.first_compartment
            .shared_items(&self.second_compartment)
    }
}

fn parse_rucksacks(rucksack_lines: Vec<String>) -> Vec<Rucksack> {
    rucksack_lines
        .iter()
        .map(|line| line.parse().expect("Could not parse rucksack line"))
        .collect()
}

pub fn duplicate_priority_sum(rucksack_lines: Vec<String>) -> u32 {
    parse_rucksacks(rucksack_lines)
        .iter()
        .map(|rucksack| {
            rucksack
                .duplicates()
                .iter()
                .map(|item| item.priority())
                .sum::<u32>()
        })
        .sum()
}

mod tests {
    const RUCKSACKS: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_parse_rucksacks() {
        let rucksack_lines = RUCKSACKS.split("\n").map(|s| s.to_string()).collect();
        let rucksacks = super::parse_rucksacks(rucksack_lines);
        assert_eq!(rucksacks.len(), 6);
        for rucksack in rucksacks {
            assert_eq!(
                rucksack.first_compartment.0.len(),
                rucksack.second_compartment.0.len()
            )
        }
    }

    #[test]
    fn test_rucksack_duplicates() {
        let rucksack_lines = RUCKSACKS.split("\n").map(|s| s.to_string()).collect();
        let rucksacks = super::parse_rucksacks(rucksack_lines);

        assert_eq!(rucksacks[0].duplicates(), vec![super::Item('p')]);
        assert_eq!(rucksacks[1].duplicates(), vec![super::Item('L')]);
        assert_eq!(rucksacks[2].duplicates(), vec![super::Item('P')]);
        assert_eq!(rucksacks[3].duplicates(), vec![super::Item('v')]);
        assert_eq!(rucksacks[4].duplicates(), vec![super::Item('t')]);
        assert_eq!(rucksacks[5].duplicates(), vec![super::Item('s')]);
    }

    #[test]
    fn test_duplicate_priority_sum() {
        let rucksack_lines = RUCKSACKS.split("\n").map(|s| s.to_string()).collect();
        let sum = super::duplicate_priority_sum(rucksack_lines);
        assert_eq!(sum, 157)
    }
}
