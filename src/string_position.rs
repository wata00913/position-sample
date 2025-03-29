use crate::position::Position;
use std::fmt::Debug;
use std::{mem, vec};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Point<'a> {
    Start,
    Mid(&'a str),
    End,
}

impl<'a> Point<'a> {
    fn from_str(st: &'a str) -> Self {
        match st {
            "" => Point::Start,
            _ => Point::Mid(st),
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Self::End => None,
            Self::Start => Some(""),
            Self::Mid(st) => Some(st),
        }
    }
}

mod digits {
    const DIGITS_BYTES: &[u8] = b"0123456789";
    const DIGITS_STR: &str = "0123456789";

    pub fn index_of(st: &str) -> Option<usize> {
        let b = st.as_bytes().get(0).unwrap();
        DIGITS_BYTES.iter().position(|d| d == b)
    }

    pub fn max_index() -> usize {
        DIGITS_STR.len()
    }

    pub fn str_at(idx: usize) -> Option<&'static str> {
        DIGITS_STR.get(idx..=idx)
    }

    pub fn string_at(idx: usize) -> Option<String> {
        match str_at(idx) {
            Some(st) => Some(st.to_string()),
            None => None,
        }
    }
}

pub struct StringPosition {
    records: Vec<(String, String)>,
}

impl StringPosition {
    const START: &str = "";

    pub fn new() -> Self {
        StringPosition { records: vec![] }
    }

    pub fn mid_point(&self, a: Point, b: Point) -> String {
        match (&a, &b) {
            (_, Point::Start) | (Point::End, _) => panic!("left >= right"),
            (Point::Start, Point::Mid(mb)) => {
                let n = self.padding(Self::START, &mb);
                if n > 0 {
                    return mb.to_string() + &self.mid_point(Point::Start, Point::Mid(&mb[n..]));
                } else {
                    return self.mid_digit(a, b);
                }
            }
            (Point::Mid(ast), Point::Mid(bst)) => {
                let n = self.padding(&ast, &bst);
                if n > 0 {
                    return bst[0..n].to_string()
                        + &self.mid_point(
                            Point::from_str(ast.get(n..).unwrap_or("")),
                            Point::from_str(bst.get(n..).unwrap_or("")),
                        );
                } else {
                    return self.mid_digit(a, b);
                }
            }
            (_, Point::End) => return self.mid_digit(a, b),
        }
    }

    fn mid_digit(&self, l: Point, r: Point) -> String {
        let l_idx = self.digit_left_idx(&l);
        let r_idx = self.digit_right_idx(&r);
        // (35, 50)
        if r_idx - l_idx > 1 {
            let mid_idx = (0.5 * (l_idx + r_idx) as f64).round() as usize;
            digits::string_at(mid_idx).unwrap()
        } else {
            // (35, 40)
            match &r {
                Point::Mid(st) if st.len() > 1 => st.get(0..=0).unwrap().to_string(),
                _ => {
                    // (35, NULL)
                    // 3 + (5, NULL)
                    // 3 + 5 + ('', NULL)
                    let l_str = &l.as_str().unwrap();
                    let other_l = Point::from_str(&l_str.get(1..).unwrap_or(""));
                    digits::string_at(l_idx).unwrap() + &self.mid_point(other_l, Point::End)
                }
            }
        }
    }

    fn digit_left_idx(&self, p: &Point) -> usize {
        match p {
            Point::Start => 0,
            Point::End => panic!("left is end!"),
            Point::Mid(st) => digits::index_of(st).unwrap(),
        }
    }

    fn digit_right_idx(&self, p: &Point) -> usize {
        match p {
            Point::Start => panic!("right is start!"),
            Point::End => digits::max_index(),
            Point::Mid(st) => digits::index_of(st).unwrap(),
        }
    }

    fn padding(&self, a: &str, b: &str) -> usize {
        let ba = a.as_bytes();
        let bb = b.as_bytes();

        let mut n = 0;
        loop {
            match (ba.get(n), bb.get(n)) {
                (Some(a), Some(b)) => {
                    if a == b {
                        n = n + 1;
                    } else {
                        break;
                    }
                }
                (None, Some(b)) => {
                    if b'0' == *b {
                        n = n + 1;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        n
    }
}

impl Position for StringPosition {
    fn from_keys(keys: Vec<&str>) -> Self {
        let mut pos = Self::new();
        for k in keys {
            pos.add(k);
        }
        pos
    }

    fn add(&mut self, key: &str) {
        let last = self.records.last();
        let mid = match last {
            Some(l) => self.mid_point(Point::Mid(&l.1), Point::End),
            None => self.mid_point(Point::Start, Point::End),
        };
        self.records.push((key.to_string(), mid));
    }

    fn insert(&mut self, key: &str, idx: usize) {
        let mid = match (self.records.get(idx - 1), self.records.get(idx)) {
            (None, Some(r)) => self.mid_point(Point::Start, Point::Mid(&r.1)),
            (Some(l), None) => self.mid_point(Point::Mid(&l.1), Point::End),
            (Some(l), Some(r)) => self.mid_point(Point::Mid(&l.1), Point::Mid(&r.1)),
            (None, None) => self.mid_point(Point::Start, Point::End),
        };
        self.records.insert(idx, (key.to_string(), mid));
    }

    fn shift(&mut self, from: usize, to: usize) {
        let t = self.records[to].clone();
        let t_i = t.1.clone();
        let f = mem::replace(&mut self.records[from], t);
        self.records[from].1 = f.1.clone();
        self.records[to] = f;
        self.records[to].1 = t_i;
    }
    fn delete(&mut self, idx: usize) -> String {
        self.records.remove(idx).0
    }

    fn keys(&self) -> Vec<String> {
        self.records.iter().map(|r| r.0.clone()).collect()
    }

    fn order(&self) -> Vec<&str> {
        let mut rs: Vec<(&str, &str)> = self
            .records
            .iter()
            .map(|r| (r.0.as_str(), r.1.as_str()))
            .collect();
        rs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        rs.iter().map(|r| r.0).collect()
    }
}

impl Debug for StringPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indices: Vec<String> = self.records.iter().map(|r| r.1.clone()).collect();
        indices.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::*;

    #[test]
    fn from_keys() {
        test_from_keys::<StringPosition>();
    }

    #[test]
    fn add() {
        test_add::<StringPosition>();
    }

    #[test]
    fn insert() {
        test_insert::<StringPosition>();
    }

    #[test]
    fn shift() {
        test_shift::<StringPosition>();
    }

    #[test]
    fn delete() {
        test_delete::<StringPosition>();
    }

    mod test_mid_point {
        use super::*;

        #[test]
        fn function_name() {
            assert_eq!("5", mid_point("[", "]"));
            assert_eq!("8", mid_point("5", "]"));
            assert_eq!("9", mid_point("8", "]"));
            assert_eq!("95", mid_point("9", "]"));
            assert_eq!("98", mid_point("95", "]"));
            assert_eq!("99", mid_point("98", "]"));
            assert_eq!("995", mid_point("99", "]"));
            assert_eq!("15", mid_point("1", "2"));
            assert_eq!("001001", mid_point("001", "001002"));
            assert_eq!("0010005", mid_point("001", "001001"));
            assert_eq!("3", mid_point("[", "5"));
            assert_eq!("2", mid_point("[", "3"));
            assert_eq!("1", mid_point("[", "2"));
            assert_eq!("05", mid_point("[", "1"));
            assert_eq!("08", mid_point("05", "1"));
        }

        fn mid_point(a: &str, b: &str) -> String {
            let pa = match a {
                "[" => Point::Start,
                _ => Point::Mid(a),
            };
            let pb = match b {
                "]" => Point::End,
                _ => Point::Mid(b),
            };

            let p = StringPosition::new();
            p.mid_point(pa, pb)
        }
    }
}
