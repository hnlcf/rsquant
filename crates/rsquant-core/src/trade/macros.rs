#[macro_export]
macro_rules! min {
    ($x:expr) => {
        $x
    };
    ($x:expr, $y:expr) => {
        if $x < $y {
            $x
        } else {
            $y
        }
    };
    ($x:expr, $($rest:expr),+) => {
        min!($x, min!($($rest),+))
    };
}
