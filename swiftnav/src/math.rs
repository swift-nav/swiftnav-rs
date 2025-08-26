/// We define a `const` max function since [`std::cmp::max`] isn't `const`
pub(crate) const fn compile_time_max_u16(a: u16, b: u16) -> u16 {
    if b < a {
        a
    } else {
        b
    }
}
