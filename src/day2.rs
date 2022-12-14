use std::str::FromStr;

#[derive(Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    pub fn beats(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
    pub fn beaten_by(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

impl RPS {
    pub fn determine_move(opponent: RPS, value: &str) -> Self {
        match value {
            // Lose
            "X" => opponent.beats(),
            // Draw
            "Y" => opponent,
            // Win
            "Z" => opponent.beaten_by(),
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Clone, Copy)]
enum OutcomeScore {
    LOSS = 0,
    DRAW = 3,
    WIN = 6,
}

struct Strategy {
    initial: RPS,
    response: RPS,
}

impl Strategy {
    fn outcome(&self) -> &OutcomeScore {
        match (self.initial, self.response) {
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
                &OutcomeScore::DRAW
            }
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => {
                &OutcomeScore::WIN
            }
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => {
                &OutcomeScore::LOSS
            }
        }
    }
    pub fn score(&self) -> u32 {
        *self.outcome() as u32 + self.response as u32
    }
}

fn parse_strategy_guide(guide_lines: Vec<String>) -> impl Iterator<Item = Strategy> {
    guide_lines.into_iter().map(|line| {
        let items: Vec<&str> = line.split(" ").collect();
        let initial: RPS = items[0].parse().expect("Could not parse first item");
        let response = RPS::determine_move(initial, items[1]);
        Strategy { initial, response }
    })
}

pub fn strategy_guide_total_score(guide_lines: Vec<String>) -> u32 {
    parse_strategy_guide(guide_lines)
        .map(|strategy| strategy.score())
        .sum()
}

mod tests {
    const STRATEGY_GUIDE: &str = "A Y
B X
C Z";

    #[test]
    fn test_strategy_guide_total_score() {
        let guide_lines = STRATEGY_GUIDE.split("\n").map(|l| l.to_string()).collect();
        assert_eq!(super::strategy_guide_total_score(guide_lines), 12)
    }
}
