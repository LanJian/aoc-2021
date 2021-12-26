use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaveSize {
    Big,
    Small,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cave {
    id: usize,
    size: CaveSize,
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("expected string to be not empty".to_string());
        }

        let size = if s.to_lowercase() == s {
            CaveSize::Small
        } else {
            CaveSize::Big
        };

        // we just encode each char into 2 digits
        let mut id = 0;
        for c in s.chars() {
            if !('a'..='z').contains(&c) && !('A'..='Z').contains(&c) {
                return Err("expected chars to be in [a-zA-Z]".to_string());
            }

            // +1 to avoid leading 0, mapping every char to a number > 0
            id = id * 100 + (c as usize - 'A' as usize + 1);
        }

        Ok(Self { id, size })
    }
}

impl Cave {
    pub const START: Self = Self {
        id: 5152335052,
        size: CaveSize::Small,
    };

    pub const END: Self = Self {
        id: 374636,
        size: CaveSize::Small,
    };

    pub fn is_start(&self) -> bool {
        self.id == 5152335052
    }

    pub fn is_end(&self) -> bool {
        self.id == 374636
    }

    pub fn is_big(&self) -> bool {
        self.size == CaveSize::Big
    }

    pub fn is_small(&self) -> bool {
        self.size == CaveSize::Small
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Edge(Cave, Cave);

impl FromStr for Edge {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut caves = s
            .split("-")
            .map(|x| Cave::from_str(x))
            .collect::<Result<Vec<Cave>, String>>()?;

        let b = caves
            .pop()
            .ok_or_else(|| "expected edge to contain 2 caves".to_string())?;
        let a = caves
            .pop()
            .ok_or_else(|| "expected edge to contain 2 caves".to_string())?;
        Ok(Self(a, b))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CaveSystem {
    edges: Vec<Edge>,
    adj: HashMap<Cave, HashSet<Cave>>,
}

impl From<Vec<Edge>> for CaveSystem {
    fn from(edges: Vec<Edge>) -> Self {
        let mut adj: HashMap<Cave, HashSet<Cave>> = HashMap::new();

        for edge in &edges {
            let set = adj.entry(edge.0).or_insert(HashSet::new());
            set.insert(edge.1);

            let set2 = adj.entry(edge.1).or_insert(HashSet::new());
            set2.insert(edge.0);
        }

        Self { edges, adj }
    }
}

impl CaveSystem {
    pub fn number_of_paths(&self) -> usize {
        let mut visited: HashMap<Cave, usize> = HashMap::new();
        self.dfs(Cave::START, &mut visited)
    }

    fn dfs(&self, cur_cave: Cave, visited: &mut HashMap<Cave, usize>) -> usize {
        if cur_cave.is_end() {
            return 1;
        }

        if *visited.get(&cur_cave).unwrap_or(&0) > 0 {
            return 0;
        }

        if cur_cave.is_small() {
            visited.entry(cur_cave).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut count = 0;

        let neighbours = &self.adj[&cur_cave];
        for neighbour in neighbours {
            count += self.dfs(*neighbour, visited);
        }

        visited.entry(cur_cave).and_modify(|e| *e -= 1);

        count
    }

    pub fn number_of_paths_with_more_time(&self) -> usize {
        let mut visited: HashMap<Cave, usize> = HashMap::new();
        self.dfs_with_more_time(Cave::START, &mut visited, true)
    }

    fn dfs_with_more_time(
        &self,
        cur_cave: Cave,
        visited: &mut HashMap<Cave, usize>,
        has_more_time: bool,
    ) -> usize {
        if cur_cave.is_end() {
            return 1;
        }

        let mut still_has_more_time = has_more_time;
        if *visited.get(&cur_cave).unwrap_or(&0) > 0 {
            if cur_cave.is_start() || !has_more_time {
                // no time to visit a small cave again
                return 0;
            }

            // otherwise we can visit, but we are using up the free time
            still_has_more_time = false;
        }

        if cur_cave.is_small() {
            visited.entry(cur_cave).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut count = 0;

        let neighbours = &self.adj[&cur_cave];
        for neighbour in neighbours {
            count += self.dfs_with_more_time(*neighbour, visited, still_has_more_time);
        }

        visited.entry(cur_cave).and_modify(|e| *e -= 1);

        count
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<Vec<Edge>, String> {
    lines
        .iter()
        .map(|x| Edge::from_str(x))
        .collect::<Result<Vec<Edge>, String>>()
}

pub fn part_one(edges: Vec<Edge>) -> usize {
    let cave_system = CaveSystem::from(edges);
    cave_system.number_of_paths()
}

pub fn part_two(edges: Vec<Edge>) -> usize {
    let cave_system = CaveSystem::from(edges);
    cave_system.number_of_paths_with_more_time()
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn char_arithmetic_test() {
        assert_eq!('a' as usize - 'A' as usize, 32);
        assert_eq!('b' as usize - 'A' as usize, 33);
        assert_eq!('z' as usize - 'A' as usize, 57);
        assert_eq!('A' as usize - 'A' as usize, 0);
        assert_eq!('Z' as usize - 'A' as usize, 25);
    }

    #[test]
    fn cave_from_str_test() {
        assert_eq!(
            Cave::from_str("start").unwrap(),
            Cave {
                id: 5152335052,
                size: CaveSize::Small
            }
        );
        assert_eq!(
            Cave::from_str("end").unwrap(),
            Cave {
                id: 374636,
                size: CaveSize::Small
            }
        );
        assert_eq!(
            Cave::from_str("A").unwrap(),
            Cave {
                id: 1,
                size: CaveSize::Big
            }
        );
        assert_eq!(
            Cave::from_str("Z").unwrap(),
            Cave {
                id: 26,
                size: CaveSize::Big
            }
        );
        assert_eq!(
            Cave::from_str("a").unwrap(),
            Cave {
                id: 33,
                size: CaveSize::Small
            }
        );
        assert_eq!(
            Cave::from_str("z").unwrap(),
            Cave {
                id: 58,
                size: CaveSize::Small
            }
        );
        assert_eq!(
            Cave::from_str("AZ").unwrap(),
            Cave {
                id: 126,
                size: CaveSize::Big
            }
        );
        assert_eq!(
            Cave::from_str("az").unwrap(),
            Cave {
                id: 3358,
                size: CaveSize::Small
            }
        );

        assert!(Cave::from_str("").is_err());
        assert!(Cave::from_str("123").is_err());
        assert!(Cave::from_str("a2").is_err());
    }

    #[test]
    fn edge_from_str_test() {
        assert_eq!(
            Edge::from_str("start-A").unwrap(),
            Edge(
                Cave::from_str("start").unwrap(),
                Cave::from_str("A").unwrap()
            )
        );
        assert_eq!(
            Edge::from_str("dc-end").unwrap(),
            Edge(
                Cave::from_str("dc").unwrap(),
                Cave::from_str("end").unwrap()
            )
        );
        assert_eq!(
            Edge::from_str("LN-dc").unwrap(),
            Edge(Cave::from_str("LN").unwrap(), Cave::from_str("dc").unwrap())
        );

        assert!(Edge::from_str("aa").is_err());
        assert!(Edge::from_str("").is_err());
        assert!(Edge::from_str("ab-").is_err());
        assert!(Edge::from_str("-").is_err());
        assert!(Edge::from_str("-A").is_err());
    }

    #[test]
    fn cave_system_from_edges_test() {
        let edges = vec![
            Edge::from_str("start-A").unwrap(),
            Edge::from_str("start-b").unwrap(),
            Edge::from_str("A-c").unwrap(),
            Edge::from_str("A-b").unwrap(),
            Edge::from_str("b-d").unwrap(),
            Edge::from_str("A-end").unwrap(),
            Edge::from_str("b-end").unwrap(),
        ];
        let actual = CaveSystem::from(edges.clone());

        let start = Cave::from_str("start").unwrap();
        let a = Cave::from_str("A").unwrap();
        let b = Cave::from_str("b").unwrap();
        let c = Cave::from_str("c").unwrap();
        let d = Cave::from_str("d").unwrap();
        let end = Cave::from_str("end").unwrap();

        let mut expected_adj: HashMap<Cave, HashSet<Cave>> = HashMap::new();
        expected_adj.insert(start, HashSet::from_iter(vec![a, b]));
        expected_adj.insert(a, HashSet::from_iter(vec![start, b, c, end]));
        expected_adj.insert(b, HashSet::from_iter(vec![start, a, d, end]));
        expected_adj.insert(c, HashSet::from_iter(vec![a]));
        expected_adj.insert(d, HashSet::from_iter(vec![b]));
        expected_adj.insert(end, HashSet::from_iter(vec![a, b]));

        let expected = CaveSystem {
            edges,
            adj: expected_adj,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_one_test_1() {
        let lines = vec![
            "start-A".to_string(),
            "start-b".to_string(),
            "A-c".to_string(),
            "A-b".to_string(),
            "b-d".to_string(),
            "A-end".to_string(),
            "b-end".to_string(),
        ];
        let edges = parse_input(lines).expect("could not parse the input");

        assert_eq!(part_one(edges), 10);
    }

    #[test]
    fn part_one_test_2() {
        let lines = vec![
            "fs-end".to_string(),
            "he-DX".to_string(),
            "fs-he".to_string(),
            "start-DX".to_string(),
            "pj-DX".to_string(),
            "end-zg".to_string(),
            "zg-sl".to_string(),
            "zg-pj".to_string(),
            "pj-he".to_string(),
            "RW-he".to_string(),
            "fs-DX".to_string(),
            "pj-RW".to_string(),
            "zg-RW".to_string(),
            "start-pj".to_string(),
            "he-WI".to_string(),
            "zg-he".to_string(),
            "pj-fs".to_string(),
            "start-RW".to_string(),
        ];
        let edges = parse_input(lines).expect("could not parse the input");

        assert_eq!(part_one(edges), 226);
    }

    #[test]
    fn part_two_test_1() {
        let lines = vec![
            "start-A".to_string(),
            "start-b".to_string(),
            "A-c".to_string(),
            "A-b".to_string(),
            "b-d".to_string(),
            "A-end".to_string(),
            "b-end".to_string(),
        ];
        let edges = parse_input(lines).expect("could not parse the input");

        assert_eq!(part_two(edges), 36);
    }

    #[test]
    fn part_two_test_2() {
        let lines = vec![
            "fs-end".to_string(),
            "he-DX".to_string(),
            "fs-he".to_string(),
            "start-DX".to_string(),
            "pj-DX".to_string(),
            "end-zg".to_string(),
            "zg-sl".to_string(),
            "zg-pj".to_string(),
            "pj-he".to_string(),
            "RW-he".to_string(),
            "fs-DX".to_string(),
            "pj-RW".to_string(),
            "zg-RW".to_string(),
            "start-pj".to_string(),
            "he-WI".to_string(),
            "zg-he".to_string(),
            "pj-fs".to_string(),
            "start-RW".to_string(),
        ];
        let edges = parse_input(lines).expect("could not parse the input");

        assert_eq!(part_two(edges), 3509);
    }
}
