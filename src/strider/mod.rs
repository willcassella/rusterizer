// strider/mod.rs
pub mod detail;
use self::detail::{RefTypeTuple, ComponentTypeTuple};

#[derive(Copy, Clone)]
pub struct Strider<Types: RefTypeTuple> {
    len: usize,
    components: Types::Components,
}

pub fn new_strider<Components: ComponentTypeTuple>(
    len: usize,
    comps: Components,
) -> Strider<Components::Refs> {
    Strider {
        len: len,
        components: comps,
    }
}

#[macro_export]
macro_rules! strider {
    ( $src:expr, $($is:ident),* ) => {{
        let len = $crate::strider::detail::StriderSource::len(&$src);
        let base = $crate::strider::detail::StriderSource::base(&$src);
        unsafe {
            $crate::strider::new_strider(
                len,
                ( $($crate::strider::detail::StriderSource::component(&$x, &(*base).$is)),* ),
            )
        }
    }};
}

pub struct Iter<Types: RefTypeTuple> {
    strider: Strider<Types>,
    idx: usize,
}

impl<Types: RefTypeTuple> Iterator for Iter<Types> {
    type Item = Types;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.strider.len {
            unsafe {
                let result = self.strider.components.as_ref();
                self.strider.components.next();
                Some(result)
            }
        } else {
            None
        }
    }
}
