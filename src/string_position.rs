use crate::position::Position;
use std::fmt::Debug;
use std::vec;

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

    fn byte_at(&self, idx: usize) -> Option<&u8> {
        match self {
            Self::Start | Self::End => return None,
            Self::Mid(st) => st.as_bytes().get(idx),
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Self::Start | Self::End => None,
            Self::Mid(st) => Some(st),
        }
    }
}

mod digits {
    const digits_bytes: &[u8] = b"0123456789";
    const digits_str: &str = "0123456789";

    pub fn index_of(st: &str) -> Option<usize> {
        let b = st.as_bytes().get(0).unwrap();
        digits_bytes.iter().position(|d| d == b)
    }

    pub fn max_index() -> usize {
        digits_str.len()
    }

    pub fn str_at(idx: usize) -> Option<&'static str> {
        digits_str.get(idx..=idx)
    }

    pub fn string_at(idx: usize) -> Option<String> {
        match str_at(idx) {
            Some(st) => Some(st.to_string()),
            None => None,
        }
    }

    pub fn byte_at(idx: usize) -> Option<&'static u8> {
        digits_bytes.get(idx)
    }
}

pub struct StringPosition {
    records: Vec<(String, String)>,
}

// sample code
// function midpoint(a, b, digits) {
//   if (b !== null && a >= b) {
//     throw new Error(a + ' >= ' + b)
//   }
//   if (a.slice(-1) === '0' || (b && b.slice(-1) === '0')) {
//     throw new Error('trailing zero')
//   }
//   // padding
//   if (b) {
//     let n = 0
//     while ((a.charAt(n) || '0') === b.charAt(n)) {
//       n++
//     }
//     if (n > 0) {
//       return b.slice(0, n) + midpoint(a.slice(n), b.slice(n), digits)
//     }
//   }
//   const digitA = a ? digits.indexOf(a.charAt(0)) : 0
//   const digitB = b !== null ? digits.indexOf(b.charAt(0)) : digits.length
//   if (digitB - digitA > 1) {
//     const midDigit = Math.round(0.5*(digitA + digitB))
//     return digits.charAt(midDigit)
//   } else {
//     if (b && b.length > 1) {
//       return b.slice(0, 1)
//     } else {
//       return digits.charAt(digitA) + midpoint(a.slice(1), null, digits)
//     }
//   }
// }

impl StringPosition {
    const DIGITS: &[u8] = b"0123456789";
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
                    return bst.to_string()
                        + &self.mid_point(Point::Mid(&ast[n..]), Point::Mid(&bst[n..]));
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
        //   (35, 50)
        //   if (digitB - digitA > 1) {
        //     const midDigit = Math.round(0.5*(digitA + digitB))
        //     return digits.charAt(midDigit)
        //   } else {
        //     (35, 40)
        //     if (b && b.length > 1) {
        //       return b.slice(0, 1)
        //     } else {
        //     (35, NULL)
        //       3 + (5, NULL)
        //       3 + 5 + ('', NULL)
        //       return digits.charAt(digitA) + midpoint(a.slice(1), null, digits)
        //     }
        //   }
        if r_idx - l_idx > 1 {
            let mid_idx = (0.5 * (l_idx + r_idx) as f64).round() as usize;
            digits::string_at(mid_idx).unwrap()
        } else {
            if let Point::Mid(st) = &r {
                st.get(0..=0).unwrap().to_string()
            } else {
                let other_l = Point::from_str(&l.as_str().unwrap()[1..]);
                digits::string_at(l_idx).unwrap() + &self.mid_point(other_l, Point::End)
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
        Self { records: vec![] }
    }

    fn add(&mut self, key: &str) {}
    fn insert(&mut self, key: &str, idx: usize) {}
    fn shift(&mut self, from: usize, to: usize) {}
    fn delete(&mut self, idx: usize) -> String {
        String::new()
    }
    fn keys(&self) -> Vec<String> {
        self.records.iter().map(|r| r.0.clone()).collect()
    }
    fn order(&self) -> Vec<&str> {
        vec![]
    }
}

impl Debug for StringPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indices: Vec<String> = self.records.iter().map(|r| r.1.clone()).collect();
        indices.fmt(f)
    }
}
