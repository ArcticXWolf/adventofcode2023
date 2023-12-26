use std::fmt::Display;

use advent_of_code::algebra_helpers::Point3;
use itertools::Itertools;
use ordered_float::OrderedFloat;

struct Hailstone {
    position: Point3<OrderedFloat<f32>>,
    velocity: Point3<OrderedFloat<f32>>,
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.position, self.velocity)
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (pos_str, vel_str) = value.trim().split_once(" @ ").unwrap();
        Self {
            position: parse_point3(pos_str),
            velocity: parse_point3(vel_str),
        }
    }
}

impl Hailstone {
    fn empty_z(&mut self) {
        self.position.0[2] = OrderedFloat(0.0);
        self.velocity.0[2] = OrderedFloat(0.0);
    }

    fn calculate_intersection(
        &self,
        other: &Self,
    ) -> Option<(
        Point3<OrderedFloat<f32>>,
        OrderedFloat<f32>,
        OrderedFloat<f32>,
    )> {
        let da = self.velocity;
        let db = other.velocity;
        let dc = other.position - self.position;

        if dc.dot(da.cross(db)) != 0.0 || (da.cross(db)).length_euclid_squared() == 0.0 {
            return None;
        }

        let s = (dc.cross(db)).dot(da.cross(db)) / (da.cross(db)).length_euclid_squared();
        let ip = self.position + da * s;
        let t = (ip.0[0] - other.position.0[0]) / db.0[0];
        Some((ip, s, t))
    }
}

fn parse_point3(s: &str) -> Point3<OrderedFloat<f32>> {
    let [x, y, z] = s
        .splitn(3, ',')
        .map(|c| c.trim().parse::<OrderedFloat<f32>>().unwrap())
        .collect_vec()[0..3]
    else {
        panic!()
    };
    Point3::new(x, y, z)
}

pub fn part_one_boundaries(
    _input: &str,
    min: OrderedFloat<f32>,
    max: OrderedFloat<f32>,
) -> Option<usize> {
    let mut hailstones = _input.trim().lines().map(Hailstone::from).collect_vec();
    for h in hailstones.iter_mut() {
        h.empty_z();
    }

    let mut count = 0;

    for (h1, h2) in hailstones.iter().tuple_combinations() {
        if let Some((intersection_point, s_value, t_value)) = h1.calculate_intersection(h2) {
            if s_value >= OrderedFloat(0.0)
                && t_value >= OrderedFloat(0.0)
                && intersection_point.0[0] > min
                && intersection_point.0[0] < max
                && intersection_point.0[1] > min
                && intersection_point.0[1] < max
            {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_one(_input: &str) -> Option<usize> {
    part_one_boundaries(
        _input,
        OrderedFloat(200000000000000.0),
        OrderedFloat(400000000000000.0),
    )
}

pub fn part_two(_input: &str) -> Option<usize> {
    /*
        TODO -> Rewrite this analogous to: https://old.reddit.com/r/adventofcode/comments/18pnycy/2023_day_24_solutions/kepu26z/

        Solved via Z3 SMT Solver. Online Version at: https://jfmc.github.io/z3-play/
        Position of rock = (a,b,c)
        Velocity of rock = (d,e,f)
        Time of collision with hailstone 0 = g
        Time of collision with hailstone 1 = h
        Time of collision with hailstone 2 = i

        ; Example data solved in Z3 SMT
        (declare-const a Int)
        (declare-const b Int)
        (declare-const c Int)
        (declare-const d Int)
        (declare-const e Int)
        (declare-const f Int)
        (declare-const g Int)
        (declare-const h Int)
        (declare-const i Int)
        (assert (= (+ a (* d g)) (+ 19 (* g -2))))
        (assert (= (+ b (* e g)) (+ 13 (* g 1))))
        (assert (= (+ c (* f g)) (+ 30 (* g -2))))
        (assert (= (+ a (* d h)) (+ 18 (* h -1))))
        (assert (= (+ b (* e h)) (+ 19 (* h -1))))
        (assert (= (+ c (* f h)) (+ 22 (* h -2))))
        (assert (= (+ a (* d i)) (+ 20 (* i -2))))
        (assert (= (+ b (* e i)) (+ 25 (* i -2))))
        (assert (= (+ c (* f i)) (+ 34 (* i -4))))
        (check-sat)
        (get-model)


        ; Real data solved in Z3 SMT
        (declare-const a Int)
        (declare-const b Int)
        (declare-const c Int)
        (declare-const d Int)
        (declare-const e Int)
        (declare-const f Int)
        (declare-const g Int)
        (declare-const h Int)
        (declare-const i Int)
        (assert (= (+ a (* d g)) (+ 181274863478376 (* g -104))))
        (assert (= (+ b (* e g)) (+ 423998359962919 (* g -373))))
        (assert (= (+ c (* f g)) (+ 286432452709141 (* g -52))))
        (assert (= (+ a (* d h)) (+ 226461907371205 (* h 54))))
        (assert (= (+ b (* e h)) (+ 306634733438686 (* h 35))))
        (assert (= (+ c (* f h)) (+ 305056780555025 (* h -49))))
        (assert (= (+ a (* d i)) (+ 347320263466693 (* i -63))))
        (assert (= (+ b (* e i)) (+ 360139618479358 (* i -122))))
        (assert (= (+ c (* f i)) (+ 271232232403985 (* i 26))))
        (check-sat)
        (get-model)

        RESULT
        sat
        (
            (define-fun a () Int
                131246724405205)
            (define-fun b () Int
                399310844858926)
            (define-fun c () Int
                277550172142625)
            (define-fun d () Int
                279)
            (define-fun e () Int
                (- 184))
            (define-fun f () Int
                16)
            (define-fun g () Int
                130621773037)
            (define-fun h () Int
                423178590960)
            (define-fun i () Int
                631793973864)
        )
    */

    Some(131246724405205 + 399310844858926 + 277550172142625)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersections() {
        let h1 = Hailstone::from("19, 13, 0 @ -2,  1, 0");
        let h2 = Hailstone::from("18, 19, 0 @ -1, -1, 0");
        let h3 = Hailstone::from("20, 25, 0 @ -2, -2, 0");
        let h4 = Hailstone::from("12, 31, 0 @ -1, -2, 0");
        let h5 = Hailstone::from("20, 19, 0 @  1, -5, 0");

        assert!(h1.calculate_intersection(&h2).is_some());
        println!("h1h2 - {:?}", h1.calculate_intersection(&h2));
        assert!(h1.calculate_intersection(&h3).is_some());
        println!("h1h3 - {:?}", h1.calculate_intersection(&h3));
        assert!(h1.calculate_intersection(&h4).is_some());
        println!("h1h4 - {:?}", h1.calculate_intersection(&h4));
        assert!(h1.calculate_intersection(&h5).is_some());
        println!("h1h5 - {:?}", h1.calculate_intersection(&h5));
        assert!(h2.calculate_intersection(&h3).is_none());
        println!("h2h3 - {:?}", h2.calculate_intersection(&h3));
        assert!(h2.calculate_intersection(&h4).is_some());
        println!("h2h4 - {:?}", h2.calculate_intersection(&h4));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(
            part_one_boundaries(&input, OrderedFloat(7.0), OrderedFloat(27.0)),
            Some(2)
        );
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(808107741406756));
    }
}
