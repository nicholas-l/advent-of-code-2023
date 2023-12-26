use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
    ops::Add,
    str::FromStr,
};

use petgraph::{algo::all_simple_paths, graphmap::GraphMap, Undirected};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Position {
    Empty,
    Wall,
    Slope(Direction),
}

impl From<char> for Position {
    fn from(c: char) -> Self {
        match c {
            '#' => Position::Wall,
            '.' => Position::Empty,
            '>' => Position::Slope(Direction::Right),
            '<' => Position::Slope(Direction::Left),
            '^' => Position::Slope(Direction::Up),
            'v' => Position::Slope(Direction::Down),
            _ => panic!("Invalid character"),
        }
    }
}

struct Map(Vec<Vec<Position>>);

impl Map {
    fn get(&self, pos: &(isize, isize)) -> Option<&Position> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        self.0
            .get(pos.0 as usize)
            .and_then(|row| row.get(pos.1 as usize))
    }

    fn set_slopes_to_empty(&mut self) {
        for row in self.0.iter_mut() {
            for pos in row.iter_mut() {
                if let Position::Slope(_) = pos {
                    *pos = Position::Empty;
                }
            }
        }
    }

    fn start(&self) -> (isize, isize) {
        let x = self.0[0]
            .iter()
            .position(|p| matches!(p, Position::Empty))
            .unwrap();
        (0_isize, x as isize)
    }

    fn end(&self) -> (isize, isize) {
        let x = self.0[self.0.len() - 1]
            .iter()
            .position(|p| matches!(p, Position::Empty))
            .unwrap();
        (self.0.len() as isize - 1, x as isize)
    }

    fn into_graph(self) -> GraphMap<(isize, isize), usize, Undirected> {
        let mut graph = GraphMap::new();

        let start = self.start();

        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut visited = HashSet::new();
        while let Some(pos) = queue.pop_front() {
            if !visited.contains(&pos) {
                visited.insert(pos);
                let current_pos = pos;
                let next_positions = DIRS
                    .iter()
                    .map(|dir| dir + current_pos)
                    .filter(|pos| matches!(self.get(pos), Some(Position::Empty)))
                    .filter(|pos| !visited.contains(pos))
                    .collect::<Vec<_>>();
                let distance = 1;
                for next_pos in next_positions {
                    graph.add_edge(pos, next_pos, distance);
                    queue.push_back(next_pos);
                }
            }
        }

        graph
    }
}

fn collapse_graph(
    graph: &mut GraphMap<(isize, isize), usize, Undirected>,
    start: &(isize, isize),
    end: &(isize, isize),
) {
    for node in graph.nodes().collect::<Vec<_>>() {
        if node != *start && node != *end {
            let neighbors = graph.neighbors(node).collect::<Vec<_>>();
            if neighbors.len() == 2 {
                let (n1, n2) = (neighbors[0], neighbors[1]);
                let d1 = graph.edge_weight(node, n1).unwrap();
                let d2 = graph.edge_weight(node, n2).unwrap();
                graph.add_edge(n1, n2, d1 + d2);
                graph.remove_node(node);
            }
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect::<Vec<_>>();
        Ok(Map(map))
    }
}

impl Add<(isize, isize)> for &Direction {
    type Output = (isize, isize);

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        match self {
            Direction::Up => (rhs.0 - 1, rhs.1),
            Direction::Left => (rhs.0, rhs.1 - 1),
            Direction::Down => (rhs.0 + 1, rhs.1),
            Direction::Right => (rhs.0, rhs.1 + 1),
        }
    }
}

impl Add<&Direction> for (isize, isize) {
    type Output = (isize, isize);

    fn add(self, rhs: &Direction) -> Self::Output {
        rhs + self
    }
}

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];

pub fn star_one(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();
    let map: Map = str.parse().unwrap();

    let start = map.start();
    let end = map.end();

    let mut stack = vec![(start, 0, HashSet::new())];

    let mut max_steps = None;

    while let Some((pos, steps, mut path)) = stack.pop() {
        if path.contains(&pos) {
            continue;
        }
        path.insert(pos);

        if pos == end {
            max_steps = max_steps.max(Some(steps));
        }

        match map.get(&pos) {
            None | Some(Position::Wall) => continue,
            Some(Position::Slope(dir)) => {
                stack.push((dir + pos, steps + 1, path.clone()));
            }
            Some(Position::Empty) => stack.extend(
                DIRS.iter()
                    .map(|dir| {
                        let new_pos = dir + pos;
                        let path = path.clone();
                        (new_pos, steps + 1, path)
                    })
                    .filter(|(pos, _, _)| !path.contains(pos))
                    .filter(|(pos, _, _)| {
                        matches!(
                            map.get(pos),
                            Some(Position::Empty) | Some(Position::Slope(_))
                        )
                    }),
            ),
        }
    }

    max_steps.unwrap().to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut str = String::new();
    input.read_to_string(&mut str).unwrap();
    let mut map: Map = str.parse().unwrap();

    map.set_slopes_to_empty();
    let start = map.start();
    let end = map.end();

    let mut graph = map.into_graph();

    collapse_graph(&mut graph, &start, &end);

    all_simple_paths(&graph, start, end, 5, None)
        .map(|x: Vec<_>| {
            x.windows(2)
                .map(|edge| graph.edge_weight(edge[0], edge[1]).unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
            )),
            "94"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
            )),
            "154"
        );
    }
}
