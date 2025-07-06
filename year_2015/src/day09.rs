use aoc_tools::input;
use std::{
    collections::{HashMap, HashSet},
    u64,
};

pub(crate) fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u64 {
    let input = input::read_input(2015, 9);
    let mut graph = CityGraph::new();

    for line in input.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        match tokens.as_slice() {
            [from, "to", to, "=", distance] => {
                let distance: u64 = distance.parse().unwrap();
                graph.add_edge(from, to, distance)
            }
            _ => panic!("Unable to parse line"),
        }
    }

    graph.shortest_distance()
}

fn part2() -> u64 {
    let input = input::read_input(2015, 9);
    let mut graph = CityGraph::new();

    for line in input.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        match tokens.as_slice() {
            [from, "to", to, "=", distance] => {
                let distance: u64 = distance.parse().unwrap();
                graph.add_edge(from, to, distance)
            }
            _ => panic!("Unable to parse line"),
        }
    }

    graph.longest_distance()
}

struct CityGraph {
    nodes: HashMap<String, Vec<(String, u64)>>,
}

impl CityGraph {
    fn new() -> Self {
        CityGraph {
            nodes: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: &str, to: &str, distance: u64) {
        let from_entry = self.nodes.entry(from.to_string()).or_insert_with(Vec::new);
        from_entry.push((to.to_string(), distance));
        from_entry.sort_by_key(|(_, distance)| *distance);

        let to_entry = self.nodes.entry(to.to_string()).or_insert_with(Vec::new);
        to_entry.push((from.to_string(), distance));
        to_entry.sort_by_key(|(_, distance)| *distance);
    }

    fn shortest_distance(&self) -> u64 {
        let city_count = self.nodes.len();
        let mut shortest_distance = u64::MAX;
        for city in self.nodes.keys() {
            let mut visited = HashSet::new();
            visited.insert(city);

            let mut next = city;
            let mut total_distance = 0;

            while visited.len() < city_count {
                let (closest, distance) = self
                    .nodes
                    .get(next)
                    .unwrap()
                    .iter()
                    .filter(|(city, _)| !visited.contains(city))
                    .nth(0)
                    .unwrap();
                visited.insert(closest);
                total_distance += distance;
                next = closest;
            }

            shortest_distance = shortest_distance.min(total_distance)
        }

        shortest_distance
    }

    fn longest_distance(&self) -> u64 {
        // Get all paths
        let all_paths: Vec<Vec<(String, u64)>> = self.get_all_paths();

        // Calculate all distances
        let all_distances: Vec<u64> = all_paths
            .iter()
            .map(|path| path.iter().map(|(_, distance)| distance).sum())
            .collect();

        // Get the highest
        all_distances.into_iter().max().unwrap_or(0)
    }

    fn get_all_paths(&self) -> Vec<Vec<(String, u64)>> {
        let mut all_paths = Vec::new();

        for city in self.nodes.keys() {
            let mut visited = HashSet::new();
            let mut path = Vec::new();

            self.dfs_path(city, 0, &mut visited, &mut path, &mut all_paths);
        }

        all_paths
    }

    fn dfs_path(
        &self,
        current: &str,
        distance: u64,
        visited: &mut HashSet<String>,
        path: &mut Vec<(String, u64)>,
        all_paths: &mut Vec<Vec<(String, u64)>>,
    ) {
        visited.insert(current.to_string());
        path.push((current.to_string(), distance));

        if visited.len() == self.nodes.len() {
            all_paths.push(path.clone());
        } else {
            for (next_city, next_distance) in self.nodes.get(current).unwrap() {
                if !visited.contains(next_city) {
                    self.dfs_path(next_city, *next_distance, visited, path, all_paths);
                }
            }
        }

        path.pop();
        visited.remove(current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_the_shortest_distance() {
        let mut graph = CityGraph::new();
        graph.add_edge("London", "Dublin", 464);
        graph.add_edge("London", "Belfast", 518);
        graph.add_edge("Dublin", "Belfast", 141);

        assert_eq!(graph.shortest_distance(), 605)
    }

    #[test]
    fn calculates_the_longest_distance() {
        let mut graph = CityGraph::new();
        graph.add_edge("London", "Dublin", 464);
        graph.add_edge("London", "Belfast", 518);
        graph.add_edge("Dublin", "Belfast", 141);

        assert_eq!(graph.longest_distance(), 982)
    }
}
