// type_tuple.rs

use super::StriderComponent;

pub trait StriderTypeTuple<'a>: 'a + Sized {
    type Components: ComponentTypeTuple<'a, Types=Self>;
    type References;
}

pub trait ComponentTypeTuple<'a>: 'a + Sized {
    type Types: StriderTypeTuple<'a, Components=Self>;
    
    fn next(&mut self);
}

macro_rules! impl_type_tuple {
    ( $t1:ident $(,$ts:ident)* : $($is:tt),*) => {
       impl<'a, $t1: 'a, $($ts: 'a),*> StriderTypeTuple<'a> for ( $t1, $($ts),* ) {
           type Components = ( StriderComponent<'a, $t1>, $(StriderComponent<'a, $ts>),* );
           type References = ( &'a $t1, $(&'a $ts),* );
       }

       impl<'a, $t1: 'a, $($ts: 'a),*> ComponentTypeTuple<'a> for ( StriderComponent<'a, $t1>, $(StriderComponent<'a, $ts>),* ) {
           type Types = ( $t1, $($ts),* );

           fn next(&mut self) {
               $(self.$is.next());*
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
