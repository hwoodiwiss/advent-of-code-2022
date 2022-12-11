use std::{
    cell::RefCell,
    io::{self, BufRead},
    rc::Rc,
};

struct Forest {
    trees: Vec<u32>,
    columns: usize,
    rows: usize,
}

impl Forest {
    pub fn from_input(lines: &[String]) -> Self {
        let rows = lines.len();
        let columns = lines[0].len();
        let trees: Vec<u32> = lines
            .iter()
            .flat_map(|m| {
                m.chars()
                    .filter(|f| f.is_numeric())
                    .map(|c| c.to_digit(10).unwrap())
            })
            .collect();
        Self {
            rows,
            columns,
            trees,
        }
    }

    pub fn get_tree(&self, x: usize, y: usize) -> u32 {
        self.trees[x + y * self.columns]
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || x == self.rows - 1 || y == 0 || y == self.columns - 1 {
            true
        } else {
            self.is_visible_x(x, y) || self.is_visible_y(x, y)
        }
    }

    fn is_visible_x(&self, x: usize, y: usize) -> bool {
        let target_tree = self.get_tree(x, y);
        let mut visible_before = true;
        for test_x in 0..x {
            let test_tree = self.get_tree(test_x, y);
            if target_tree <= test_tree {
                visible_before = false;
                break;
            }
        }

        let mut visible_after = true;
        for test_x in x + 1..self.columns {
            let test_tree = self.get_tree(test_x, y);
            if target_tree <= test_tree {
                visible_after = false;
                break;
            }
        }

        visible_before || visible_after
    }

    fn is_visible_y(&self, x: usize, y: usize) -> bool {
        let target_tree = self.get_tree(x, y);
        let mut visible_before = true;
        for test_y in 0..y {
            let test_tree = self.get_tree(x, test_y);
            if target_tree <= test_tree {
                visible_before = false;
                break;
            }
        }

        let mut visible_after = true;
        for test_y in y + 1..self.columns {
            let test_tree = self.get_tree(x, test_y);
            if target_tree <= test_tree {
                visible_after = false;
                break;
            }
        }

        visible_before || visible_after
    }

    pub fn get_scenic_score(&self, x: usize, y: usize) -> u32 {
        let target_tree = self.get_tree(x, y);
        let mut neg_x = 0;
        for test_x in (0..x).rev() {
            let test_tree = self.get_tree(test_x, y);
            neg_x = neg_x + 1;
            if target_tree <= test_tree {
                break;
            }
        }

        let mut pos_x = 0;
        for test_x in x + 1..self.columns {
            let test_tree = self.get_tree(test_x, y);
            pos_x = pos_x + 1;
            if target_tree <= test_tree {
                break;
            }
        }

        let mut neg_y = 0;
        for test_y in (0..y).rev() {
            let test_tree = self.get_tree(x, test_y);
            neg_y = neg_y + 1;
            if target_tree <= test_tree {
                break;
            }
        }

        let mut pos_y = 0;
        for test_y in y + 1..self.rows {
            let test_tree = self.get_tree(x, test_y);
            pos_y = pos_y + 1;
            if target_tree <= test_tree {
                break;
            }
        }

        neg_x * neg_y * pos_x * pos_y
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|ln| ln.unwrap()).collect();
    let forest = Forest::from_input(&lines);
    let mut count = 0;
    let mut max_scenic_score = 0;
    for x in 0..forest.columns {
        for y in 0..forest.rows {
            if forest.is_visible(x, y) {
                count = count + 1;
            }
            let scenic_score = forest.get_scenic_score(x, y);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    println!("Part 1: {}", count);
    println!("Part 2: {}", max_scenic_score);
}

#[cfg(test)]
mod test {
    use crate::Forest;
    use test_case::test_case;

    #[test]
    fn test_from_input() {
        let lines = Vec::from([
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ]);

        let forest = Forest::from_input(&lines);

        assert_eq!(forest.columns, 5);
        assert_eq!(forest.rows, 5);
        assert_eq!(forest.trees.len(), 25);
    }

    #[test]
    fn test_get_tree() {
        let lines = Vec::from([
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ]);

        let forest = Forest::from_input(&lines);

        assert_eq!(forest.get_tree(0, 0), 3);
        assert_eq!(forest.get_tree(1, 0), 0);
        assert_eq!(forest.get_tree(0, 1), 2);
        assert_eq!(forest.get_tree(4, 4), 0);
    }

    #[test_case(3, 1, false)]
    #[test_case(0, 0, true)]
    #[test_case(4, 4, true)]
    #[test_case(1, 1, true)]
    #[test_case(2, 1, true)]
    #[test_case(1, 2, true)]
    fn test_is_visible(x: usize, y: usize, expect_visible: bool) {
        let lines = Vec::from([
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ]);

        let forest = Forest::from_input(&lines);

        assert_eq!(forest.is_visible(x, y), expect_visible);
    }

    #[test]
    fn test_get_scenic_score() {
        let lines = Vec::from([
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ]);

        let forest = Forest::from_input(&lines);

        assert_eq!(forest.get_scenic_score(2, 3), 8);
    }
}
