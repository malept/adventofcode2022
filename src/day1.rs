struct Elf {
    pub calories: Vec<u32>,
}

impl Elf {
    pub fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}

fn parse_elf_calories(list: &str) -> Vec<Elf> {
    list.split("\n\n")
        .map(|calblock| {
            let calories = calblock
                .split("\n")
                .map(|calorie| calorie.parse().expect("could not parse string to number"))
                .collect();
            Elf { calories }
        })
        .collect()
}

pub fn most_calories(list: &str) -> u32 {
    parse_elf_calories(list)
        .iter()
        .max_by(|elf1, elf2| elf1.total_calories().cmp(&elf2.total_calories()))
        .expect("No elves in list")
        .total_calories()
}

mod tests {
    use super::{most_calories, parse_elf_calories};
    const EXAMPLE_LIST: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_parse_elf_calories() {
        let elves = parse_elf_calories(EXAMPLE_LIST);
        assert_eq!(elves.len(), 5)
    }

    #[test]
    fn test_elf_total_calories() {
        let elves = parse_elf_calories(EXAMPLE_LIST);
        assert_eq!(elves[0].total_calories(), 6_000);
        assert_eq!(elves[1].total_calories(), 4_000);
        assert_eq!(elves[2].total_calories(), 11_000);
        assert_eq!(elves[3].total_calories(), 24_000);
        assert_eq!(elves[4].total_calories(), 10_000);
    }

    #[test]
    fn test_most_calories() {
        assert_eq!(most_calories(EXAMPLE_LIST), 24_000)
    }
}
