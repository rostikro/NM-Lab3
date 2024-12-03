fn norm(vector: &Vec<f64>) -> f64 {
    vector.iter().map(|x| x.abs()).fold(0.0, f64::max)
}

fn multiply_matrix_vector(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>) -> Vec<f64> {
    let n = matrix.len();
    let mut result = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            result[i] += matrix[i][j] * vector[j];
        }
    }
    result
}

pub fn power_method(matrix: &Vec<Vec<f64>>, iterations: usize) -> f64 {
    let n = matrix.len();
    let mut eigen_vector = vec![1.0; n]; // Start with an arbitrary vector
    let mut temp_vector = vec![0.0; n];

    let matrix_norm = matrix.iter()
        .map(|row| row.iter().map(|x| x.abs()).sum::<f64>())
        .fold(0.0, f64::max);

    let shifted_matrix: Vec<Vec<f64>> = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &val)| if i == j { matrix_norm - val } else { -val })
                .collect()
        })
        .collect();

    for _ in 0..iterations {
        // Multiply by the shifted matrix
        temp_vector = multiply_matrix_vector(&shifted_matrix, &eigen_vector);

        let norm_factor = norm(&temp_vector);
        eigen_vector = temp_vector.iter().map(|&x| x / norm_factor).collect();
    }

    let dot_product: f64 = eigen_vector
        .iter()
        .enumerate()
        .map(|(i, &val)| val * multiply_matrix_vector(&matrix, &eigen_vector)[i])
        .sum();

    dot_product / eigen_vector.iter().map(|x| x * x).sum::<f64>()
}