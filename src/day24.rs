use std::fmt::Debug;
use std::{io::BufRead, str::FromStr};

use itertools::Itertools;

use nalgebra::{ComplexField, Matrix2, Scalar, Vector2, Vector3};
use num_traits::Float;

#[derive(Debug)]
struct Hailstone<T> {
    position: (T, T, T),
    velocity: (T, T, T),
}

impl<T: FromStr> FromStr for Hailstone<T>
where
    T::Err: Debug,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (positions, velocities) = s.split_once(" @ ").unwrap();

        let mut positions = positions
            .split(", ")
            .map(|x| x.trim().parse::<T>().unwrap());

        let mut velocities = velocities
            .split(", ")
            .map(|x| x.trim().parse::<T>().unwrap());

        let hailstone = Hailstone {
            position: (
                positions.next().unwrap(),
                positions.next().unwrap(),
                positions.next().unwrap(),
            ),
            velocity: (
                velocities.next().unwrap(),
                velocities.next().unwrap(),
                velocities.next().unwrap(),
            ),
        };

        assert!(positions.next().is_none());
        assert!(velocities.next().is_none());

        Ok(hailstone)
    }
}

impl<T: Float + ComplexField> Hailstone<T> {
    fn intersects_xy(&self, other: &Self) -> Option<(T, T)> {
        let (x1, y1, _) = self.position;
        let (vx1, vy1, _) = self.velocity;

        let (x2, y2, _) = other.position;
        let (vx2, vy2, _) = other.velocity;

        let v = Vector2::new(x2 - x1, y2 - y1);
        let m = Matrix2::new(vx1, -vx2, vy1, -vy2);

        let m_inv = m.try_inverse()?;

        let t = m_inv * v;

        if t[0].is_sign_negative() || t[1].is_sign_negative() {
            return None;
        }

        Some((x1 + vx1 * t[0], y1 + vy1 * t[0]))
    }
}

fn process(mut input: impl BufRead, min: f32, max: f32) -> usize {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let hailstones: Vec<_> = str
        .lines()
        .map(|line| line.parse::<Hailstone<f32>>().unwrap())
        .collect();

    hailstones
        .iter()
        .combinations(2)
        .filter(|x| {
            x[0].intersects_xy(x[1])
                .map(|(x, y)| x >= min && x <= max && y >= min && y <= max)
                .unwrap_or(false)
        })
        .count()
}

pub fn star_one(input: impl BufRead) -> String {
    process(input, 200_000_000_000_000.0, 400_000_000_000_000.0).to_string()
}

fn convert_to_vector3<T: Scalar>(position: &(T, T, T)) -> Vector3<T> {
    Vector3::from([position.0.clone(), position.1.clone(), position.2.clone()])
}

/// Reworked from https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kxqjg33/
pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();

    let hailstones: Vec<_> = str
        .lines()
        .map(|line| line.parse::<Hailstone<f64>>().unwrap())
        .collect();

    let p1 =
        convert_to_vector3(&hailstones[1].position) - convert_to_vector3(&hailstones[0].position);
    let v1 =
        convert_to_vector3(&hailstones[1].velocity) - convert_to_vector3(&hailstones[0].velocity);
    let p2 =
        convert_to_vector3(&hailstones[2].position) - convert_to_vector3(&hailstones[0].position);
    let v2 =
        convert_to_vector3(&hailstones[2].velocity) - convert_to_vector3(&hailstones[0].velocity);

    let t1 = -((p1.cross(&p2)).dot(&v2) / ((v1.cross(&p2)).dot(&v2)));
    let t2 = -((p1.cross(&p2)).dot(&v1) / ((p1.cross(&v2)).dot(&v1)));

    let c1 = convert_to_vector3(&hailstones[1].position)
        + t1 * convert_to_vector3(&hailstones[1].velocity);
    let c2 = convert_to_vector3(&hailstones[2].position)
        + t2 * convert_to_vector3(&hailstones[2].velocity);
    let v = (c2 - c1) / (t2 - t1);
    let p = c1 - t1 * v;

    (p[0] + p[1] + p[2]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            process(
                Cursor::new(
                    b"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
                ),
                7.0,
                27.0
            ),
            2
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
            )),
            "47"
        );
    }
}
