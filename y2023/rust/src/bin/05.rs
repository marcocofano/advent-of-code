// This day input for me was wrong, spent so much time to debug this until I realized that the problem was in the input
// and not the implementation. 
// The fourth seed range was having a seed mapping to zero. i removed the range altogether
// and boom, it worked and got the star. Very strange behaviour. The test was passing 
// and also using other people's code was giving me the zero result, so it was clear that the problem was in the input.

use itertools::Itertools;
use std::collections::VecDeque;
use std::ops::Range;
advent_of_code::solution!(5);

#[derive(Default, Debug)]
struct SeedLocMapping {
    mappings: Vec<StageMapping>,
}

#[derive(Default, Debug)]
struct StageMapping {
    mapping: Vec<Mapping>,
}

#[derive(Default, Debug)]
struct Mapping {
    source: Range<u64>,
    destination: Range<u64>,
}

impl Mapping {
    pub fn process_item(&self, item: u64) -> u64 {
        return self.destination.start + item - self.source.start;
    }

    pub fn process_range(&self, item_range: Range<u64>) -> (Vec<Range<u64>>, Option<Range<u64>>) {
        if item_range.start < self.source.start {
            if item_range.end > self.source.end {
                return (
                    vec![
                        item_range.start..self.source.start,
                        (self.source.end + 1)..(item_range.end + 1),
                    ],
                    Some(self.process_item(self.source.start)..self.process_item(self.source.start))
                );
            }
            if item_range.end < self.source.start {
                return (vec![item_range], None);
            }
            return (
                vec![item_range.start..self.source.start],
                Some(self.destination.start..self.process_item(self.source.end)),
            );
        }
        if item_range.start <= self.source.end {
            if item_range.end <= self.source.end {
                return (
                    vec![],
                    Some(self.process_item(item_range.start)..self.process_item(item_range.end)),
                );
            }
            return (
                vec![item_range.end..self.source.end],
                Some(self.process_item(item_range.start)..self.destination.end),
            );
        }
        return (vec![item_range], None);
    }
}

fn parse_maps(input_data: &mut VecDeque<&str>) -> StageMapping {
    let mut res = StageMapping { mapping: vec![] };
    input_data.pop_front().unwrap(); // removes the header
    loop {
        if input_data.is_empty() {
            return res;
        }
        let next = input_data.pop_front().unwrap();
        if next.trim().is_empty() {
            return res;
        }
        let (dest, src, rng) = next
            .split_ascii_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        res.mapping.push(Mapping {
            source: src..(src + rng),
            destination: dest..(dest + rng),
        })
    }
}

fn parse_seeds(seeds: &str) -> Vec<u64> {
    seeds
        .split(": ")
        .nth(1)
        .unwrap_or_default()
        .split(" ")
        .map(|seed| seed.parse::<u64>().unwrap_or_default())
        .collect_vec()
}

fn parse_input(input: &str) -> (Vec<u64>, SeedLocMapping) {
    let mut input_data: VecDeque<&str> = input.lines().collect();
    let seeds = parse_seeds(input_data.pop_front().unwrap());
    input_data.pop_front().unwrap();
    let mut maps = vec![];
    while !input_data.is_empty() {
        maps.push(parse_maps(&mut input_data));
    }
    return (seeds, SeedLocMapping { mappings: maps });
}

fn map_seed_to_location(seed: u64, map_list: &SeedLocMapping) -> u64 {
    map_list
        .mappings
        .iter()
        .fold(seed, |state, maps| map_item(state, maps))
}

fn map_item(item: u64, full_map: &StageMapping) -> u64 {
    // if let Some((source_range, destination_range)) = full_map.mapping
    //     .iter()
    //     .find(|(source_range, _)| source_range.contains(&item) ) {
    //         item + destination_range.start - source_range.start
    //     }
    // else {
    //     item
    // }
    for range_map in full_map.mapping.iter() {
        if range_map.source.contains(&item) {
            return range_map.process_item(item);
        }
    }
    return item
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);
    let location = seeds
        .iter()
        .map(|&s| map_seed_to_location(s, &maps))
        .min()
        .unwrap();
    return Some(location);
}
// Part Two specifics
//
//
fn parse_seeds_ranges(seeds: &str) -> Vec<Range<u64>> {
    seeds
        .split(": ")
        .nth(1)
        .unwrap_or_default()
        .split(" ")
        .map(|seed| seed.parse::<u64>().unwrap_or_default())
        .tuples()
        .map(|(a, b)| a..a + b)
        .collect_vec()
}

fn parse_input_2(input: &str) -> (Vec<Range<u64>>, SeedLocMapping) {
    let mut input_data: VecDeque<&str> = input.lines().collect();
    let seeds = parse_seeds_ranges(input_data.pop_front().unwrap());
    input_data.pop_front().unwrap();
    let mut maps = vec![];
    while !input_data.is_empty() {
        maps.push(parse_maps(&mut input_data));
    }
    return (seeds, SeedLocMapping { mappings: maps });
}

fn find_min_location(seed_range: Range<u64>, seed_loc_mappings: &SeedLocMapping) -> u64 {
    let mut remaining_seeds = vec![seed_range];
    let mut processed_seeds: Vec<Range<u64>> = vec![];
    for proc in seed_loc_mappings.mappings.iter() {
        for r in proc.mapping.iter() {
            let mut next_batch = vec![];
            for seed in remaining_seeds {
                let (mut rems, treated) = r.process_range(seed);
                if let Some(rng) = treated {
                    processed_seeds.push(rng);
                }
                next_batch.append(&mut rems);
            }
            remaining_seeds = next_batch;
        }
        remaining_seeds.append(&mut processed_seeds);
    }
    remaining_seeds
        .iter()
        .min_by(|a, b| a.start.cmp(&b.start))
        .unwrap()
        .start
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seed_ranges, maps) = parse_input_2(input);
    Some(
        seed_ranges
            .iter()
            .map(|s| find_min_location(s.clone(), &maps))
            .min()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
