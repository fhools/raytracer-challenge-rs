
#[macro_export]
macro_rules! assert_f64_eq {
    ($left:expr , $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if ((*left_val - *right_val) as f64).abs() > 0.000000001 {
                    panic!("assert_f64_eq failed. (left: {}, right: {})", *left_val, *right_val);
                }
            }
        }
    })
}

pub fn f64_eq(f1: f64, f2: f64) -> bool {
    const EPISLON : f64 = 0.0000001;
    (f1 - f2).abs() < EPISLON
}


#[macro_export]
macro_rules! assert_vector4d_eq {
    ($left:expr , $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !left_val.eq(right_val) {
                    panic!("left != right. (left: {:?}, right: {:?}", *left_val, *right_val);
                }
            }
        }
    })
}

#[macro_export]
macro_rules! assert_color_eq {
    ($left:expr , $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !left_val.eq(right_val) {
                    panic!("left != right. (left: {:?}, right: {:?}", *left_val, *right_val);
                }
            }
        }
    })
}
