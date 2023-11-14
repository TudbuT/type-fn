use type_fn::*;

#[derive(Debug)]
pub struct True;
#[derive(Debug)]
pub struct False;

type_fn! {
    pub fn Or<A, B>;
    pub fn And<A, B>;
    pub fn Xor<A, B>;
    pub fn Not<A>;
}

type_fn_impl! {
    fn<TypeFn> Or< => True, B> => True;
    fn<TypeFn> Or< => False, B> => B;

    fn<TypeFn> And< => True, B> => B;
    fn<TypeFn> And< => False, B> => False;

    fn<TypeFn> Not< => True> => False;
    fn<TypeFn> Not< => False> => True;

    fn<TypeFn> Xor< => True, B>
        where Not<B>: + TypeFn,
        => <Not<B> as TypeFn>::Ret;
    fn<TypeFn> Xor< => False, B> => B;
}

fn main() {
    println!(
        "True AND False: {}",
        core::any::type_name::<<And<True, False> as TypeFn>::Ret>()
    );
    println!(
        "True OR False: {}",
        core::any::type_name::<<Or<True, False> as TypeFn>::Ret>()
    );
}
