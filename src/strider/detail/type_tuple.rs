// type_tuple.rs

pub trait RefType: Sized {
    type Component: ComponentType<Ref=Self>;
}

pub trait ComponentType: Sized {
    type Ref: RefType<Component=Self>;

    unsafe fn as_ref(&self) -> Self::Ref;

    unsafe fn next(&mut self);

    unsafe fn prev(&mut self);
}

pub trait RefTypeTuple: Sized {
    type Components: ComponentTypeTuple<Refs=Self>;
}

pub trait ComponentTypeTuple: Sized {
    type Refs: RefTypeTuple<Components=Self>;

    unsafe fn as_ref(&self) -> Self::Refs;

    unsafe fn next(&mut self);

    unsafe fn prev(&mut self);
}

macro_rules! impl_type_tuple {
    ( $($ts:ident),* : $($is:tt),* ) => {
       impl<$($ts: RefType),*> RefTypeTuple for ( $($ts,)* ) {
           type Components = ( $($ts::Component,)* );
       }

       impl<$($ts: ComponentType),*> ComponentTypeTuple for ( $($ts,)* ) {
           type Refs = ( $($ts::Ref,)* );

           unsafe fn as_ref(&self) -> Self::Refs {
               ( $(self.$is.as_ref(),)* )
           }

           unsafe fn next(&mut self) {
               $(self.$is.next();)*
           }

           unsafe fn prev(&mut self) {
               $(self.$is.prev();)*
           }
       }
    };
}

impl_type_tuple!(T0 : 0);
impl_type_tuple!(T0, T1 : 0, 1);
impl_type_tuple!(T0, T1, T2 : 0, 1, 2);
impl_type_tuple!(T0, T1, T2, T3 : 0, 1, 2, 3);
impl_type_tuple!(T0, T1, T2, T3, T4 : 0, 1, 2, 3, 4);
impl_type_tuple!(T0, T1, T2, T3, T4, T5 : 0, 1, 2, 3, 4, 5);
impl_type_tuple!(T0, T1, T2, T3, T4, T5, T6 : 0, 1, 2, 3, 4, 5, 6);
impl_type_tuple!(T0, T1, T2, T3, T4, T5, T6, T7 : 0, 1, 2, 3, 4, 5, 7);
