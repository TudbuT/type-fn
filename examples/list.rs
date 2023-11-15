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
    type MyListB = call!(AddElement<MyListA, World>);
    type MyListC = call!(AddElement<MyListB, Hello>);

    type RemovalA = call!(RemoveLastElement<MyListC>);
    println!("{}", any::type_name::<call!(GetRemoved<RemovalA>)>());

    type MyListD = call!(GetChangedList<RemovalA>);
    type RemovalB = call!(RemoveLastElement<MyListD>);
    println!("{}", any::type_name::<call!(GetRemoved<RemovalB>)>());
}
