use crate::util::point::*;
use std::ops::{ Index, IndexMut };

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid {
            width,
            height,
            bytes,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn find(&self, needle: T) -> Option<Point> {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        };
        self.bytes
            .iter()
            .position(|&h| h == needle)
            .map(to_point)
    }

    pub fn to_point(&self, index: usize) -> Option<Point> {
        let x = (index as i32) % self.width;
        let y = (index as i32) / self.width;
        let point = Point::new(x, y);
        match self.contains(point) {
            true => Some(point),
            false => None,
        }
    }

    pub fn find_all(&self, needle: T) -> Option<Vec<Point>> {
        let mut temp_vec = Vec::new();
        for (i, byte) in self.bytes.iter().enumerate() {
            if byte == &needle {
                //unwrap will never panic
                temp_vec.push(self.to_point(i).unwrap());
            }
        }
        if temp_vec.len() > 0 {
            Some(temp_vec)
        } else {
            None
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: i32, height: i32, value: T) -> Grid<T> {
        Grid {
            width,
            height,
            bytes: vec![value; (width * height) as usize],
        }
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![value; (self.width * self.height) as usize],
        }
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * index.y + index.x) as usize]
    }
}
