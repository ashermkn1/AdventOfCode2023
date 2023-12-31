use crate::util::grid::*;
use crate::util::point::*;
use fxhash::FxHashSet;
use itertools::Itertools;
use std::collections::HashMap;

const START: Point = Point::new(1, 0);
const END: Point = Point::new(139, 140);
type Graph = HashMap<Point, Vec<(usize, Point)>>;
fn contract_graph(mut graph: Graph) -> Graph {
    let corridors = graph
        .iter()
        .filter(|(_, adj)| adj.len() == 2)
        .map(|(&pos, _)| pos)
        .collect_vec();

    for pos in corridors {
        let [(d1, p1), (d2, p2)] = graph.remove(&pos).unwrap()[..] else {
            unreachable!()
        };

        let n1 = graph.get_mut(&p1).unwrap();
        if let Some(i) = n1.iter().position(|(_, neighbor)| *neighbor == pos) {
            n1[i] = (d1 + d2, p2);
        }

        let n2 = graph.get_mut(&p2).unwrap();
        if let Some(i) = n2.iter().position(|(_, neighbor)| *neighbor == pos) {
            n2[i] = (d1 + d2, p1);
        }
    }
    graph
}
#[aoc_generator(day23, part1)]
fn parse_input1(input: &str) -> Graph {
    let grid = Grid::parse(input);
    let mut graph = Graph::new();
    // build graph
    for (x, y) in (0..grid.width).cartesian_product(0..grid.height) {
        let pos = Point::new(x, y);
        let neighbors = match grid[pos] {
            b'#' => continue,
            b'.' => &ORTHOGONAL,
            b'^' => &ORTHOGONAL[0..1],
            b'v' => &ORTHOGONAL[1..2],
            b'<' => &ORTHOGONAL[2..3],
            b'>' => &ORTHOGONAL[3..4],
            _ => unreachable!("malformed input"),
        };
        let adj = graph.entry(pos).or_default();
        for &dir in neighbors {
            let next = pos + dir;
            if grid.contains(next) && grid[next] != b'#' {
                adj.push((1, next));
            }
        }
    }

    contract_graph(graph)
}

fn dfs(graph: &Graph, visited: &mut FxHashSet<Point>, pos: Point) -> Option<usize> {
    if pos == END {
        return Some(0);
    }
    let mut max = None;
    for &(edge, next) in &graph[&pos] {
        if !visited.contains(&next) {
            visited.insert(next);
            if let Some(dist) = dfs(graph, visited, next) {
                max = Some(max.unwrap_or(0).max(edge + dist));
            }
            visited.remove(&next);
        }
    }
    max
}
#[aoc(day23, part1)]
fn part1(input: &Graph) -> usize {
    dfs(input, &mut FxHashSet::default(), START).unwrap()
}

#[aoc_generator(day23, part2)]
fn parse_input2(input: &str) -> Graph {
    let grid = Grid::parse(input);
    let mut graph = Graph::new();
    // build graph
    for (x, y) in (0..grid.width).cartesian_product(0..grid.height) {
        let pos = Point::new(x, y);
        if grid[pos] == b'#' {
            continue;
        }
        let adj = graph.entry(pos).or_default();
        for dir in ORTHOGONAL {
            let next = pos + dir;
            if grid.contains(next) && grid[next] != b'#' {
                adj.push((1, next));
            }
        }
    }

    contract_graph(graph)
}
#[aoc(day23, part2)]
fn part2(input: &Graph) -> usize {
    dfs(input, &mut FxHashSet::default(), START).unwrap()
}
