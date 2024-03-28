use anyhow::Result;
use rayon::prelude::*;
use std::{collections::BTreeMap, fs};

fn main() -> Result<()> {
    fs::read_to_string("measurements.txt")?
        .par_lines()
        .flat_map(parse_line)
        .fold(Stations::default, Stations::insert_line)
        .reduce(Stations::default, Stations::merge)
        .print();
    Ok(())
}

type Line = (String, f64);
fn parse_line(line: &str) -> Option<Line> {
    let mut parts = line.split(';');
    let name = parts.next()?.to_string();
    let value = parts.next()?.parse().ok()?;
    Some((name, value))
}

#[derive(Default, Clone, Copy, Debug)]
struct Station {
    min: f64,
    max: f64,
    total: f64,
    count: usize,
}

impl Station {
    fn new(value: f64) -> Self {
        Self {
            min: value,
            max: value,
            total: value,
            count: 1,
        }
    }

    fn add(&mut self, other: Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.total += other.total;
        self.count += other.count;
    }
}

#[derive(Default, Clone, Debug)]
struct Stations {
    map: BTreeMap<String, Station>,
}

impl Stations {
    fn insert_line(mut self, line: Line) -> Self {
        let (name, value) = line;
        let station = Station::new(value);
        self.map
            .entry(name)
            .and_modify(|e| e.add(station))
            .or_insert(station);
        self
    }

    fn merge(mut self, other: Self) -> Self {
        for (name, station) in other.map {
            self.map
                .entry(name)
                .and_modify(|e| e.add(station))
                .or_insert(station);
        }
        self
    }

    fn print(&self) {
        let results = self
            .map
            .iter()
            .map(|(name, station)| {
                format!(
                    "{name}={:.1}/{:.1}/{:.1}",
                    station.min,
                    station.total / station.count as f64,
                    station.max
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        println!("{{{results}}}");
    }
}
