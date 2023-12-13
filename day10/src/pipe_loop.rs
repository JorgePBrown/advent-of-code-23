use std::cmp::max;

#[derive(Debug, PartialEq)]
pub struct PipeLoop {
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
struct Node {
    neighbours: Vec<usize>,
    value: char,
}

impl Node {
    fn is_connected(direction: &Direction, pipe: char) -> bool {
        match pipe {
            '|' => *direction == Direction::Up || *direction == Direction::Down,
            '-' => *direction == Direction::Left || *direction == Direction::Right,
            'L' => *direction == Direction::Down || *direction == Direction::Left,
            'J' => *direction == Direction::Down || *direction == Direction::Right,
            '7' => *direction == Direction::Up || *direction == Direction::Right,
            'F' => *direction == Direction::Up || *direction == Direction::Left,
            '.' => false,
            'S' => true,
            _ => unreachable!("{pipe} is not a valid pipe"),
        }
    }

    fn directions(c: char) -> Vec<Direction> {
        match c {
            '|' => vec![Direction::Up, Direction::Down],
            '-' => vec![Direction::Left, Direction::Right],
            'L' => vec![Direction::Up, Direction::Right],
            'J' => vec![Direction::Up, Direction::Left],
            '7' => vec![Direction::Down, Direction::Left],
            'F' => vec![Direction::Down, Direction::Right],
            '.' => vec![],
            'S' => vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for PipeLoop {
    fn from(value: &str) -> Self {
        let mut nodes = Vec::new();

        let width = value.find('\n').unwrap() + 1;
        let start_idx = value.find('S').unwrap();
        let mut visited = vec![(false, false, 0usize); value.len()];
        let mut to_visit: Vec<(usize, usize)> = vec![];

        nodes.push(Node {
            neighbours: vec![],
            value: 'S',
        });
        to_visit.push((0, start_idx));

        visited[start_idx] = (true, false, 0);

        while let Some((node_idx, map_idx)) = to_visit.pop() {
            visited[map_idx].1 = true;
            for direction in Node::directions(value.chars().nth(map_idx).unwrap()) {
                let map_idx = step(map_idx, &direction, width);
                if map_idx.is_none() {
                    continue;
                }

                let map_idx = map_idx.unwrap();
                if map_idx >= value.len() {
                    continue;
                }

                let pipe = value.chars().nth(map_idx).unwrap();

                let is_connected = Node::is_connected(&direction, pipe);

                if !is_connected {
                    continue;
                }

                let (is_created, is_visited, new_node_idx) = visited[map_idx];

                if is_created || is_visited {
                    nodes
                        .get_mut(node_idx)
                        .unwrap()
                        .neighbours
                        .push(new_node_idx);
                } else {
                    let new_node_idx = nodes.len();
                    nodes
                        .get_mut(node_idx)
                        .unwrap()
                        .neighbours
                        .push(new_node_idx);

                    visited[map_idx] = (true, false, new_node_idx);

                    let node = Node {
                        neighbours: vec![],
                        value: pipe,
                    };

                    to_visit.push((new_node_idx, map_idx));

                    nodes.push(node);
                }
            }
        }

        PipeLoop { nodes }
    }
}

impl PipeLoop {
    pub fn longest_distance_from_start(&self) -> u32 {
        let mut visited = vec![false; self.nodes.len()];
        let mut to_visit = vec![(0usize, 0u32)];

        let mut depth = 0;

        let mut i = 0;
        while i < to_visit.len() {
            let (idx, d) = to_visit[i];
            if visited[idx] {
                i += 1;
                continue;
            }
            visited[idx] = true;

            depth = max(depth, d);

            for neighbor in self.nodes[idx].neighbours.iter() {
                if !visited[*neighbor] {
                    to_visit.push((*neighbor, d + 1))
                }
            }

            i += 1;
        }

        depth
    }
}

fn step(i: usize, d: &Direction, w: usize) -> Option<usize> {
    match d {
        Direction::Up => {
            if i >= w {
                Some(i - w)
            } else {
                None
            }
        }
        Direction::Down => Some(i + w),
        Direction::Left => {
            if (i % w) == 0 {
                None
            } else {
                Some(i - 1)
            }
        }
        Direction::Right => {
            if w - (i % w) == 1 {
                None
            } else {
                Some(i + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pipe_loop::{Node, PipeLoop};

    #[test]
    fn parse_tight2() {
        let s = r"
S7.
LJ.
        "
        .trim();
        assert_eq!(
            PipeLoop::from(s),
            PipeLoop {
                nodes: vec![
                    Node {
                        neighbours: vec![1, 2],
                        value: 'S'
                    },
                    Node {
                        neighbours: vec![0, 3],
                        value: 'L'
                    },
                    Node {
                        neighbours: vec![3, 0],
                        value: '7'
                    },
                    Node {
                        neighbours: vec![2, 1],
                        value: 'J'
                    },
                ]
            }
        );
    }

    #[test]
    fn parse_tight() {
        let s = r"
S7
LJ
        "
        .trim();
        assert_eq!(
            PipeLoop::from(s),
            PipeLoop {
                nodes: vec![
                    Node {
                        neighbours: vec![1, 2],
                        value: 'S'
                    },
                    Node {
                        neighbours: vec![0, 3],
                        value: 'L'
                    },
                    Node {
                        neighbours: vec![3, 0],
                        value: '7'
                    },
                    Node {
                        neighbours: vec![2, 1],
                        value: 'J'
                    },
                ]
            }
        );
    }

    #[test]
    fn parse() {
        let s = r"
.....
.S77.
.LJ|.
.L-J.
.....
        "
        .trim();
        assert_eq!(
            PipeLoop::from(s),
            PipeLoop {
                nodes: vec![
                    Node {
                        neighbours: vec![1, 2],
                        value: 'S'
                    },
                    Node {
                        neighbours: vec![0, 3],
                        value: 'L'
                    },
                    Node {
                        neighbours: vec![3, 0],
                        value: '7'
                    },
                    Node {
                        neighbours: vec![2, 1],
                        value: 'J'
                    },
                ]
            }
        );
    }

    #[test]
    fn parse2() {
        let s = r"
.....
.S-7.
.|.|.
.L-J.
.....
        "
        .trim();
        assert_eq!(
            PipeLoop::from(s),
            PipeLoop {
                nodes: vec![
                    Node {
                        neighbours: vec![1, 2],
                        value: 'S'
                    },
                    Node {
                        neighbours: vec![0, 7],
                        value: '|'
                    },
                    Node {
                        neighbours: vec![0, 3],
                        value: '-'
                    },
                    Node {
                        neighbours: vec![4, 2],
                        value: '7'
                    },
                    Node {
                        neighbours: vec![3, 5],
                        value: '|'
                    },
                    Node {
                        neighbours: vec![4, 6],
                        value: 'J'
                    },
                    Node {
                        neighbours: vec![7, 5],
                        value: '-'
                    },
                    Node {
                        neighbours: vec![1, 6],
                        value: 'L'
                    }
                ]
            }
        );
    }
}
