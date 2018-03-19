// strider_component.rs
use std::mem::transmute;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct StriderComponent<'a, T: 'a> {
    addr: *const T,
    stride: isize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> StriderComponent<'a, T> {
    pub fn new(
        addr: *const T,
        stride: isize,
    ) -> Self {
        StriderComponent {
            addr: addr,
            stride: stride,
            phantom: PhantomData,
        }
    }

    pub unsafe fn next(
        &mut self,
    ) {
        let mut addr: *const u8 = transmute(self.addr);
        addr = addr.offset(self.stride);
        self.addr = transmute(addr);
    }

    pub unsafe fn prev(
        &mut self,
    ) {
        let mut addr: *const u8 = transmute(self.addr);
        addr = addr.offset(-self.stride);
        self.addr = transmute(addr);
    }
}
