pub const EPSILON: f64 = 0.00001;
#[macro_export]
macro_rules! assert_f64_eq {
    ($left:expr , $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if ((*left_val - *right_val) as f64).abs() > utils::EPSILON {
                    panic!("assert_f64_eq failed. (left: {}, right: {})", *left_val, *right_val);
                }
            }
        }
    })
}

pub fn f64_eq(f1: f64, f2: f64) -> bool {
    (f1 - f2).abs() < EPSILON

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
