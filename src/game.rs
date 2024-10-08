type PlayerId = u16;

enum GameState {
    WaitingRoom(WaitingRoomState),
    GameStart,
}

pub struct WaitingRoomState {
    in_room: Vec<PlayerId>,
    ready: Vec<PlayerId>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GameAxiom {
    IsEven,
    IsOdd,
    IsPrime,
    IsDivis10,
    IsLessThan50,
    IsMoreThan50,
    IsSquare,
}

impl TryFrom<u8> for GameAxiom {
    type Error = ();

    fn try_from(value: u8) -> Result<GameAxiom, Self::Error> {
        match value {
            0x00 => Ok(GameAxiom::IsEven),
            0x01 => Ok(GameAxiom::IsOdd),
            0x02 => Ok(GameAxiom::IsPrime),
            0x03 => Ok(GameAxiom::IsDivis10),
            0x04 => Ok(GameAxiom::IsLessThan50),
            0x05 => Ok(GameAxiom::IsMoreThan50),
            0x06 => Ok(GameAxiom::IsSquare),
            _ => Err(()),
        }
    }
}

impl From<GameAxiom> for u8 {
    fn from(value: GameAxiom) -> u8 {
        match value {
            GameAxiom::IsEven => 0x00,
            GameAxiom::IsOdd => 0x01,
            GameAxiom::IsPrime => 0x02,
            GameAxiom::IsDivis10 => 0x03,
            GameAxiom::IsLessThan50 => 0x04,
            GameAxiom::IsMoreThan50 => 0x05,
            GameAxiom::IsSquare => 0x06,
        }
    }
}

// Test if the axiom is true for the given number
pub type AxiomPredicate = fn(n: u8) -> bool;

impl From<GameAxiom> for AxiomPredicate {
    fn from(value: GameAxiom) -> AxiomPredicate {
        match value {
            GameAxiom::IsEven => |n: u8| n % 2 == 0,
            GameAxiom::IsOdd => |n: u8| (n + 1) % 2 == 0,
            GameAxiom::IsPrime => |n: u8| {
                if n < 2 {
                    false
                } else {
                    for i in 2..=((n as f32).sqrt() as u8) {
                        if n % i == 0 {
                            return false;
                        }
                    }

                    true
                }
            },
            GameAxiom::IsDivis10 => |n: u8| n % 10 == 0,
            GameAxiom::IsLessThan50 => |n: u8| n < 50,
            GameAxiom::IsMoreThan50 => |n: u8| n > 50,
            GameAxiom::IsSquare => |n: u8| {
                let root = (n as f32).sqrt() as u8;
                root * root == n
            },
        }
    }
}

// Upper and lower are both inclusive.
// example: lower_bound = 0, upper_bound = 100
pub fn calc_probability(lower_bound: u8, upper_bound: u8, axiom: GameAxiom) -> f32 {
    let test = AxiomPredicate::from(axiom);

    let mut hits = 0;

    let quantity = (lower_bound..=upper_bound).count();
    for i in lower_bound..upper_bound {
        if test(i) {
            hits += 1;
        }
    }

    hits as f32 / quantity as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probabilities() {
        let is_odd = GameAxiom::IsOdd;
        assert_eq!(calc_probability(0, 100, is_odd), (50_f32 / 101_f32));

        let is_even = GameAxiom::IsEven;
        assert_eq!(calc_probability(0, 100, is_even), (50_f32 / 101_f32));

        let is_prime = GameAxiom::IsPrime;
        assert_eq!(calc_probability(0, 100, is_prime), (25_f32 / 101_f32));

        let is_divis_10 = GameAxiom::IsDivis10;
        assert_eq!(calc_probability(0, 100, is_divis_10), (10_f32 / 101_f32));

        let is_gr_50 = GameAxiom::IsMoreThan50;
        assert_eq!(calc_probability(0, 100, is_gr_50), (49_f32 / 101_f32));

        let is_lt_50 = GameAxiom::IsLessThan50;
        assert_eq!(calc_probability(0, 100, is_lt_50), (50_f32 / 101_f32));

        let is_sq = GameAxiom::IsSquare;
        assert_eq!(calc_probability(0, 100, is_sq), (10_f32 / 101_f32));
    }
}
