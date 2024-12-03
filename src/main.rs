use crate::jacobi_rotate::jacobi_rotate;
use crate::newton_method::newton_method;
use crate::power_method::power_method;

mod jacobi_rotate;
mod power_method;
mod newton_method;

fn main() {
    let mut a = vec![
        vec![3., 1., 1., 0.],
        vec![1., 3., 0., 2.],
        vec![1., 0., 4., 1.],
        vec![0., 2., 1., 4.],
    ];

    println!("Smallest own number of A: {}", power_method(&a, 100));
    println!("Matrix A own numbers: {:?}", jacobi_rotate(&mut a, 0.0001, 100));

    let newton_result = newton_method(0.0001, 5).unwrap();
    println!("Newton method solve: x: {}, y: {}", newton_result.0, newton_result.1);
}
