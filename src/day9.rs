use std::cmp::Ordering;
use std::iter;
use std::ops::RangeInclusive;

use crate::utils;
use crate::utils::{Vec2, read_lines};

fn get_positions() -> Vec<Vec2<u64>> {
    read_lines(9)
        .map(|s| {
            let (l, r) = s.split_once(',').unwrap();
            [l, r].map(|v| v.parse().unwrap()).into()
        })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Turn {
    Clockwise,
    Counterclockwise,
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match (self, turn) {
            (Self::Left, Turn::Clockwise) | (Self::Right, Turn::Counterclockwise) => Self::Up,
            (Self::Up, Turn::Clockwise) | (Self::Down, Turn::Counterclockwise) => Self::Right,
            (Self::Right, Turn::Clockwise) | (Self::Left, Turn::Counterclockwise) => Self::Down,
            (Self::Down, Turn::Clockwise) | (Self::Up, Turn::Counterclockwise) => Self::Left,
        }
    }

    fn get_turn(self, other: Self) -> Turn {
        match (self, other) {
            (Self::Up, Self::Right)
            | (Self::Right, Self::Down)
            | (Self::Down, Self::Left)
            | (Self::Left, Self::Up) => Turn::Clockwise,
            (Self::Right, Self::Up)
            | (Self::Down, Self::Right)
            | (Self::Left, Self::Down)
            | (Self::Up, Self::Left) => Turn::Counterclockwise,
            _ => unreachable!(),
        }
    }

    fn flip(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    fn vertical(self) -> bool {
        match self {
            Self::Up | Self::Down => true,
            Self::Right | Self::Left => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Vec2<u64>,
    end: Vec2<u64>,
    direction: Direction,
}

impl Line {
    fn new(start: Vec2<u64>, end: Vec2<u64>) -> Self {
        Self {
            start,
            end,
            direction: match (start.x.cmp(&end.x), start.y.cmp(&end.y)) {
                (Ordering::Equal, Ordering::Less) => Direction::Down,
                (Ordering::Equal, Ordering::Greater) => Direction::Up,
                (Ordering::Less, Ordering::Equal) => Direction::Right,
                (Ordering::Greater, Ordering::Equal) => Direction::Left,
                _ => unreachable!(),
            },
        }
    }

    fn vertical(&self) -> Option<(u64, RangeInclusive<u64>)> {
        match self.direction {
            Direction::Down => Some((self.start.x, self.start.y..=self.end.y)),
            Direction::Up => Some((self.start.x, self.end.y..=self.start.y)),
            Direction::Left | Direction::Right => None,
        }
    }

    fn horizontal(&self) -> Option<(RangeInclusive<u64>, u64)> {
        match self.direction {
            Direction::Right => Some((self.start.x..=self.end.x, self.start.y)),
            Direction::Left => Some((self.end.x..=self.start.x, self.start.y)),
            Direction::Up | Direction::Down => None,
        }
    }
}

pub fn puzzle1() {
    let positions = get_positions();
    let result = positions[..positions.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| {
            positions[i + 1..].iter().map(move |&p2| (p1.map2(p2, u64::abs_diff) + 1).product())
        })
        .max()
        .unwrap();
    println!("{result}");
}

pub fn puzzle2() {
    let positions = get_positions();
    let lines: Vec<_> = positions
        .array_windows()
        .chain(iter::once(&[positions[positions.len() - 1], positions[0]]))
        .map(|&[start, end]| Line::new(start, end))
        .collect();
    let total_turns: i32 = lines
        .array_windows()
        .chain(iter::once(&[lines[lines.len() - 1], lines[0]]))
        .map(|[l, r]| match l.direction.get_turn(r.direction) {
            Turn::Clockwise => 1,
            Turn::Counterclockwise => -1,
        })
        .sum();
    let polygon_turn = match total_turns {
        4 => Turn::Clockwise,
        -4 => Turn::Counterclockwise,
        _ => unreachable!(),
    };
    let mut rectangles: Vec<_> = (0..positions.len() - 1)
        .flat_map(|i| {
            let p = &positions;
            (i + 1..positions.len()).map(move |j| {
                let p1 = p[i];
                let p2 = p[j];
                let area = (p1.map2(p2, u64::abs_diff) + 1).product();
                let points = utils::sort2_by_key((i, p1), (j, p2), |i| i.1.x);
                (area, points)
            })
        })
        .collect();
    rectangles.sort_unstable_by(|(area1, ..), (area2, ..)| area2.cmp(area1));
    let get_lines =
        |i: usize| [&lines[i.checked_sub(1).unwrap_or_else(|| lines.len() - 1)], &lines[i]];
    let result = rectangles
        .into_iter()
        .find_map(|(area, points)| {
            let corners = points.map(|(n, _)| get_lines(n));
            let convex =
                corners.map(|[l1, l2]| l1.direction.get_turn(l2.direction) == polygon_turn);
            let inside = corners.map(|[l1, l2]| {
                utils::sort2_by_key(
                    l1.direction.turn(polygon_turn),
                    l2.direction.turn(polygon_turn),
                    |l| l.vertical(),
                )
            });
            let y_diff = match points[0].1.y.cmp(&points[1].1.y) {
                Ordering::Less => Direction::Down,
                Ordering::Greater => Direction::Up,
                Ordering::Equal => unreachable!(),
            };
            let expected = [[Direction::Right, y_diff], [Direction::Left, y_diff.flip()]];
            // Check corners, convex corners have to match exact, concave corner cannot be opposite
            for i in 0..=1 {
                if convex[i] && inside[i] != expected[i]
                    || !convex[i] && inside[i] == expected[i].map(Direction::flip)
                {
                    return None;
                }
            }
            // Check if any edges intersect, unless they only intersect on a vertex of a line, that
            // does not go into the rectangle
            let x_range = points[0].1.x + 1..points[1].1.x;
            for i in 0..=1 {
                let y = points[i].1.y;
                let y_expected = expected[i][1];
                for (x, y_range) in lines.iter().filter_map(Line::vertical) {
                    if x_range.contains(&x)
                        && y_range.contains(&y)
                        && !(y_expected == Direction::Down && *y_range.end() == y
                            || y_expected == Direction::Up && *y_range.start() == y)
                    {
                        return None;
                    }
                }
            }
            let y_range = if y_diff == Direction::Down {
                points[0].1.y + 1..points[1].1.y
            } else {
                points[1].1.y + 1..points[0].1.y
            };
            for i in 0..=1 {
                let x = points[i].1.x;
                let x_expected = expected[i][0];
                for (x_range, y) in lines.iter().filter_map(Line::horizontal) {
                    if y_range.contains(&y)
                        && x_range.contains(&x)
                        && !(x_expected == Direction::Right && *x_range.end() == x
                            || x_expected == Direction::Left && *x_range.start() == x)
                    {
                        return None;
                    }
                }
            }
            Some(area)
        })
        .unwrap();
    println!("{result}");
}
