use rand::Rng;

const TIMESTAMPS_COUNT: usize = 50000;

const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;

const PROBABILITY_HOME_SCORE: f64 = 0.45;

const OFFSET_MAX_STEP: i32 = 3;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
struct Score {
    home: i32,
    away: i32,
}

#[derive(Debug, Clone, Copy)]
struct Stamp {
    offset: i32,
    score: Score,
}

fn generate_stamp(previous_value: Stamp) -> Stamp {
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home
                + if score_changed && home_score_change {
                    1
                } else {
                    0
                },
            away: previous_value.score.away
                + if score_changed && !home_score_change {
                    1
                } else {
                    0
                },
        },
    }
}

fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}

/// In: sorted vector of stamps, offset should be nonnegative; 
/// Out: score (home, away) in a moment equal or before offset 
///     or zeros if there is no such value. Cause panic for 
///     empty vector. 
fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    let score = match game_stamps.iter().position(|stamp| stamp.offset > offset) {
        Some(0) => Score { home: 0, away: 0 },
        Some(next_index) => {
            game_stamps
                .get(next_index - 1)
                .expect(&format!(
                    "calculate index error! next_index: {}",
                    next_index
                ))
                .score
        }
        None => game_stamps.last().expect("empty stamps!").score,
    };

    return (score.home, score.away);
}

fn main() {
    let stamps = generate_game();

    dbg!(get_score(&stamps, 0));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_successful_match() {
        let mut stamps = Vec::new();

        let score = Score { home: 1, away: -1 };

        stamps.push(Stamp {
            offset: 10,
            score: Score { home: 1, away: 0 },
        });
        stamps.push(Stamp { offset: 20, score });
        stamps.push(Stamp {
            offset: 30,
            score: Score { home: 0, away: -1 },
        });

        assert_eq!(get_score(&stamps, 20), (score.home, score.away));
    }

    #[test]
    fn test_match_before() {
        let stamp: Stamp = Stamp {
            offset: 10,
            score: Score {
                home: 10,
                away: -10,
            },
        };

        assert_eq!(get_score(&vec![stamp], 5), (0, 0));
    }

    #[test]
    fn test_match_behind() {
        let stamp: Stamp = Stamp {
            offset: 10,
            score: Score {
                home: 10,
                away: -10,
            },
        };

        assert_eq!(
            get_score(&vec![stamp], 15),
            (stamp.score.home, stamp.score.away)
        );
    }

    #[test]
    #[should_panic]
    fn test_unsuccessful_match() {
        assert_eq!(get_score(&Vec::new(), 10), (0, 0));
    }
}
