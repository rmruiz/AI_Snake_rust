#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn north(self) -> Self {
        Point { x: self.x, y: self.y.saturating_sub(1) }
    }

    pub fn south(self) -> Self {
        Point { x: self.x, y: self.y + 1 }
    }

    pub fn east(self) -> Self {
        Point { x: self.x + 1, y: self.y }
    }

    pub fn west(self) -> Self {
        Point { x: self.x.saturating_sub(1), y: self.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_directions() {
        let p = Point { x: 5, y: 5 };

        assert_eq!(p.north(), Point { x: 5, y: 4 });
        assert_eq!(p.south(), Point { x: 5, y: 6 });
        assert_eq!(p.east(), Point { x: 6, y: 5 });
        assert_eq!(p.west(), Point { x: 4, y: 5 });
    }

    #[test]
    fn test_point_saturating_bounds() {
        let p = Point { x: 0, y: 0 };

        assert_eq!(p.north(), Point { x: 0, y: 0 });
        assert_eq!(p.west(), Point { x: 0, y: 0 });
    }
}

