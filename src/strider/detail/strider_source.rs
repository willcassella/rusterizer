// strider_source.rs
use std::mem::size_of;
use super::StriderComponent;

pub trait StriderSource {
    type Base;

    /**
     * Returns the number of things you can stride over.
     * For arrays this is the length of the array, for striders this is the len.
     */
    fn len(
        &self
    ) -> usize;

    /**
     * Returns the address of the 'base' object to get components from
     * For arrays this is the address of the first(?) element
     * For Striders this is the address of the component tuple.
     */
    fn base(
        &self
    ) -> *const Self::Base;

    fn component<'a, T>(
        &'a self,
        addr: *const T
    ) -> StriderComponent<'a, T>;
}

impl<T> StriderSource for [T] {
    type Base = T;

    fn len(
        &self,
    ) -> usize {
        <[T]>::len(self)
    }

    fn base(
        &self,
    ) -> *const Self::Base {
        self.as_ptr()
    }

    fn component<'a, M>(
        &'a self,
        addr: *const M,
    ) -> StriderComponent<'a, M> {
        StriderComponent::new(addr, size_of::<i32>() as isize)
    }
}

macro_rules! impl_StriderSource_for_array {
    ($x:expr) => {
       impl<T> StriderSource for [T; $x] {
           type Base = T;
           fn len(
               &self,
           ) -> usize {
               $x
           }

           fn base(
               &self,
           ) -> *const Self::Base {
               self.as_ptr()
           }

           fn component<'a, M>(
               &'a self,
               addr: *const M,
           ) -> StriderComponent<'a, M> {
               StriderComponent::new(addr, size_of::<T>() as isize)
           }
       } 
    };
}

impl_StriderSource_for_array!(1);
impl_StriderSource_for_array!(2);
impl_StriderSource_for_array!(3);
impl_StriderSource_for_array!(4);
impl_StriderSource_for_array!(5);
impl_StriderSource_for_array!(6);
impl_StriderSource_for_array!(7);
impl_StriderSource_for_array!(8);
impl_StriderSource_for_array!(9);
impl_StriderSource_for_array!(10);
impl_StriderSource_for_array!(11);
impl_StriderSource_for_array!(12);
impl_StriderSource_for_array!(13);
impl_StriderSource_for_array!(14);
impl_StriderSource_for_array!(15);
impl_StriderSource_for_array!(16);
impl_StriderSource_for_array!(17);
impl_StriderSource_for_array!(18);
impl_StriderSource_for_array!(20);
impl_StriderSource_for_array!(21);
impl_StriderSource_for_array!(22);
impl_StriderSource_for_array!(23);
impl_StriderSource_for_array!(24);
impl_StriderSource_for_array!(25);
impl_StriderSource_for_array!(26);
impl_StriderSource_for_array!(27);
impl_StriderSource_for_array!(28);
impl_StriderSource_for_array!(29);
impl_StriderSource_for_array!(30);
impl_StriderSource_for_array!(31);
impl_StriderSource_for_array!(32);
