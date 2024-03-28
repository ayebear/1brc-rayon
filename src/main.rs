use anyhow::Result;
use rayon::prelude::*;
use std::fs;

fn main() -> Result<()> {
    let data = fs::read_to_string("measurements.txt")?;
    println!("Done reading file");
    let results = data
        .par_lines()
        .flat_map(parse_line)
        .fold(Station::default, |a: Station, b: Line| {
            a.merge(Station::from_line(b))
        })
        .reduce(Station::default, |a, b| a.merge(b));
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
    fn from_line(line: Line) -> Self {
        // todo: deal with name
        let (_name, value) = line;
        Self {
            min: value,
            max: value,
            total: value,
            count: 1,
        }
    }

    fn merge(self, other: Self) -> Self {
        if self.count == 0 {
            return other;
        }
        if other.count == 0 {
            return self;
        }
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
            total: self.total + other.total,
            count: self.count + other.count,
        }
    }
}
