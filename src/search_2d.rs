extern crate test;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.len() == 0 || matrix[0].len() == 0 {
        false
    } else {
        let first_row_n = 0usize;
        let last_row_n = matrix.len() - 1usize;

        let first_col_n = 0usize;
        let last_col_n = matrix[0].len() - 1usize;

        let mut l = 0usize;
        let mut r = last_row_n;
        let mut row = 0usize;
        let mut col = 0usize;

        while l < r {
            row = (l + r) / 2;

            if matrix[row][first_col_n] <= target && target <= matrix[row][last_col_n] {
                break;
            } else if target < matrix[row][0] {
                r = row.checked_sub(1).unwrap_or(first_row_n);
                row = r;
            } else {
                l = row.checked_add(1).unwrap_or(last_row_n);
                row = l;
            }
        }

        l = 0;
        r = matrix[row].len() - 1;

        while l < r {
            col = (l + r) / 2;

            if target == matrix[row][col] {
                return true;
            } else if target < matrix[row][col] {
                r = col.checked_sub(1).unwrap_or(first_col_n);
                col = r;
            } else {
                l = col.checked_add(1).unwrap_or(last_col_n);
                col = l;
            }
        }

        matrix[row][col] == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::prelude::*;
    use test::Bencher;

    #[test]
    fn search_test() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(search_matrix(matrix.clone(), 3), true);
        assert_eq!(search_matrix(matrix.clone(), 1), true);
        assert_eq!(search_matrix(matrix.clone(), 60), true);
        assert_eq!(search_matrix(matrix.clone(), 16), true);
        assert_eq!(search_matrix(matrix.clone(), 17), false);
        assert_eq!(search_matrix(matrix.clone(), 21), false);
        assert_eq!(search_matrix(matrix.clone(), 66), false);
        assert_eq!(search_matrix(vec![vec![1, 1]], -66), false);

        assert_eq!(search_matrix(vec![], 3), false);
        assert_eq!(search_matrix(vec![vec![], vec![]], 3), false);
    }
}