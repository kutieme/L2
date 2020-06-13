pub mod errors;
mod ops;
pub mod tensor;

use errors::TensorError;
use tensor::Tensor;

#[cfg(test)]
mod tests {
    use super::tensor::*;
    use super::*;

    #[test]
    fn test_add() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();
        let b = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = add(&a, &b);

        assert!((c.data == vec![4.0, 6.0]) && (c.shape == vec![2]))
    }

    #[test]
    fn test_subtract() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();
        let b = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = sub(&a, &b);

        assert!((c.data == vec![0.0, 0.0]) && (c.shape == vec![2]))
    }
    #[test]
    fn test_mul() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();
        let b = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = mul(&a, &b);

        assert!((c.data == vec![4.0, 9.0]) && (c.shape == vec![2]))
    }

    #[test]
    fn test_div() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();
        let b = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = div(&a, &b);

        assert!((c.data == vec![1.0, 1.0]) && (c.shape == vec![2]))
    }

    #[test]
    fn test_pow() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = pow(&a, 2).unwrap();

        assert!((c.data == vec![4.0, 9.0]) && (c.shape == vec![2]))
    }

    #[test]
    fn test_sum() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = sum(&a, 0).unwrap();

        assert!((c.data == vec![5.0]) && (c.shape == vec![1]))
    }

    #[test]
    fn test_mean() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = mean(&a, 0).unwrap();

        assert!((c.data == vec![2.5]) && (c.shape == vec![1]))
    }
    #[test]
    fn test_max() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = max(&a, 0).unwrap();

        assert!((c.data == vec![3.0]) && (c.shape == vec![1]))
    }
    #[test]
    fn test_min() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = min(&a, 0).unwrap();

        assert!((c.data == vec![2.0]) && (c.shape == vec![1]))
    }

    #[test]
    fn test_argmax() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = argmax(&a, 0).unwrap();

        assert!((c.data == vec![1.0]) && (c.shape == vec![1]))
    }
    #[test]
    fn test_argmin() {
        let a = Tensor::new(vec![2.0, 3.0], &[2]).unwrap();

        let c = argmin(&a, 0).unwrap();

        assert!((c.data == vec![0.0]) && (c.shape == vec![1]))
    }

    #[test]
    fn test_matmul() {
        let x = Tensor::new(
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ],
            &[2, 2, 4],
        )
        .unwrap();
        let y = Tensor::new(
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ],
            &[2, 4, 2],
        )
        .unwrap();

        let z = matmul(&x, &y).unwrap();

        assert!(
            (z.data == vec![50.0, 60.0, 114.0, 140.0, 514.0, 556.0, 706.0, 764.0])
                && (z.shape == vec![2, 2, 2])
        )
    }

    #[test]
    fn test_concat() {
        let x = Tensor::new(
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ],
            &[2, 2, 2, 2],
        )
        .unwrap();
        let y = Tensor::new(
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
                16.0,
            ],
            &[2, 2, 2, 2],
        )
        .unwrap();

        let z = concat(&x, &y, -1).unwrap();

        assert!(
            (z.data
                == vec![
                    1.0, 2.0, 1.0, 2.0, 3.0, 4.0, 3.0, 4.0, 5.0, 6.0, 5.0, 6.0, 7.0, 8.0, 7.0, 8.0,
                    9.0, 10.0, 9.0, 10.0, 11.0, 12.0, 11.0, 12.0, 13.0, 14.0, 13.0, 14.0, 15.0,
                    16.0, 15.0, 16.0
                ])
                && (z.shape == vec![2, 2, 2, 4])
        )
    }
}

pub fn add<'a>(lhs: &'a Tensor, rhs: &'a Tensor) -> Tensor<'a> {
    lhs + rhs
}

pub fn sub<'a>(lhs: &'a Tensor, rhs: &'a Tensor) -> Tensor<'a> {
    lhs - rhs
}

pub fn mul<'a>(lhs: &'a Tensor, rhs: &'a Tensor) -> Tensor<'a> {
    lhs * rhs
}

pub fn div<'a>(lhs: &'a Tensor, rhs: &'a Tensor) -> Tensor<'a> {
    lhs / rhs
}

pub fn pow<'a>(lhs: &'a Tensor, exp: isize) -> Result<Tensor<'a>, TensorError> {
    lhs.pow(exp)
}

pub fn sqrt<'a>(lhs: &'a Tensor) -> Result<Tensor<'a>, TensorError> {
    lhs.sqrt()
}

pub fn exp<'a>(lhs: &'a Tensor) -> Result<Tensor<'a>, TensorError> {
    lhs.exp()
}

pub fn log10<'a>(lhs: &'a Tensor) -> Result<Tensor<'a>, TensorError> {
    lhs.log10()
}

pub fn log<'a>(lhs: &'a Tensor) -> Result<Tensor<'a>, TensorError> {
    lhs.log()
}

pub fn abs<'a>(lhs: &'a Tensor) -> Result<Tensor<'a>, TensorError> {
    lhs.abs()
}

pub fn sin<'a>(lhs: &'a Tensor) -> Result<Tensor<'a>, TensorError> {
    lhs.sin()
}

pub fn cos<'a>(lhs: &'a Tensor) -> Result<Tensor<'a>, TensorError> {
    lhs.cos()
}

pub fn tan<'a>(lhs: &'a Tensor) -> Result<Tensor<'a>, TensorError> {
    lhs.tan()
}

pub fn sum<'a>(lhs: &'a Tensor, dim: isize) -> Result<Tensor<'a>, TensorError> {
    lhs.sum(dim)
}

pub fn mean<'a>(lhs: &'a Tensor, dim: isize) -> Result<Tensor<'a>, TensorError> {
    lhs.mean(dim)
}

pub fn max<'a>(lhs: &'a Tensor, dim: isize) -> Result<Tensor<'a>, TensorError> {
    lhs.max(dim)
}

pub fn min<'a>(lhs: &'a Tensor, dim: isize) -> Result<Tensor<'a>, TensorError> {
    lhs.min(dim)
}

pub fn argmax<'a>(lhs: &'a Tensor, dim: isize) -> Result<Tensor<'a>, TensorError> {
    lhs.argmax(dim)
}

pub fn argmin<'a>(lhs: &'a Tensor, dim: isize) -> Result<Tensor<'a>, TensorError> {
    lhs.argmin(dim)
}

pub fn matmul<'a>(lhs: &'a Tensor, rhs: &'a Tensor) -> Result<Tensor<'a>, TensorError> {
    lhs.matmul(rhs)
}

pub fn concat<'a>(lhs: &'a Tensor, rhs: &'a Tensor, dim: isize) -> Result<Tensor<'a>, TensorError> {
    lhs.concat(&rhs, dim)
}
