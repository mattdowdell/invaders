//!

/// Represents a rectangle that starts at `(x1, y1)` at the bottom left and ends at `(x2, y2)` at
/// the top right.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Area {
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
}

impl Area {
    /// Create a new instance of area.
    pub fn new(left: f64, bottom: f64, right: f64, top: f64) -> Self {
        Self {
            left,
            bottom,
            right,
            top,
        }
    }

    /// Check if this area overlaps with another.
    pub fn overlaps(&self, other: Area) -> bool {
        self.left < other.right
            && self.right > other.left
            && self.top > other.bottom
            && self.bottom < other.top
    }
}

impl From<((f64, f64), (f64, f64))> for Area {
    fn from(value: ((f64, f64), (f64, f64))) -> Self {
        let ((left, bottom), (right, top)) = value;
        Self::new(left, bottom, right, top)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_area_overlaps() {
        let coordinates = vec![
            // same size
            (((0.0, 0.0), (1.0, 1.0)), ((0.0, 0.0), (1.0, 1.0))),
            // inner within outer
            (((0.0, 0.0), (3.0, 3.0)), ((1.0, 1.0), (2.0, 2.0))),
            // inner on edge of outer
            (((0.0, 0.0), (2.0, 2.0)), ((1.0, 1.0), (2.0, 2.0))),
            (((0.0, 0.0), (2.0, 2.0)), ((0.0, 0.0), (1.0, 1.0))),
            (((0.0, 0.0), (2.0, 2.0)), ((0.0, 1.0), (2.0, 1.0))),
            (((0.0, 0.0), (2.0, 2.0)), ((1.0, 0.0), (2.0, 1.0))),
            // overlap corners
            (((0.0, 0.0), (2.0, 2.0)), ((1.0, 1.0), (3.0, 3.0))),
            (((1.0, 1.0), (3.0, 3.0)), ((0.0, 0.0), (2.0, 2.0))),
            (((0.0, 1.0), (2.0, 3.0)), ((1.0, 0.0), (3.0, 2.0))),
            (((1.0, 0.0), (3.0, 2.0)), ((0.0, 1.0), (2.0, 3.0))),
            // overlap edges
            (((0.0, 0.0), (3.0, 3.0)), ((2.0, 1.0), (4.0, 2.0))),
            (((0.0, 0.0), (3.0, 3.0)), ((1.0, 2.0), (2.0, 4.0))),
            (((1.0, 0.0), (4.0, 3.0)), ((0.0, 1.0), (2.0, 2.0))),
            (((0.0, 1.0), (3.0, 4.0)), ((1.0, 0.0), (2.0, 2.0))),
        ];

        for (a, b) in coordinates.into_iter() {
            let area_a = Area::from(a);
            let area_b = Area::from(b);

            assert!(
                area_a.overlaps(area_b),
                "{:?} does not overlap {:?}",
                area_a,
                area_b
            );
        }
    }

    #[test]
    fn test_area_not_overlaps() {
        let coordinates = vec![
            // vertical alignment
            (((0.0, 0.0), (1.0, 1.0)), ((0.0, 1.0), (1.0, 2.0))),
            (((0.0, 1.0), (1.0, 2.0)), ((0.0, 0.0), (1.0, 1.0))),
            // vertical alignment with gap
            (((0.0, 0.0), (1.0, 1.0)), ((0.0, 2.0), (1.0, 3.0))),
            (((0.0, 2.0), (1.0, 3.0)), ((0.0, 0.0), (1.0, 1.0))),
            // horizontal alignment
            (((0.0, 0.0), (1.0, 1.0)), ((1.0, 0.0), (2.0, 1.0))),
            (((1.0, 0.0), (2.0, 1.0)), ((0.0, 0.0), (1.0, 1.0))),
            // horizontal alignment with gap
            (((0.0, 0.0), (1.0, 1.0)), ((2.0, 0.0), (3.0, 1.0))),
            (((2.0, 0.0), (3.0, 1.0)), ((0.0, 0.0), (1.0, 1.0))),
            // diagonal alignment
            (((0.0, 0.0), (1.0, 1.0)), ((1.0, 1.0), (2.0, 2.0))),
            (((0.0, 1.0), (1.0, 2.0)), ((1.0, 0.0), (2.0, 1.0))),
            // diagonal alignment with gap
            (((0.0, 0.0), (1.0, 1.0)), ((2.0, 2.0), (3.0, 3.0))),
            (((0.0, 2.0), (1.0, 3.0)), ((2.0, 0.0), (3.0, 1.0))),
        ];

        for (a, b) in coordinates.into_iter() {
            let area_a = Area::from(a);
            let area_b = Area::from(b);

            assert!(
                !area_a.overlaps(area_b),
                "{:?} overlaps {:?}",
                area_a,
                area_b
            );
        }
    }
}
