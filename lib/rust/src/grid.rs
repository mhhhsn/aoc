#[derive(Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

type Point = (usize, usize);
type Direction = (i64, i64);

pub fn chebyshev_ball(n: i64) -> impl Iterator<Item = Direction> {
    (-n..=n).flat_map(move |x| (-n..=n).map(move |y| (x, y)))
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Grid<T> {
        // unfold grid. assume that width/height are consistent
        Grid {
            height: grid.len(),
            width: grid.first().map_or(0, Vec::len),
            data: grid.into_iter().flatten().collect(),
        }
    }

    pub fn at(&self, (x, y): Point) -> Option<&T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.data.get(x + y * self.width)
        }
    }

    pub fn is_in(&self, p: Point) -> bool {
        self.at(p).is_some()
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| (x, y)))
    }

    pub fn move_pos(&self, (x, y): Point, (dx, dy): Direction, n: i64) -> Option<Point> {
        let x = (i64::try_from(x).ok()? + n * dx).try_into().ok()?;
        let y = (i64::try_from(y).ok()? + n * dy).try_into().ok()?;
        let p = (x, y);
        if self.is_in(p) { Some(p) } else { None }
    }

    pub fn ray(&self, p: Point, dp: Direction) -> impl Iterator<Item = &T> {
        (0..)
            .map(move |n| self.move_pos(p, dp, n))
            .take_while(Option::is_some)
            .flatten()
            .map(|p| self.at(p))
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn chebyshev_ball_zero() {
        assert_eq!(
            chebyshev_ball(0).collect::<HashSet<_>>(),
            [(0, 0)].iter().cloned().collect()
        )
    }

    #[test]
    fn chebyshev_ball_one() {
        assert_eq!(
            chebyshev_ball(1).collect::<HashSet<_>>(),
            [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 0),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ]
            .iter()
            .cloned()
            .collect()
        )
    }

    #[test]
    fn chebyshev_ball_two() {
        let points = chebyshev_ball(2).collect::<HashSet<_>>();
        assert_eq!(points.len(), 25);
        let norms = points
            .iter()
            .map(|(x, y)| x.abs().max(y.abs()))
            .collect::<Vec<_>>();

        assert_eq!(norms.iter().filter(|&&n| n == 0).count(), 1);
        assert_eq!(norms.iter().filter(|&&n| n == 1).count(), 8);
        assert_eq!(norms.iter().filter(|&&n| n == 2).count(), 16);
    }

    #[test]
    fn create_grid() {
        let _grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }

    #[test]
    fn at_exists() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(grid.at((1, 0)).unwrap(), &2);
    }

    #[test]
    fn at_not_exists() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert!(grid.at((17, 10)).is_none());
    }

    #[test]
    fn points_empty() {
        let grid = Grid::<u64>::new(vec![vec![]]);
        assert!(grid.points().eq(vec![]))
    }

    #[test]
    fn points_equal_size() {
        let grid = Grid::new(vec![vec![1, 2], vec![3, 4]]);
        let points = grid.points().collect::<HashSet<_>>();
        assert_eq!(
            points,
            [(0, 0), (0, 1), (1, 0), (1, 1)].iter().cloned().collect()
        )
    }

    #[test]
    fn points_unequal_size() {
        let grid = Grid::new(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
        let points = grid.points().collect::<HashSet<_>>();
        assert_eq!(
            points,
            [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)]
                .iter()
                .cloned()
                .collect()
        )
    }

    #[test]
    fn ray_empty() {
        let grid = Grid::<u64>::new(vec![vec![]]);
        assert!(grid.ray((1, 1), (1, 2)).eq(vec![].iter()))
    }

    #[test]
    fn ray_single_element() {
        let grid = Grid::<u64>::new(vec![vec![1]]);
        assert!(grid.ray((0, 0), (1, 2)).eq(vec![1].iter()))
    }

    #[test]
    fn ray_horizontal() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert!(grid.ray((0, 0), (1, 0)).eq(vec![1, 2, 3].iter()))
    }

    #[test]
    fn ray_vertical() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert!(grid.ray((1, 0), (0, 1)).eq(vec![2, 5, 8].iter()))
    }

    #[test]
    fn ray_diagonal() {
        let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert!(grid.ray((0, 0), (1, 2)).eq(vec![1, 8].iter()))
    }
}
