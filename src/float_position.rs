use crate::position::Position;
use std::{fmt::Debug, mem};

pub struct FloatPosition {
    records: Vec<(String, f64)>,
}

impl FloatPosition {
    pub fn new() -> Self {
        FloatPosition { records: vec![] }
    }
}

impl Position for FloatPosition {
    fn from_keys(keys: Vec<&str>) -> Self {
        let mut pos = Self::new();
        for k in keys {
            pos.add(k);
        }
        pos
    }

    fn add(&mut self, key: &str) {
        let last = self.records.last();
        match last {
            Some(l) => {
                if l.1 + 0.1 >= 1.0 {
                    self.records.push((key.to_string(), (l.1 + 1.0) / 2.0))
                } else {
                    self.records.push((key.to_string(), l.1 + 0.1))
                }
            }
            None => self.records.push((key.to_string(), 0.1)),
        }
    }

    fn insert(&mut self, key: &str, idx: usize) {
        let both_edges = (self.records[idx - 1].1, self.records[idx].1);
        let pos = (both_edges.0 + both_edges.1) / 2.0;

        self.records.insert(idx, (key.to_string(), pos));
    }

    fn shift(&mut self, from: usize, to: usize) {
        let t = self.records[to].clone();
        let t_i = t.1;
        let f = mem::replace(&mut self.records[from], t);
        self.records[from].1 = f.1;
        self.records[to] = f;
        self.records[to].1 = t_i;
    }

    // 1, 2, 2.5, 3, 4, 5
    fn delete(&mut self, idx: usize) -> String {
        self.records.remove(idx).0
    }

    fn keys(&self) -> Vec<String> {
        self.records.iter().map(|r| r.0.clone()).collect()
    }

    fn order(&self) -> Vec<&str> {
        let mut rs: Vec<(&str, f64)> = self.records.iter().map(|r| (r.0.as_str(), r.1)).collect();
        rs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        rs.iter().map(|r| r.0).collect()
    }
}

impl Debug for FloatPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indices: Vec<f64> = self.records.iter().map(|r| r.1.clone()).collect();
        indices.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::*;

    #[test]
    fn from_keys() {
        test_from_keys::<FloatPosition>();
    }

    #[test]
    fn add() {
        test_add::<FloatPosition>();
    }

    #[test]
    fn insert() {
        test_insert::<FloatPosition>();
    }

    #[test]
    fn shift() {
        test_shift::<FloatPosition>();
    }

    #[test]
    fn delete() {
        test_delete::<FloatPosition>();
    }
}
