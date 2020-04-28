
trait ApproxEq<Rhs=Self> {
    type Output;
    fn is_approx_eq(self, rhs:Rhs) -> bool;
}

impl ApproxEq for f64 {
    type Output = f64;
    fn is_approx_eq(self, rhs: f64) -> bool {
        const TOL: f64 = 1e-6;
        return (self == rhs) || ((self-rhs).abs() < TOL);
    }
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

impl ApproxEq for Tuple {
    type Output = f64;
    fn is_approx_eq(self, rhs: Tuple) -> bool {
        return self.x.is_approx_eq(rhs.x) && 
               self.y.is_approx_eq(rhs.y) && 
               self.z.is_approx_eq(rhs.z) && 
               self.w == rhs.w;
    }
}

fn is_approx_eq<T: ApproxEq>(lhs:T, rhs:T) -> bool
{
    return lhs.is_approx_eq(rhs);
}

#[test]
fn test_is_approx_eq() {
    assert!( 1.0.is_approx_eq(1.0));
    assert!( 1.0.is_approx_eq(1.0+1e-7));
    assert!(!1.0.is_approx_eq(1.0+1e-5));

    assert!( make_point(1.1,2.2,3.3).is_approx_eq(make_point(1.1,2.2,3.3)));    
    assert!(!make_point(1.1,2.2,3.3).is_approx_eq(make_point(2.1,2.2,3.3)));    

    assert!( make_vector(1.1,2.2,3.3).is_approx_eq(make_vector(1.1,2.2,3.3)));    
    assert!(!make_vector(1.1,2.2,3.3).is_approx_eq(make_vector(2.1,2.2,3.3)));    

    assert!(!make_point(1.1,2.2,3.3).is_approx_eq(make_vector(1.1,2.2,3.3)));    

    assert!( is_approx_eq(1.0, 1.0));
    assert!( is_approx_eq(1.0, 1.0+1e-7));
    assert!(!is_approx_eq(1.0, 1.0+1e-5));

    assert!( is_approx_eq(make_point(1.1,2.2,3.3), make_point(1.1,2.2,3.3)));    
    assert!(!is_approx_eq(make_point(1.1,2.2,3.3), make_point(2.1,2.2,3.3)));    

    assert!( is_approx_eq(make_vector(1.1,2.2,3.3), make_vector(1.1,2.2,3.3)));    
    assert!(!is_approx_eq(make_vector(1.1,2.2,3.3), make_vector(2.1,2.2,3.3)));    

    assert!(!is_approx_eq(make_point(1.1,2.2,3.3), make_vector(1.1,2.2,3.3)));    
}

fn main() {
    println!("Ray Tracer!");
}
