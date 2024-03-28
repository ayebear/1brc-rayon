use anyhow::Result;
use rayon::prelude::*;
use std::{collections::BTreeMap, fs};

fn main() -> Result<()> {
    let data = fs::read_to_string("measurements.txt")?;
    println!("Done reading file");
    let results = data
        .par_lines()
        .flat_map(parse_line)
        .fold(Stations::default, Stations::insert_line)
        .reduce(Stations::default, Stations::merge);
    println!("{results:?}");
    Ok(())
}

type Line = (String, f32);
fn parse_line(line: &str) -> Option<Line> {
    let mut parts = line.split(';');
    let name = parts.next()?.to_string();
    let value = parts.next()?.parse().ok()?;
    Some((name, value))
}

#[derive(Default, Clone, Copy, Debug)]
struct Station {
    min: f32,
    max: f32,
    total: f32,
    count: usize,
}

impl Station {
    fn from_value(value: f32) -> Self {
        Self {
            min: value,
            max: value,
            total: value,
            count: 1,
        }
    }

    fn add_value(&mut self, value: f32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.total += value;
        self.count += 1;
    }
}

#[derive(Default, Clone, Debug)]
struct Stations {
    map: BTreeMap<String, Station>,
}

impl Stations {
    fn insert_line(mut self, line: Line) -> Self {
        let (name, value) = line;
        self.map
            .entry(name)
            .and_modify(|e| e.add_value(value))
            .or_insert_with(|| Station::from_value(value));
        self
    }

    fn merge(mut self, other: Self) -> Self {
        // TODO: Actually merge
        self.map.par_extend(other.map);
        self
    }
}
