pub fn f64_eq(f1: f64, f2: f64) -> bool {
    const EPISLON : f64 = 0.0000001;
    (f1 - f2).abs() < EPISLON
}

