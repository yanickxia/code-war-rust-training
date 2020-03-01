use std::collections::HashSet;

struct Sudoku {
    data: Vec<Vec<u32>>,
}


impl Sudoku {
    fn is_valid(&self) -> bool {
        let (mut i, mut j) = (0, 0);
        let sudo_len = (self.data.len() as f64).sqrt() as usize;
        for i in 0..(self.data.len() / sudo_len) {

            //init n set
            let mut sets = Vec::new();
            for k in 0..sudo_len {
                sets.push(HashSet::new());
            }

            for sub_row in 0..sudo_len {
                let row = i * sudo_len + sub_row;

                //fill
                for col in 0..self.data.len() {
                    let set_index = col / sudo_len;
                    let num = self.data[row][col];
                    sets[set_index].insert(num);
                }
            }

            //check
            for set in sets {
                if set.len() != sudo_len * sudo_len {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Sudoku;

    #[test]
    fn good_sudoku() {
        let good_sudoku_1 = Sudoku {
            data: vec![
                vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
                vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
                vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
                vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
                vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
                vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
                vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
                vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
                vec![1, 9, 5, 2, 8, 7, 6, 3, 4]
            ]
        };

        let good_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 4, 2, 3],
                vec![3, 2, 4, 1],
                vec![4, 1, 3, 2],
                vec![2, 3, 1, 4],
            ]
        };
        assert!(good_sudoku_1.is_valid());
        assert!(good_sudoku_2.is_valid());
    }

    #[test]
    fn bad_sudoku() {
        let bad_sudoku_1 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ]
        };

        let bad_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1],
            ]
        };
        assert!(!bad_sudoku_1.is_valid());
        assert!(!bad_sudoku_2.is_valid());

        let bad_sudoku_3 = Sudoku {
            data: vec![
                vec![1, 4, 1, 4],
                vec![3, 2, 3, 2],
                vec![1, 4, 1, 4],
                vec![3, 2, 3, 2],
            ]
        };
        assert!(!bad_sudoku_3.is_valid());
    }
}
