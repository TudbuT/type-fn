use std::{any, marker::PhantomData};

use type_fn::*;

pub struct EmptyList;
pub struct ListExt<T, Next>(PhantomData<T>, PhantomData<Next>);

pub struct RemovalResult<Element, List>(PhantomData<Element>, PhantomData<List>);

type_fn! {
    pub fn AddElement<List, Element>;
    pub fn RemoveLastElement<List>;

    pub fn GetRemoved<R>;
    pub fn GetChangedList<R>;
}

type_fn_impl! {
    fn<TypeFn> AddElement<List, Element> => ListExt<Element, List>;
    fn<TypeFn> RemoveLastElement<List => ListExt<Element, List> | Element> => RemovalResult<Element, List>;

    fn<TypeFn> GetRemoved<Element => RemovalResult<Element, List> | List> => Element;
    fn<TypeFn> GetChangedList<List => RemovalResult<Element, List> | Element> => List;
}

fn main() {
    struct Hello;
    struct World;

    type MyListA = EmptyList;
    type MyListB = <AddElement<MyListA, World> as TypeFn>::Ret;
    type MyListC = <AddElement<MyListB, Hello> as TypeFn>::Ret;

    type RemovalA = <RemoveLastElement<MyListC> as TypeFn>::Ret;
    println!(
        "{}",
        any::type_name::<<GetRemoved<RemovalA> as TypeFn>::Ret>()
    );

    type MyListD = <GetChangedList<RemovalA> as TypeFn>::Ret;
    type RemovalB = <RemoveLastElement<MyListD> as TypeFn>::Ret;
    println!(
        "{}",
        any::type_name::<<GetRemoved<RemovalB> as TypeFn>::Ret>()
    );
}
