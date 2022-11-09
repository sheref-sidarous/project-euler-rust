use std::fs::File;
use std::io::{self, BufRead};

use crate::indexed_iter::IndexedIter;

struct TriangleCell<'a> {
    row: usize,
    col: usize,
    grid : &'a Triangle,
}

#[derive(Debug)]
struct Triangle(Vec<Vec<u32>>);

impl Triangle {
    fn cell_at(&self, row: usize, col: usize) -> TriangleCell {
        TriangleCell { row: row, col: col, grid: &self }
    }

    fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    fn collapse_last_row(self) -> Self {

        let mut new_rows = Vec::new();

        let n = self.0.len(); // number of rows

        // last 2 rows are merged into one
        let mut new_last_row = Vec::new();
        for col in 0..self.0[n - 2].len() {
            let cell = self.cell_at(n - 2, col);
            let new_val = cell.childen_max_value() + cell.value();
            new_last_row.push(new_val);
        }

        // n - 2 rows are just as is
        for (row, index) in IndexedIter::new(self.0.into_iter()) {
            if index < n - 2 {
                new_rows.push(row);
            } else  {
                break;
            }
        };

        new_rows.push(new_last_row);

        Triangle(new_rows)
    }
}

impl<'a> TriangleCell<'a> {
    fn value(&self) -> u32 {
        self.grid.0[self.row][self.col]
    }

    fn left_child(&self) -> Self {
        TriangleCell { row: self.row+1, col: self.col, grid: self.grid }
    }

    fn right_child(&self) -> Self {
        TriangleCell { row: self.row+1, col: self.col+1, grid: self.grid }
    }

    fn children(&self) -> [Self; 2] {
        [self.right_child(), self.left_child()]
    }

    fn childen_max_value(&self) -> u32 {
        std::cmp::max(self.left_child().value(), self.right_child().value())
    }
}

fn build_triangle(path : &str) -> Triangle {

    let mut rows = Vec::new();

    let file = File::open(path).unwrap();
    for line in io::BufReader::new(file).lines() {
        let mut cols = Vec::new();
        for value in line.unwrap().split(' ').map(|x| { x.parse::<u32>().unwrap()}) {
            cols.push(value);
        }
        rows.push(cols);
    };

    Triangle(rows)

}

pub fn algorithm (input_path : &str) {
    let mut triangle = build_triangle(input_path);
    println!("{:?}", triangle);

    while triangle.number_of_rows() > 1 {
        triangle = triangle.collapse_last_row();
        println!("{:?}", triangle);
    }
}

pub fn solution() {
    algorithm("res/problem_18.txt");
}