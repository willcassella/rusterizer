// strider_component.rs
use std::mem::transmute;
use std::marker::PhantomData;
use super::{RefType, ComponentType};

#[derive(Copy, Clone)]
pub struct StriderComponent<'a, T: 'a> {
    addr: *const T,
    stride: isize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> StriderComponent<'a, T> {
    pub fn new(
        addr: *const T,
        stride: isize,
    ) -> Self {
        StriderComponent {
            addr: addr,
            stride: stride,
            _phantom: PhantomData,
        }
    }

    pub unsafe fn as_ref(&self) -> &'a T {
        &*self.addr
    }

    pub unsafe fn next(&mut self) {
        let mut addr: *const u8 = transmute(self.addr);
        addr = addr.offset(self.stride);
        self.addr = transmute(addr);
    }

    pub unsafe fn prev(&mut self) {
        let mut addr: *const u8 = transmute(self.addr);
        addr = addr.offset(-self.stride);
        self.addr = transmute(addr);
    }
}

impl<'a, T> RefType for &'a T {
    type Component = StriderComponent<'a, T>;
}

impl<'a, T> ComponentType for StriderComponent<'a, T> {
    type Ref = &'a T;

    unsafe fn as_ref(&self) -> Self::Ref {
        self.as_ref()
    }

    unsafe fn next(&mut self) {
        self.next()
    }

    unsafe fn prev(&mut self) {
        self.prev()
    }
}

#[derive(Copy, Clone)]
pub struct StriderComponentMut<'a, T: 'a> {
    addr: *mut T,
    stride: isize,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T> StriderComponentMut<'a, T> {
    pub fn new(
        addr: *mut T,
        stride: isize,
    ) -> Self {
        StriderComponentMut {
            addr: addr,
            stride: stride,
            _phantom: PhantomData,
        }
    }

    pub unsafe fn as_ref(&self) -> &'a mut T {
        &mut *self.addr
    }

    pub unsafe fn next(&mut self) {
        let mut addr: *const u8 = transmute(self.addr);
        addr = addr.offset(self.stride);
        self.addr = transmute(addr);
    }

    pub unsafe fn prev(&mut self) {
        let mut addr: *const u8 = transmute(self.addr);
        addr = addr.offset(-self.stride);
        self.addr = transmute(addr);
    }
}

impl<'a, T> RefType for &'a mut T {
    type Component = StriderComponentMut<'a, T>;
}

impl<'a, T> ComponentType for StriderComponentMut<'a, T> {
    type Ref = &'a mut T;

    unsafe fn as_ref(&self) -> Self::Ref {
        self.as_ref()
    }

    unsafe fn next(&mut self) {
        self.next()
    }

    unsafe fn prev(&mut self) {
        self.prev()
    }
}
