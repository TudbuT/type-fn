use std::marker::PhantomData;

use type_fn::*;

pub struct Zero;
pub struct Succ<T>(PhantomData<T>);
pub trait ToNum<NumType> {
    const RESULT: NumType;
}
impl ToNum<usize> for Zero {
    const RESULT: usize = 0;
}
impl<T> ToNum<usize> for Succ<T>
where
    T: ToNum<usize>,
{
    const RESULT: usize = T::RESULT + 1;
}
type_fn! {
    pub fn Add<Lhs, Rhs>;
    pub fn Sub<Lhs, Rhs>;
    pub fn Mul<Lhs, Rhs>;
    pub fn Pow<N, Exponent>;

    pub fn Root<V, N>;

    pub fn DistanceDirect<DistX, DistY>;
}
type_fn_impl! {
    fn<TypeFn> Add< => Zero, Rhs> => Rhs;
    fn<TypeFn> Add<N => Succ<N>, Rhs>
        where
            Add<N, Rhs>: + TypeFn,
        => Succ<call!(Add<N, Rhs>)>;

    fn<TypeFn> Sub<Lhs, => Zero> => Lhs;
    fn<TypeFn> Sub<Lhs => Succ<Lhs>, Rhs => Succ<Rhs>>
        where
            Sub<Lhs, Rhs> : + TypeFn,
        => call!(Sub<Lhs, Rhs>);

    fn<TypeFn> Mul< => Zero, Rhs> => Zero;
    fn<TypeFn> Mul<Lhs => Succ<Lhs>, Rhs>
        where
            Mul<Lhs, Rhs>: + TypeFn,
            Add<Rhs, call!(Mul<Lhs, Rhs>)>: + TypeFn,
        => call!(Add<Rhs, call!(Mul<Lhs, Rhs>)>);

    fn<TypeFn> Pow<N, => Zero> => Succ<Zero>;
    fn<TypeFn> Pow<N, Exp => Succ<Exp>>
        where
            Pow<N, Exp>: + TypeFn,
            Mul<N, call!(Pow<N, Exp>)>: + TypeFn,
        => call!(Mul<N, call!(Pow<N, Exp>)>);
}

fn main() {
    type One = Succ<Zero>;
    type Two = call!(Add<One, One>);
    type Five = call!(Add<One, call!(Add<Two, Two>)>);
    println!("2^5: {}", <call!(Pow<Two, Five>) as ToNum<usize>>::RESULT);
}
