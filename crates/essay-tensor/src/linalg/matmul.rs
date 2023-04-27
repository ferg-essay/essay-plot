use std::rc::Rc;

use crate::tensor::{Tensor, TensorData, Dtype};

enum Transpose {
    None,
    TransposeA,
    TransposeB,
}

pub fn matmul<const N:usize>(a: &Tensor<N>, b: &Tensor<N>) -> Tensor<N> {
    assert!(N > 1, "matrix multiplication requires dim >= 2");
    assert_eq!(&a.shape()[2..], &b.shape()[2..], "matmul batch shape must match");
    assert_eq!(a.shape()[0], b.shape()[1], "matmul a.shape[0] must equal b.shape[1]");

    let n : usize = a.shape()[2..].iter().product();
    let a_size = a.shape()[0] * a.shape()[1];
    let b_size = b.shape()[0] * b.shape()[1];
    let o_size = a.shape()[1] * b.shape()[0];

    unsafe {
        let mut out = TensorData::<f32>::new_uninit(o_size * n);

        let mut a_start = 0;
        let mut b_start = 0;
        let mut o_start = 0;

        let cols = b.shape()[0];
        let rows = a.shape()[1];
    
        for _ in 0..n {
            naive_matmul_f32(
                &mut out, 
                o_start,
                a,
                a_start,
                b,
                b_start,
                cols,
                rows,
            );

            a_start += a_size;
            b_start += b_size;
            o_start += o_size;
        }

        let mut o_shape = a.shape().clone();
        o_shape[0] = b.shape()[0];
        o_shape[1] = a.shape()[1];
    
        Tensor::new(Rc::new(out), o_shape)
    }
}

unsafe fn naive_matmul_f32<const N:usize>(
    out: &mut TensorData<f32>, 
    out_start: usize,
    a: &Tensor<N, f32>, 
    a_start: usize,
    b: &Tensor<N, f32>,
    b_start: usize,
    cols: usize,
    rows: usize,
) {
    let out_stride = cols;
    let a_stride = a.shape()[0];
    let b_stride = b.shape()[0];

    let a_ptr = a.buffer().ptr();
    let b_ptr = b.buffer().ptr();
    let out_ptr = out.ptr();

    let mut out_row = out_start;
    let mut a_row = a_start;

    for _ in 0..rows {
        for col in 0..cols {
            let mut b_off = b_start + col;
            let mut v: f32 = 0.;
            for k in 0..a_stride {
                v += a_ptr.add(a_row + k).read() * b_ptr.add(b_off).read();

                b_off += b_stride;
            }

            out_ptr.add(out_row + col).write(v);
        }

        out_row += out_stride;
        a_row += a_stride;
    }

}

#[cfg(test)]
mod test {
    use crate::{tensor, Tensor};

    use super::matmul;

    #[test]
    fn test_matmul_1() {
        let a = tensor!([[2.]]);
        let b = tensor!([[3.]]);

        assert_eq!(matmul(&a, &b), tensor!([[6.]]));
    }

    #[test]
    fn test_matmul_vectors() {
        let a = tensor!([[1., 2.]]);
        let b = tensor!([[1.], [3.]]);
        assert_eq!(matmul(&a, &b), tensor!([[7.]]));

        let a = tensor!([[1., 2.]]);
        let b = tensor!([[1.], [3.]]);
        assert_eq!(matmul(&b, &a), tensor!([[1., 2.], [3., 6.]]));
    }

    #[test]
    fn test_matmul_square() {

        let id = tensor!([[1., 0.], [0., 1.]]);
        assert_eq!(&matmul(&id, &id), &id);

        let a = tensor!([[1., 0.], [0., 2.]]);
        assert_eq!(&matmul(&id, &a), &a);
        assert_eq!(&matmul(&a, &id), &a);
        assert_eq!(matmul(&a, &a), tensor!([[1., 0.], [0., 4.]]));

        let top = tensor!([[1., 1.], [0., 1.]]);
        assert_eq!(matmul(&top, &top), tensor!([[1., 2.], [0., 1.]]));

        let bot = tensor!([[1., 0.], [1., 1.]]);
        assert_eq!(matmul(&bot, &bot), tensor!([[1., 0.], [2., 1.]]));
    }

    #[test]
    fn test_matmul_2x3() {

        let a = tensor!([[1., 0., 2.], [0., 1., 10.]]);
        let b = tensor!([[1., 0.], [0., 1.], [3., 4.]]);
        assert_eq!(matmul(&a, &b), tensor!([[7., 8.], [30., 41.]]));
        assert_eq!(matmul(&b, &a), tensor!([
            [1., 0., 2.],
            [0., 1., 10.],
            [3., 4., 46.]]));
    }
}