pub fn compose_chain<F, G, R>(f: F, g: G) -> impl FnOnce(R) -> R
where
    F: FnOnce(R) -> R,
    G: FnOnce(R) -> R,
{
    move |x: R| f(g(x))
}

/**
 * Composes functions from right to left.
 */
#[macro_export]
macro_rules! compose {
    ($f:expr) => {
        $f
    };
    ($f:expr, $($g:expr),+) => {
        $crate::compose_chain($f, compose!($($g),+))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compose() {
        let f = |x: i32| x + 2;
        let g = |x: i32| x * 10;
        let h = compose!(f, g);

        assert_eq!(h(2), 22);
    }
}
