#![allow(dead_code)]

use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hands<'a> {
    HighCard(&'a Card),
    OnePair(&'a Card),
    TwoPairs((&'a Card, &'a Card)),
    ThreeOfAKind(&'a Card),
    FullHouse(&'a Card, &'a Card),
    FourOfAKind(&'a Card),
    FiveOfAKind(&'a Card),
}

#[derive(Debug, Clone, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(usize),
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Card::A, Card::A) => true,
            (Card::K, Card::K) => true,
            (Card::Q, Card::Q) => true,
            (Card::J, Card::J) => true,
            (Card::Number(a), Card::Number(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Card::A, Card::A) => Some(Ordering::Equal),
            (Card::A, _) => Some(Ordering::Greater),
            (Card::K, Card::K) => Some(Ordering::Equal),
            (Card::K, Card::A) => Some(Ordering::Less),
            (Card::K, _) => Some(Ordering::Greater),
            (Card::Q, Card::Q) => Some(Ordering::Equal),
            (Card::Q, Card::A) => Some(Ordering::Less),
            (Card::Q, Card::K) => Some(Ordering::Less),
            (Card::Q, _) => Some(Ordering::Greater),
            (Card::J, Card::J) => Some(Ordering::Equal),
            (Card::J, Card::A) => Some(Ordering::Less),
            (Card::J, Card::K) => Some(Ordering::Less),
            (Card::J, Card::Q) => Some(Ordering::Less),
            (Card::J, _) => Some(Ordering::Greater),
            (Card::T, Card::A) => Some(Ordering::Less),
            (Card::T, Card::K) => Some(Ordering::Less),
            (Card::T, Card::Q) => Some(Ordering::Less),
            (Card::T, Card::J) => Some(Ordering::Less),
            (Card::T, Card::T) => Some(Ordering::Equal),
            (Card::T, _) => Some(Ordering::Greater),
            (Card::Number(a), Card::Number(b)) => a.partial_cmp(b),
            (Card::Number(_a), Card::A) => Some(Ordering::Less),
            (Card::Number(_a), Card::K) => Some(Ordering::Less),
            (Card::Number(_a), Card::Q) => Some(Ordering::Less),
            (Card::Number(_a), Card::J) => Some(Ordering::Less),
            (Card::Number(_), Card::T) => Some(Ordering::Less),
            // (Card::Number(a), _) => Some(Ordering::Greater),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Card::A, Card::A) => Ordering::Equal,
            (Card::A, _) => Ordering::Greater,
            (Card::K, Card::K) => Ordering::Equal,
            (Card::K, Card::A) => Ordering::Less,
            (Card::K, _) => Ordering::Greater,
            (Card::Q, Card::Q) => Ordering::Equal,
            (Card::Q, Card::A) => Ordering::Less,
            (Card::Q, Card::K) => Ordering::Less,
            (Card::Q, _) => Ordering::Greater,
            (Card::J, Card::J) => Ordering::Equal,
            (Card::J, Card::A) => Ordering::Less,
            (Card::J, Card::K) => Ordering::Less,
            (Card::J, Card::Q) => Ordering::Less,
            (Card::J, _) => Ordering::Greater,
            (Card::T, Card::A) => Ordering::Less,
            (Card::T, Card::K) => Ordering::Less,
            (Card::T, Card::Q) => Ordering::Less,
            (Card::T, Card::J) => Ordering::Less,
            (Card::T, Card::T) => Ordering::Equal,
            (Card::T, _) => Ordering::Greater,
            (Card::Number(a), Card::Number(b)) => a.cmp(b),
            (Card::Number(_a), Card::A) => Ordering::Less,
            (Card::Number(_a), Card::K) => Ordering::Less,
            (Card::Number(_a), Card::Q) => Ordering::Less,
            (Card::Number(_a), Card::J) => Ordering::Less,
            (Card::Number(_), Card::T) => Ordering::Less,
            // (Card::Number(a), _) => Ordering::Greater,
        }
    }
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        match s {
            "A" => Card::A,
            "K" => Card::K,
            "Q" => Card::Q,
            "J" => Card::J,
            "T" => Card::T,
            _ => Card::Number(s.parse::<usize>().unwrap()),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn score_hand(&self) -> Hands {
        if let Some(c) = self.is_five_of_a_kind() {
            return Hands::FiveOfAKind(c);
        } else if let Some(c) = self.is_four_of_a_kind() {
            return Hands::FourOfAKind(c);
        } else if let Some(c) = self.is_three_of_a_kind() {
            return Hands::ThreeOfAKind(c);
        } else if let Some(c) = self.is_two_pairs() {
            return Hands::TwoPairs(c);
        } else if let Some(c) = self.has_one_pair() {
            return Hands::OnePair(c);
        }

        Hands::HighCard(self.cards.iter().max().expect("no cards"))
    }

    fn is_five_of_a_kind(&self) -> Option<&Card> {
        self.cards
            .iter()
            .filter(|c| self.cards.iter().filter(|c2| c2 == c).count() == 5)
            .next()
    }

    fn is_four_of_a_kind(&self) -> Option<&Card> {
        self.cards
            .iter()
            .filter(|c| self.cards.iter().filter(|c2| c2 == c).count() == 4)
            .next()
    }

    fn is_full_house(&self) -> Option<(&Card, &Card)> {
        let three = self.is_three_of_a_kind();
        let two = self.has_one_pair();
        if three.is_some() && two.is_some() {
            return Some((three.unwrap(), two.unwrap()));
        }
        None
    }

    fn is_three_of_a_kind(&self) -> Option<&Card> {
        self.cards
            .iter()
            .filter(|c| self.cards.iter().filter(|c2| c2 == c).count() == 3)
            .next()
    }
    fn is_two_pairs(&self) -> Option<(&Card, &Card)> {
        self.cards
            .iter()
            .filter(|c| self.cards.iter().filter(|c2| c2 == c).count() == 2)
            .unique()
            .collect_tuple()
    }
    fn has_one_pair(&self) -> Option<&Card> {
        self.cards
            .iter()
            .filter(|c| self.cards.iter().filter(|c2| c2 == c).count() == 2)
            .unique()
            .next()
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let mut cards = s
            .split("")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| Card::from(l))
            .collect_vec()
            .into_iter();

        Hand {
            cards: [
                cards.next().expect("unable to get first card"),
                cards.next().expect("unable to get second card"),
                cards.next().expect("unable to get third card"),
                cards.next().expect("unable to get forth card"),
                cards.next().expect("unable to get fifth card"),
            ],
        }
    }
}

#[derive(Debug)]
struct Game {
    hand: Hand,
    bet: usize,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hand.score_hand().eq(&other.hand.score_hand())
    }
}

impl Eq for Game {}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match std::mem::discriminant(&self.hand.score_hand())
            == std::mem::discriminant(&other.hand.score_hand())
        {
            true => {
                for (a, b) in self.hand.cards.iter().zip(other.hand.cards.iter()) {
                    if a != b {
                        return a.partial_cmp(b);
                    }
                }
                return Some(Ordering::Equal);
            }
            false => self.hand.score_hand().partial_cmp(&other.hand.score_hand()),
        }
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        match std::mem::discriminant(&self.hand.score_hand())
            == std::mem::discriminant(&other.hand.score_hand())
        {
            true => {
                for (a, b) in self.hand.cards.iter().zip(other.hand.cards.iter()) {
                    if a != b {
                        return a.cmp(b);
                    }
                }
                return Ordering::Equal;
            }
            false => self.hand.score_hand().cmp(&other.hand.score_hand()),
        }
    }
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|x| {
            let (hand, bet) = x
                .split(" ")
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .collect_tuple()
                .unwrap();
            // dbg!(hand);
            let hand = Hand::from(hand);
            let bet = bet.parse::<usize>().unwrap();
            Game { hand, bet }
        })
        .sorted()
        .enumerate()
        .map(|(idx, x)| dbg!((idx + 1) * x.bet))
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn part_1_example() {
        let input = include_str!("./part1.test.txt");
        assert_eq!(6440, solve_part_1(input));
    }

    #[test]
    fn part_1_real() {
        let input = include_str!("./part1.txt");
        assert_eq!(250775371, solve_part_1(input));
    }

    // #[test]
    // fn part_2_example() {
    //     let input = include_str!("./part1.test.txt");
    //     assert_eq!(2286, solve2(input) );
    // }

    // #[test]
    // fn part_2_real() {
    //     let input = include_str!("./part1.txt");
    //     assert_eq!(49710, solve2(input) );
    // }

    #[test]
    fn ace_is_high_card() {
        assert!(Card::A > Card::K);
        assert!(Card::K > Card::Q);
        assert!(Card::Q > Card::J);
        assert!(Card::J > Card::Number(10));
        assert!(Card::Number(10) > Card::Number(9));
        assert!(Card::Number(9) > Card::Number(8));
        assert!(Card::Number(8) > Card::Number(7));
        assert!(Card::Number(7) > Card::Number(6));
        assert!(Card::Number(6) > Card::Number(5));
        assert!(Card::Number(5) > Card::Number(4));
        assert!(Card::Number(4) > Card::Number(3));
        assert!(Card::Number(3) > Card::Number(2));
        assert!(Card::Number(2) < Card::A);
        assert!(Card::Number(3) < Card::A);
        assert!(Card::Number(4) < Card::A);
        assert!(Card::Number(5) < Card::A);
        assert!(Card::Number(6) < Card::A);
        assert!(Card::Number(7) < Card::A);
        assert!(Card::Number(8) < Card::A);
        assert!(Card::Number(9) < Card::A);
        assert!(Card::Number(10) < Card::A);
        assert!(Card::J < Card::A);
        assert!(Card::Q < Card::A);
        assert!(Card::K < Card::A);
    }

    #[test]
    fn three_of_a_kind_high() {
        let hand1 = Hand::from("QQQJA");
        let hand2 = Hand::from("T55J5");
        assert!(hand1.score_hand() > hand2.score_hand())
    }

    #[test]
    fn two_pair_is_better_than_one() {
        let two_p = Game {
            hand: Hand::from("KK677"),
            bet: 1,
        };
        let pair = Game {
            hand: Hand::from("32T3K"),
            bet: 1,
        };

        assert!(two_p > pair)
    }

    #[test]
    fn better_two_pair() {
        let highp = Game {
            hand: Hand::from("KK677"),
            bet: 1,
        };
        let lowp = Game {
            hand: Hand::from("KTJJT"),
            bet: 1,
        };

        assert!(highp > lowp)
    }

    #[test]
    fn three_of_a_kind_beats_two_of_a_kind() {
        let expected = vec![
            Game {
                hand: Hand::from("KK677"),
                bet: 1,
            },
            Game {
                hand: Hand::from("QQQJA"),
                bet: 1,
            },
        ];
        let mut actual = vec![
            Game {
                hand: Hand::from("QQQJA"),
                bet: 1,
            },
            Game {
                hand: Hand::from("KK677"),
                bet: 1,
            },
        ];

        actual = actual.into_iter().sorted().collect_vec();

        assert_equal(actual, expected)
    }

    #[test]
    fn is_full_house() {
        let fh = Game {
            hand: Hand::from("KK777"),
            bet: 1,
        };

        assert!(fh.hand.is_full_house().is_some())
    }

    #[test]
    fn is_full_house_correct() {
        let fh = Game {
            hand: Hand::from("KK777"),
            bet: 1,
        };

        assert_eq!(Some((&Card::Number(7), &Card::K)),fh.hand.is_full_house())
    }

    #[test]
    fn full_house_order() {
        let fh = Game {
            hand: Hand::from("KK777"),
            bet: 1,
        };

        let fh2 = Game {
            hand: Hand::from("K77K7"),
            bet: 1,
        };

        assert!(fh > fh2)
    }


    #[test]
    fn is_not_full_house() {
        let fh = Game {
            hand: Hand::from("KK377"),
            bet: 1,
        };

        assert!(fh.hand.is_full_house().is_none())
    }
}
