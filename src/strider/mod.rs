// strider/mod.rs
pub mod detail;
use self::detail::ComponentTypeTuple;

#[derive(Copy, Clone)]
pub struct Strider<'a, Types: detail::StriderTypeTuple<'a>> {
    len: usize,
    components: Types::Components,
}

pub fn new_strider<'a, Components: ComponentTypeTuple<'a>>(
    len: usize,
    comps: Components,
) -> Strider<'a, Components::Types> { 
    Strider {
        len: len,
        components: comps,
    }
}

#[macro_export]
macro_rules! strider {
    ( $src:expr, $i1: ident $(,$is:ident)* ) => {{
        let base = $crate::strider::detail::StriderSource::base(&$src);
        unsafe {
            $crate::strider::new_strider(
                $crate::strider::detail::StriderSource::len(&$src),
                ( $crate::strider::detail::StriderSource::component(&$src, &(*base).$i1), $($crate::strider::detail::StriderSource::component(&$x, &(*base).$is)),* ),
            )
        }
    }};
}

pub struct Iter<'a, Types: detail::StriderTypeTuple<'a>> {
    strider: Strider<'a, Types>,
}

impl<'a, Types: detail::StriderTypeTuple<'a>> Iterator for Iter<'a, Types> {
    type Item = Types::References;

    fn next(&mut self) -> Option<Self::Item> {

    }
}
