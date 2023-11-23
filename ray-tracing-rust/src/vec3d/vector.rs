use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Field<F>
where
    F: Copy
        + Add<F, Output = F>
        + Mul<F, Output = F>
        + Neg<Output = F>
        + Sub<F, Output = F>
        + Div<F, Output = F>,
{
    fn equals(&self, lhs: F) -> bool;
    fn one() -> F;
    fn zero() -> F;
}

pub trait Vector<const D: usize, F>:
    PartialEq + Add<Self, Output = Self> + Mul<Self, Output = Self> + Sized
where
    F: Debug
        + Copy
        + Clone
        + PartialEq
        + Field<F>
        + Add<F, Output = F>
        + Mul<F, Output = F>
        + Neg<Output = F>
        + Sub<F, Output = F>
        + Div<F, Output = F>
        + std::iter::Sum,
{
    const DIM: usize = D;
    fn component(&self, index: usize) -> &F;
    fn new(components: [F; D]) -> Self
    where
        Self: Sized;

    fn zero() -> Self
    where
        Self: Sized + FromIterator<F>,
    {
        vec![F::zero(); Self::DIM].into_iter().collect()
    }

    fn unit(i: usize) -> Self
    where
        Self: Sized + FromIterator<F>,
    {
        let mut components = vec![F::zero(); Self::DIM];
        components[i] = F::one();
        components.into_iter().collect()
    }

    fn scale(&self, scalar: F) -> Self
    where
        Self: Sized + FromIterator<F>,
    {
        self.component_wise_unary_operation(|x| *x * scalar)
    }

    fn dot(&self, rhs: &Self) -> F
    where
        Self: Sized,
    {
        (0..Self::DIM)
            .map(|i| (*self.component(i) * *rhs.component(i)))
            .sum()
    }

    fn norm_squared(&self) -> F
    where
        Self: Sized + FromIterator<F>,
    {
        self.dot(self)
    }

    fn norm(&self) -> F
    where
        Self: Sized + FromIterator<F>;

    fn from_iter<T: IntoIterator<Item = F>>(iter: T) -> Self
    where
        Self: Sized,
    {
        let c = iter
            .into_iter()
            .take(Self::DIM)
            .collect::<Vec<F>>()
            .try_into()
            .unwrap();
        Self::new(c)
    }

    fn component_wise_binary_operation<OP>(&self, rhs: &Self, op: OP) -> Self
    where
        Self: Sized + FromIterator<F>,
        OP: Fn(&F, &F) -> F,
    {
        (0..Self::DIM)
            .map(|i| op(self.component(i), rhs.component(i)))
            .collect()
    }

    fn component_wise_unary_operation<OP>(&self, op: OP) -> Self
    where
        Self: Sized + FromIterator<F>,
        OP: Fn(&F) -> F,
    {
        (0..Self::DIM).map(|i| op(self.component(i))).collect()
    }
}

impl Field<f64> for f64 {
    fn equals(&self, lhs: f64) -> bool {
        self == &lhs
    }

    fn one() -> f64 {
        1.0
    }

    fn zero() -> f64 {
        0.0
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3D {
    components: [f64; 3],
}

impl Vec3D {
    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn cross(&self, other: &Self) -> Self {
        let u = self;
        let v = other;

        Self::new([
            u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x(),
        ])
    }
}

impl Vector<3, f64> for Vec3D {
    const DIM: usize = 3;
    fn component(&self, index: usize) -> &f64 {
        &self[index]
    }
    fn new(components: [f64; 3]) -> Self
    where
        Self: Sized,
    {
        Vec3D { components }
    }

    fn norm(&self) -> f64
    where
        Self: Sized + FromIterator<f64>,
    {
        self.norm_squared().sqrt()
    }
}

impl std::ops::Index<usize> for Vec3D {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl FromIterator<f64> for Vec3D {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        Vector::from_iter(iter)
    }
}

impl std::ops::Add for Vec3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::component_wise_binary_operation(&self, &rhs, |x, y| x + y)
    }
}

impl std::ops::Sub for Vec3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector::component_wise_binary_operation(&self, &rhs, |x, y| x - y)
    }
}

impl std::ops::Mul<Vec3D> for Vec3D {
    type Output = Self;
    fn mul(self, rhs: Vec3D) -> Self::Output {
        Vector::component_wise_binary_operation(&self, &rhs, |x, y| x * y)
    }
}

impl std::ops::Mul<f64> for Vec3D {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::scale(&self, rhs)
    }
}

impl std::ops::Div<f64> for Vec3D {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vector::scale(&self, 1.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for Vec3D {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop::array::uniform3(-1e10f64..1e10f64)
                .prop_map(|[x, y, z]| Vec3D::new([x, y, z]))
                .boxed()
        }
    }

    fn close_enough(lhs: f64, rhs: f64) -> bool {
        const EPSILON: f64 = 1e-10f64;
        (lhs - rhs).abs() < EPSILON
    }

    #[test]
    fn zero_vector_is_correct() {
        let zero = Vec3D::zero();
        for i in 0..Vec3D::DIM {
            assert_eq!(zero[i], 0.0);
        }
    }

    #[test]
    fn regression_1() {
        let v = Vec3D::new([0.0, 7712151695.054922f64, 0.0]);
        let f = 179279790.71764097f64;

        let result = v / f;

        assert!(close_enough(result[0], 0.0));
        assert!(close_enough(
            result[1],
            7712151695.054922f64 / f
        ));
        assert!(close_enough(result[2], 0.0));
    }

    proptest! {
        #[test]
        fn addition_is_component_wise(v1: Vec3D, v2: Vec3D) {
            let result = v1 + v2;
            for i in 0..Vec3D::DIM {
                prop_assert!(close_enough(result[i], v1[i] + v2[i]));
            }
        }

        #[test]
        fn subtraction_is_component_wise(v1: Vec3D, v2: Vec3D) {
            let result = v1 - v2;
            for i in 0..Vec3D::DIM {
                prop_assert!(close_enough(result[i], v1[i] - v2[i]));
            }
        }

        #[test]
        fn scale_is_component_wise(v1: Vec3D, f in 0.0f64..1e10f64) {
            let result = v1 * f;
            for i in 0..Vec3D::DIM {
                prop_assert!(close_enough(result[i], v1[i] * f));
            }
        }

        #[test]
        fn scale_div_is_component_wise(v1: Vec3D, f in f64::EPSILON..1e10f64) {
            let result = v1 / f;
            for i in 0..Vec3D::DIM {
                prop_assert!(close_enough(result[i], v1[i] / f));
            }
        }

        #[test]
        fn dot_product_of_orthogonal_vectors_is_zero(d in 0..Vec3D::DIM) {
            let v1 = Vec3D::unit(d);
            let v2 = Vec3D::unit((d + 1) % Vec3D::DIM);
            prop_assert!(close_enough(v1.dot(&v2), 0.0));
        }

        #[test]
        fn dot_product_of_unit_vector_with_itself_is_one(d in 0..Vec3D::DIM) {
            let v1 = Vec3D::unit(d);
            prop_assert!(close_enough(v1.dot(&v1), 1.0));
        }

        #[test]
        fn norm_of_unit_vector_is_one(d in 0..Vec3D::DIM) {
            let v1 = Vec3D::unit(d);
            prop_assert!(close_enough(v1.norm(), 1.0));
        }

        #[test]
        fn norm_squared_of_vector_of_length_two_is_four(d in 0..Vec3D::DIM) {
            let v1 = Vec3D::unit(d);
            prop_assert!(close_enough((v1 + v1).norm_squared(), 4.0));
        }

    }
}
