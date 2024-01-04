use std::collections::BTreeMap;
use advent_of_code::parsers::lines;
use itertools::Itertools;

advent_of_code::solution!(8);
type Graph = BTreeMap<String, (String, String)>;
fn parse_node(line: &str) -> (String, (String, String)) {
    let (node, maps) = line.split_once(" = ").unwrap();
    let (left, right) = maps.split_once(", ").unwrap();
    return (
        node.to_string(),
        (
            left.trim_matches(|c| c == '(' || c == ')').to_string(),
            right.trim_matches(|c| c == '(' || c == ')').to_string(),
        ),
    );
}
fn parse_graph(lns: Vec<&str>) -> Graph {
    let mut graph: Graph = BTreeMap::new();
    for l in lns {
        let (node, maps) = parse_node(l);
        graph.insert(node, maps);
    }
    graph
}

fn graph_move(graph: &Graph, instruction: char, node_id: &String) -> Option<String> {
    if let Some((left_edge, right_edge)) = graph.get(node_id) {
        let result = match instruction {
            'L' => Some(left_edge.to_string()),
            'R' => Some(right_edge.to_string()),
            _ => None,
        };
        return result;
    } else {
        return None;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lns = lines(input).collect_vec();
    let instructions = lns[0].chars();
    let graph = parse_graph(lns[1..].to_vec());
    let mut current_node = "AAA".to_string();
    let count = 
        instructions.cycle().enumerate().find_map( 
        |(ix, instruction)| {
            let next_node = graph_move(&graph, instruction, &current_node).unwrap();
            if next_node == "ZZZ".to_string() {
                return Some(ix + 1);
            }
            else {
            current_node = next_node;
            None
            }
        }
    )
        .expect("This should not happen :)");
    Some(count as u32)
}


fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a
    }
    return gcd(b, a % b)
} 

fn lcm(a: u64, b: u64) -> u64 {
    return (a * b) / gcd(a, b)
}



pub fn part_two(input: &str) -> Option<u64> {
    // input graph has cycles ends on the first element with a Z at the end, So no cycle finding algos
    // of course.
    let lns = lines(input).collect_vec();
    let instructions = lns[0].chars().collect_vec();
    let graph = parse_graph(lns[1..].to_vec());
    let cycles_len = graph
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|start| {
            let mut current_node = start.to_string();
            let count = 
                instructions.iter().cycle().enumerate().find_map( 
                |(ix, instruction)| {
                    let next_node = graph_move(&graph, *instruction, &current_node).unwrap();
                    if next_node.ends_with('Z') {
                        return Some(ix + 1);
                    }
                    else {
                        current_node = next_node;
                        None
                    }
                })
            .expect("It should find a cycle");
            return count as u64;
            }).collect_vec();

    Some(cycles_len.iter().fold(1, |a, b| lcm(a, *b)))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
