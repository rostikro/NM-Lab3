use std::f64::EPSILON;

pub fn newton_method(tol: f64, max_iter: usize) -> Option<(f64, f64)> {
    let mut x: f64 = 0.5;
    let mut y: f64 = 0.5;

    for i in 0..max_iter {
        let f1 = x - (-y).exp();
        let f2 = y - x.exp();

        if f1.abs() < tol && f2.abs() < tol {
            return Some((x, y));
        }

        let j11 = 1.0;
        let j12 = (-y).exp();
        let j21 = -x.exp();
        let j22 = 1.0;

        let det = j11 * j22 - j12 * j21;
        if det.abs() < EPSILON {
            return None;
        }

        let inv_j11 = j22 / det;
        let inv_j12 = -j12 / det;
        let inv_j21 = -j21 / det;
        let inv_j22 = j11 / det;

        let delta_x = -(inv_j11 * f1 + inv_j12 * f2);
        let delta_y = -(inv_j21 * f1 + inv_j22 * f2);

        x += delta_x;
        y += delta_y;

        println!("Iteration {}: x: {x}, y: {y}", i+1);

        if delta_x.abs() < tol && delta_y.abs() < tol {
            return Some((x, y));
        }
    }

    return Some((x, y));
}