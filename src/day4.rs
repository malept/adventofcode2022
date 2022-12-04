use std::{num::ParseIntError, ops::RangeInclusive, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Assignment(RangeInclusive<u32>);

impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range: Vec<&str> = s.splitn(2, "-").collect();
        let begin: u32 = range[0].parse()?;
        let end: u32 = range[1].parse()?;
        Ok(Assignment(RangeInclusive::new(begin, end)))
    }
}

impl Ord for Assignment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len().cmp(&other.len())
    }
}

impl PartialOrd for Assignment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Assignment {
    fn len(&self) -> usize {
        let start = *self.0.start();
        let end = *self.0.end();
        if start > end {
            println!("WTF!!! Start = {}, End = {}", start, end)
        }
        (end - start + 1) as usize
    }
}

struct AssignmentPair {
    first: Assignment,
    second: Assignment,
}

impl FromStr for AssignmentPair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let assignments: Vec<&str> = s.splitn(2, ",").collect();
        let first: Assignment = assignments[0].parse()?;
        let second: Assignment = assignments[1].parse()?;
        Ok(AssignmentPair { first, second })
    }
}

impl AssignmentPair {
    fn smallest_assignment(&self) -> &Assignment {
        std::cmp::min(&self.first, &self.second)
    }

    fn sizes_equal(&self) -> bool {
        self.first.len() == self.second.len()
    }

    pub fn smallest_is_subset(&self) -> bool {
        if self.sizes_equal() {
            self.first == self.second
        } else {
            let smallest = self.smallest_assignment();
            let greatest = if smallest == &self.first {
                &self.second
            } else {
                &self.first
            };

            let mut subset = (*smallest).clone();
            subset.0.all(|n| greatest.0.contains(&n))
        }
    }
}

fn parse_assignment_pairs(lines: &Vec<String>) -> impl Iterator<Item = AssignmentPair> + '_ {
    lines.iter().map(|l| {
        l.parse::<AssignmentPair>()
            .expect("Could not parse assignment pair line")
    })
}

pub fn full_subset_assignment_pairs_count(lines: &Vec<String>) -> usize {
    parse_assignment_pairs(lines)
        .filter(|pair| pair.smallest_is_subset())
        .count()
}

mod tests {
    const ASSIGNMENTS: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_parse_assignment_pairs() {
        let assignments = crate::util::testcase_to_input(ASSIGNMENTS);
        let pairs: Vec<super::AssignmentPair> =
            super::parse_assignment_pairs(&assignments).collect();
        assert_eq!(pairs.len(), 6);
        assert_eq!(pairs[0].first, super::Assignment(2..=4));
        assert_eq!(pairs[0].second, super::Assignment(6..=8));
        assert_eq!(pairs[5].first, super::Assignment(2..=6));
        assert_eq!(pairs[5].second, super::Assignment(4..=8));
    }

    #[test]
    fn test_smallest_assignment() {
        let assignments = crate::util::testcase_to_input(ASSIGNMENTS);
        let pairs: Vec<super::AssignmentPair> =
            super::parse_assignment_pairs(&assignments).collect();
        assert_eq!(*pairs[3].smallest_assignment(), super::Assignment(3..=7));
        assert_eq!(*pairs[4].smallest_assignment(), super::Assignment(6..=6));
    }

    #[test]
    fn test_full_subset_assignment_pairs_count() {
        let assignments = crate::util::testcase_to_input(ASSIGNMENTS);
        let count = super::full_subset_assignment_pairs_count(&assignments);
        assert_eq!(count, 2);
    }
}
