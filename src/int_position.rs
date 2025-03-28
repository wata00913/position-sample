use crate::position::Position;
use std::{fmt::Debug, mem};

pub struct IntPosition {
    records: Vec<(String, i32)>,
}

impl IntPosition {
    fn new() -> Self {
        IntPosition { records: vec![] }
    }
}

impl Position for IntPosition {
    fn from_keys(keys: Vec<&str>) -> Self {
        let mut pos = Self::new();
        pos.records = keys
            .iter()
            .enumerate()
            .map(|(idx, key)| (key.to_string(), idx as i32))
            .collect();
        pos
    }

    fn add(&mut self, key: &str) {
        let last = self.records.last();
        match last {
            Some(l) => self.records.push((key.to_string(), l.1 + 1)),
            None => self.records.push((key.to_string(), 1)),
        }
    }

    fn insert(&mut self, key: &str, idx: usize) {
        let updates = &mut self.records[idx..];
        for r in updates.iter_mut() {
            r.1 = r.1 + 1;
        }
        self.records.insert(idx, (key.to_string(), idx as i32));
    }

    fn shift(&mut self, from: usize, to: usize) {
        let t = self.records[to].clone();
        let t_i = t.1;
        let f = mem::replace(&mut self.records[from], t);
        self.records[from].1 = f.1;
        self.records[to] = f;
        self.records[to].1 = t_i;
    }

    fn delete(&mut self, idx: usize) -> String {
        let updates = &mut self.records[idx + 1..];
        for r in updates.iter_mut() {
            r.1 = r.1 - 1;
        }
        self.records.remove(idx).0
    }

    fn keys(&self) -> Vec<String> {
        self.records.iter().map(|r| r.0.clone()).collect()
    }

    fn order(&self) -> Vec<&str> {
        let mut rs: Vec<(&str, i32)> = self.records.iter().map(|r| (r.0.as_str(), r.1)).collect();
        rs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        rs.iter().map(|r| r.0).collect()
    }
}

impl Debug for IntPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indices: Vec<i32> = self.records.iter().map(|r| r.1.clone()).collect();
        indices.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::*;

    #[test]
    fn from_keys() {
        test_from_keys::<IntPosition>();
    }

    #[test]
    fn add() {
        test_add::<IntPosition>();
    }

    #[test]
    fn insert() {
        test_insert::<IntPosition>();
    }

    #[test]
    fn shift() {
        test_shift::<IntPosition>();
    }

    #[test]
    fn delete() {
        test_delete::<IntPosition>();
    }
}
