use l2;
use l2::tensor::*;

fn main() -> Result<(), l2::errors::TensorError> {
    // let t = Tensor {
    //     data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
    //     shape: vec![2, 2, 2],
    //     strides: vec![4, 2, 1],
    // };

    // let x = t.slice(&[[0, 2], [0, 2], [0, 1]]);

    let a = Tensor::new(vec![2.0, 3.0, 4.0, 5.0], &[2, 2]).unwrap();
    // let b = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

    // let c = &a * &b;
    // let c = l2::sqrt(&a);

    let c = a.view(&[6])?;

    println!("{:?}", c);

    Ok(())
}
