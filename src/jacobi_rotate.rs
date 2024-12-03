

pub fn jacobi_rotate1() {

}

fn max_off_diagonal(matrix: &Vec<Vec<f64>>) -> (usize, usize, f64) {
    let n = matrix.len();
    let mut max_value = 0.0;
    let mut p = 0;
    let mut q = 1;

    for i in 0..n {
        for j in (i + 1)..n {
            if matrix[i][j].abs() > max_value {
                max_value = matrix[i][j].abs();
                p = i;
                q = j;
            }
        }
    }
    (p, q, max_value)
}

pub fn jacobi_rotate(matrix: &mut Vec<Vec<f64>>, tol: f64, max_iter: usize) -> Vec<f64> {
    let n = matrix.len();
    let mut iter = 0;

    // Iterate until off-diagonal elements are sufficiently small
    loop {
        let (p, q, max_value) = max_off_diagonal(matrix);

        if max_value < tol || iter >= max_iter {
            break;
        }

        // Calculate rotation parameters
        let theta = 0.5 * ((2.0 * matrix[p][q]) / (matrix[q][q] - matrix[p][p])).atan();
        let cos = theta.cos();
        let sin = theta.sin();

        // Apply rotation
        let mut temp_matrix = matrix.clone();
        for i in 0..n {
            for j in 0..n {
                if i != p && i != q && j != p && j != q {
                    temp_matrix[i][j] = matrix[i][j];
                }
            }
        }

        // Update matrix with rotation
        for i in 0..n {
            if i != p && i != q {
                temp_matrix[p][i] = cos * matrix[p][i] - sin * matrix[q][i];
                temp_matrix[i][p] = temp_matrix[p][i];

                temp_matrix[q][i] = sin * matrix[p][i] + cos * matrix[q][i];
                temp_matrix[i][q] = temp_matrix[q][i];
            }
        }

        temp_matrix[p][p] = cos * cos * matrix[p][p] + sin * sin * matrix[q][q]
            - 2.0 * sin * cos * matrix[p][q];
        temp_matrix[q][q] = sin * sin * matrix[p][p] + cos * cos * matrix[q][q]
            + 2.0 * sin * cos * matrix[p][q];
        temp_matrix[p][q] = 0.0;
        temp_matrix[q][p] = 0.0;

        *matrix = temp_matrix;

        iter += 1;
    }

    // Extract eigenvalues from the diagonal
    (0..n).map(|i| matrix[i][i]).collect()
}