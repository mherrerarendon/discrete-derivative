use itertools::{Itertools, TupleWindows};
use std::ops::Sub;

pub struct DiscreteDerivativeIter<I> 
where 
    I: Iterator, 
    I::Item : Sub + Clone
{
    window_iter: TupleWindows<I, (I::Item, I::Item)>
}

impl<I> Iterator for DiscreteDerivativeIter<I> 
where 
    I: Iterator, 
    I::Item : Sub + Clone
{
    type Item = <I::Item as Sub>::Output;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((prev, next)) = self.window_iter.next() {
            Some(next - prev)
        } else {
            None
        }
    }
}

pub trait DiscreteDerivativeIterExt<I> 
where
    I: Iterator,
    I::Item: Sub + Clone
{
    fn discrete_derivative(self) -> DiscreteDerivativeIter<I>;
}

impl<I> DiscreteDerivativeIterExt<I> for I
where
    I: Iterator,
    I::Item: Sub + Clone
{
    fn discrete_derivative(self) -> DiscreteDerivativeIter<I> {
        DiscreteDerivativeIter {
            window_iter: self.tuple_windows()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::{ApproxEq};
    #[test]
    fn vec_works() {
        let mut deriv_iter = vec![1, 2, 1].into_iter().discrete_derivative();
        assert_eq!(deriv_iter.next(), Some(1));
        assert_eq!(deriv_iter.next(), Some(-1));
        assert_eq!(deriv_iter.next(), None);
    }

    #[test]
    fn slice_works() {
        let mut deriv_iter = [1, 2, 1].iter().discrete_derivative();
        assert_eq!(deriv_iter.next(), Some(1));
        assert_eq!(deriv_iter.next(), Some(-1));
        assert_eq!(deriv_iter.next(), None);
    }

    #[test]
    fn doubles_work() {
        let mut deriv_iter = [1.0, 2.1, 1.5].iter().discrete_derivative();
        let result: f64 = deriv_iter.next().expect("test");
        assert!(result.approx_eq(1.1, (0.0, 2)));
        let result: f64 = deriv_iter.next().expect("test");
        assert!(result.approx_eq(-0.6, (0.0, 2)));
        assert_eq!(deriv_iter.next(), None);
    }
}
