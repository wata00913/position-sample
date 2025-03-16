use std::fmt::Debug;

pub trait Position: Debug {
    fn from_keys(keys: Vec<&str>) -> Self;
    fn add(&mut self, key: &str);
    fn insert(&mut self, key: &str, idx: usize);
    fn shift(&mut self, from: usize, to: usize);
    fn delete(&mut self, idx: usize) -> String;
    fn keys(&self) -> Vec<String>;
}

pub fn test_from_keys<T: Position>() {
    let data = vec!["a", "b", "c", "d"];
    let pos = T::from_keys(data);

    assert_eq!(vec!["a", "b", "c", "d"], pos.keys())
}

pub fn test_add<T: Position>() {
    let mut pos = T::from_keys(vec!["a", "b"]);
    pos.add("c");

    assert_eq!(vec!["a", "b", "c"], pos.keys())
}

pub fn test_insert<T: Position>() {
    let mut pos = T::from_keys(vec!["a", "b", "c"]);
    // a, d, b, c
    pos.insert("d", 1); // aとbの間に挿入。Resultで結果の判定

    assert_eq!(vec!["a", "d", "b", "c"], pos.keys())
}

pub fn test_shift<T: Position>() {
    let mut pos = T::from_keys(vec!["a", "b", "c", "d"]);

    pos.shift(1, 3);

    assert_eq!(vec!["a", "d", "c", "b"], pos.keys())
}

pub fn test_delete<T: Position>() {
    let mut pos = T::from_keys(vec!["a", "b", "c", "d"]);

    let deleted = pos.delete(2);
    assert_eq!("c", deleted);
    assert_eq!(vec!["a", "b", "d"], pos.keys())
}
