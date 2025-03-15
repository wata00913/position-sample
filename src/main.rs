use std::mem;

fn main() {
    test_from_keys::<IntPosition>();
    test_add::<IntPosition>();
    test_insert::<IntPosition>();
    test_shift::<IntPosition>();
    test_delete::<IntPosition>();
}

fn test_from_keys<T: Position>() {
    let data = vec!["a", "b", "c", "d"];
    let pos = T::from_keys(data);

    assert_eq!(vec!["a", "b", "c", "d"], pos.keys())
}

fn test_add<T: Position>() {
    let mut pos = T::from_keys(vec!["a", "b"]);
    pos.add("c");

    assert_eq!(vec!["a", "b", "c"], pos.keys())
}

fn test_insert<T: Position>() {
    let mut pos = T::from_keys(vec!["a", "b", "c"]);
    // a, d, b, c
    pos.insert("d", 1); // aとbの間に挿入。Resultで結果の判定

    assert_eq!(vec!["a", "d", "b", "c"], pos.keys())
}

fn test_shift<T: Position>() {
    let mut pos = T::from_keys(vec!["a", "b", "c", "d"]);

    pos.shift(1, 3);

    assert_eq!(vec!["a", "d", "c", "b"], pos.keys())
}

fn test_delete<T: Position>() {
    let mut pos = T::from_keys(vec!["a", "b", "c", "d"]);

    let deleted = pos.delete(2);
    assert_eq!("c", deleted);
    assert_eq!(vec!["a", "b", "d"], pos.keys())
}

trait Position {
    fn from_keys(keys: Vec<&str>) -> Self;
    fn add(&mut self, key: &str);
    fn insert(&mut self, key: &str, idx: usize);
    fn shift(&mut self, from: usize, to: usize);
    fn delete(&mut self, idx: usize) -> String;
    fn keys(&self) -> Vec<String>;
}

struct IntPosition {
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
        let f = mem::replace(&mut self.records[from], t);
        self.records[to] = f;
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
}
