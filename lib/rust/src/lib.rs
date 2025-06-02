#[derive(Debug)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Grid<T> {
        Grid { grid }
    }

    pub fn ray(&self, (x, y): (usize, usize), (dx, dy): (i64, i64)) -> impl Iterator<Item = &T> {
        let (x, y) = (i64::try_from(x).unwrap(), i64::try_from(y).unwrap());
        (0..)
            .map(move |n| {
                self.grid
                    .get(usize::try_from(y + n * dy).unwrap())
                    .and_then(|row| row.get(usize::try_from(x + n * dx).unwrap()))
            })
            .take_while(Option::is_some)
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grid() {
        let _grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
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
