#![no_std]

/// Default trait for returning something from a type-fn
pub trait TypeFn {
    type Ret;
}

/// Calls a type-fn
#[macro_export]
macro_rules! call {
    ($($fn:tt)+) => {
        <$($fn)+ as TypeFn>::Ret
    };
}

#[macro_export]
macro_rules! call_as {
    ($fty:ty => $($fn:tt)+) => {
        <$($fn)+ as $fty>::Ret
    };
}

/// Verifies equality between two types at compile-time.
#[macro_export]
macro_rules! assert_types_eq {
    ($a:ty, $b:ty) => {{
        let _: ::core::marker::PhantomData<$a> = ::core::marker::PhantomData::<$b>;
    }};
}

/// Generates type-fn implementations.
/// Syntax:
/// `fn<$FnType$> $name$<$args$> $[$where-clause$]$ => $return-type$;`
///
/// FnType should simply be a trait containing `type Ret`, e.g. [`TypeFn`].
///
/// args is a type-arg list. To pattern-match types, use `T => Successor<T>` to
/// implement something like like `Add<Successor<T>, Rhs>`.
/// When implementing something for a specific type, use ` => TypeHere` (leave out
/// the type arg name).
/// If you need an extra type arg that wont be in the resulting type anywhere,
/// add a bar `|` and write type args there as normal.
///
/// The where clause is just like rust's, except you need to put a plus before the first
/// trait bound as well. Trailing commas are also mandatory.
/// Trait bounds can only be set using the where-clause.
///
/// The return type can use any of the type arguments.
///
/// You can define an arbitrary amount of functions in one macro invocation.
#[macro_export]
macro_rules! type_fn_impl {
    {@a_or_else_b  => } => {compiler_error!()};
    {@a_or_else_b  => $($b:ident)+} => {$($b)*};
    {@a_or_else_b $($a:ty)+ => $($b:ident)*} => { $($a)+ };
    {$(fn < $sup:ty > $name:ident <$($($arg:ident)? $(=> $($argv:ty)+)?),* $(| $($targ:ident),+)?>
        $(where $($tv:ty : $( + $( ?$tcqm:ident )? $( $tc:ident )? )+ ,)+)? => $ret:ty;)+}
    => {
        $(
            impl<$($($arg, )?)* $($($targ, )*)?>
                $sup for $name <$($crate::type_fn_impl!(@a_or_else_b $($($argv)*)? => $($arg)?)),*>
            $(where $($tv : $($(?$tcqm)? $($tc)? + )+ ),+)?
            {
                type Ret = $ret;
            }
        )+
    };
}

/// Creates type functions. You will still need to implement them yourself, e.g. using
/// [`type_fn_impl!`].
/// Syntax:
/// `$[$visibility$]$ fn $name$ <$args$>;`
///
/// visibility is just like rust's normal visibility modifier.
///
/// args is a list of type arguments. They can not have constraints at this time.
#[macro_export]
macro_rules! type_fn {
    ($($vis:vis fn $name:ident <$($arg:ident),*>;)*) => {
        $(
            $vis struct $name <$($arg),*> ($(::core::marker::PhantomData<$arg>, )*);
        )*
    };
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    use crate::TypeFn;

    #[test]
    fn test_compile() {
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

        type TwoMinusOne = <Sub<Succ<Succ<Zero>>, Succ<Zero>> as TypeFn>::Ret;
        assert_types_eq!(TwoMinusOne, Succ<Zero>);
        assert_types_eq!(call!(Sub<TwoMinusOne, Succ<Zero>>), Zero);
        assert_types_eq!(
            Succ<Succ<Succ<Succ<Zero>>>>,
            <Mul<Succ<Succ<Zero>>, Succ<Succ<Zero>>> as TypeFn>::Ret
        );
        assert_types_eq!(Zero, <Mul<Succ<Succ<Zero>>, Zero> as TypeFn>::Ret);
    }
}
