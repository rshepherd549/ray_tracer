
fn is_approx_eq(lhs: f64, rhs: f64) -> bool {
    const tol: f64 = 1e-6;
    return (lhs == rhs) || ((lhs-rhs).abs() < tol);
}

#[test]
fn test_is_approx_eq_f64() {
    assert!( is_approx_eq(1.0,1.0));
    assert!( is_approx_eq(1.0,1.0+1e-7));
    assert!(!is_approx_eq(1.0,1.0+1e-5));
}

struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: i8,
}

fn make_point(x:f64, y:f64, z:f64) -> Tuple {
    return Tuple {x,y,z,w:1};
}

fn make_vector(x:f64, y:f64, z:f64) -> Tuple {
    return Tuple {x,y,z,w:0};
}

fn is_approx_eq_tuple(lhs: Tuple, rhs: Tuple) -> bool {
    return is_approx_eq(lhs.x, rhs.x)
        && is_approx_eq(lhs.y, rhs.y)
        && is_approx_eq(lhs.z, rhs.z)
        && (lhs.w == rhs.w);
}

#[test]
fn test_is_approx_eq_tuple() {
    assert!( is_approx_eq_tuple(make_point(1.1,2.2,3.3),make_point(1.1,2.2,3.3)));    
    assert!(!is_approx_eq_tuple(make_point(1.1,2.2,3.3),make_point(2.1,2.2,3.3)));    
}

fn main() {
    println!("Ray Tracer!");
}
