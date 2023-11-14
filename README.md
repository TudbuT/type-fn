# type-fn

type-fn allows you to more simply create logic at the type level.

## Example

Unsigned addition, subtraction, and multiplication:

```rs
struct Zero;
struct Succ<T>(PhantomData<T>);
type_fn! {
    fn Add<Lhs, Rhs>;
    fn Sub<Lhs, Rhs>;
    fn Mul<Lhs, Rhs>;
}
type_fn_impl! {
    fn<TypeFn> Add< => Zero, Rhs> => Rhs;
    fn<TypeFn> Add<N => Succ<N>, Rhs>
        where
            Add<N, Rhs>: + TypeFn,
        => Succ<<Add<N, Rhs> as TypeFn>::Ret>;

    fn<TypeFn> Sub<Lhs, => Zero> => Lhs;
    fn<TypeFn> Sub<Lhs => Succ<Lhs>, Rhs => Succ<Rhs>>
        where
            Sub<Lhs, Rhs> : + TypeFn,
        => <Sub<Lhs, Rhs> as TypeFn>::Ret;

    fn<TypeFn> Mul< => Zero, Rhs> => Zero;
    fn<TypeFn> Mul<Lhs => Succ<Lhs>, Rhs>
        where
            Mul<Lhs, Rhs>: + TypeFn,
            Add<Rhs, <Mul<Lhs, Rhs> as TypeFn>::Ret>: + TypeFn,
        => <Add<Rhs, <Mul<Lhs, Rhs> as TypeFn>::Ret> as TypeFn>::Ret;
}
```
