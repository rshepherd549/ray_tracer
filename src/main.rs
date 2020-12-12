use std::f64;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

trait ApproxEq<Rhs=Self> {
    type Output;
    fn is_approx_eq(&self, rhs:&Rhs) -> bool;
}

impl ApproxEq for f64 {
    type Output = f64;
    fn is_approx_eq(&self, rhs: &f64) -> bool {
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
    fn is_approx_eq(&self, other: &Tuple) -> bool {
        return self.x.is_approx_eq(&other.x) && 
               self.y.is_approx_eq(&other.y) && 
               self.z.is_approx_eq(&other.z) && 
               self.w == other.w;
    }
}

fn is_approx_eq<T: ApproxEq>(lhs:&T, rhs:&T) -> bool
{
    return lhs.is_approx_eq(rhs);
}

#[test]
fn test_is_approx_eq() {
    assert!( 1.0.is_approx_eq(&1.0));
    assert!( 1.0.is_approx_eq(&(1.0+1e-7)));
    assert!(!1.0.is_approx_eq(&(1.0+1e-5)));

    let p123 = make_point(1.1,2.2,3.3);
    assert!( p123.is_approx_eq(&make_point(1.1,2.2,3.3)));    
    assert!(!p123.is_approx_eq(&make_point(2.1,2.2,3.3)));    

    let v123 = make_vector(1.1,2.2,3.3);
    assert!( v123.is_approx_eq(&make_vector(1.1,2.2,3.3)));    
    assert!(!v123.is_approx_eq(&make_vector(2.1,2.2,3.3)));    

    assert!(!p123.is_approx_eq(&make_vector(1.1,2.2,3.3)));    

    assert!( is_approx_eq(&1.0, &1.0));
    assert!( is_approx_eq(&1.0, &(1.0+1e-7)));
    assert!(!is_approx_eq(&1.0, &(1.0+1e-5)));

    assert!( is_approx_eq(&p123, &make_point(1.1,2.2,3.3)));    
    assert!(!is_approx_eq(&p123, &make_point(2.1,2.2,3.3)));    

    assert!( is_approx_eq(&v123, &make_vector(1.1,2.2,3.3)));    
    assert!(!is_approx_eq(&v123, &make_vector(2.1,2.2,3.3)));    

    assert!(!is_approx_eq(&p123, &v123));    
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Tuple {x:-self.x, y:-self.y, z:-self.z, w:self.w}
    }
}

#[test]
fn test_neg_tuple() {
    assert!(is_approx_eq(&-make_point(1.2,-3.4,5.6), &make_point(-1.2,3.4,-5.6)));
}

impl Add for &Tuple {
    type Output = Tuple;
    fn add(self, other:&Tuple) -> Tuple {
        Tuple {x:self.x+other.x, y:self.y+other.y, z:self.z+other.z, w:self.w+other.w}
    }
}

impl<'a,'b> AddAssign<&'b Tuple> for Tuple {
    fn add_assign(&mut self, other:&'b Tuple) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

#[test]
fn test_add() {
    let p123 = make_point(1.0,2.0,3.0);
    assert!(is_approx_eq(&(&p123 + &make_vector(4.0,-5.0,6.0)), &make_point(5.0,-3.0,9.0)));
    assert!(is_approx_eq(&(&p123 + &make_vector(4.0,-5.5,6.1)), &make_point(5.0,-3.5,9.1)));

    let mut a = make_vector(-2.0,3.5,5.6);
    a += &make_vector(3.5,1.0,2.3);
    assert!(is_approx_eq(&a, &make_vector(1.5,4.5,7.9)));

    let v123 = make_vector(1.0, 2.0, 3.0);
    a += &v123;
    assert!(is_approx_eq(&a, &make_vector(2.5,6.5,10.9)));
    a += &v123;
    assert!(is_approx_eq(&a, &make_vector(3.5,8.5,13.9)));

}

impl Sub for &Tuple {
    type Output = Tuple;
    fn sub(self, other:&Tuple) -> Tuple {
        Tuple {x:self.x-other.x, y:self.y-other.y, z:self.z-other.z, w:self.w-other.w}
    }
}

impl SubAssign for Tuple {
    fn sub_assign(&mut self, other:Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

#[test]
fn test_sub() {
    assert!(is_approx_eq(&(&make_point(1.0,2.0,3.0) - &make_vector(4.0,-5.0,6.0)), &make_point(-3.0,7.0,-3.0)));

    let mut a = make_vector(-2.0,3.5,5.6);
    a -= make_vector(3.5,1.0,2.3);
    assert!(is_approx_eq(&a, &make_vector(-5.5,2.5,3.3)));
}

impl Mul<f64> for &Tuple {
    type Output = Tuple;
    fn mul(self, scale:f64) -> Tuple {
        Tuple {x:self.x*scale, y:self.y*scale, z:self.z*scale, w:self.w}
    }
}

impl MulAssign<f64> for Tuple {
    fn mul_assign(&mut self, scale:f64) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
}

#[test]
fn test_mul() {
    assert!(is_approx_eq(&(&make_point(1.0,2.0,3.0) * 2.0), &make_point(2.0,4.0,6.0)));

    let mut a = make_vector(-2.0,3.5,5.6);
    a *= -0.5;
    assert!(is_approx_eq(&a, &make_vector(1.0,-1.75,-2.8)));
}

impl Div<f64> for &Tuple {
    type Output = Tuple;
    fn div(self, scale:f64) -> Tuple {
        Tuple {x:self.x/scale, y:self.y/scale, z:self.z/scale, w:self.w}
    }
}

impl DivAssign<f64> for Tuple {
    fn div_assign(&mut self, scale:f64) {
        self.x /= scale;
        self.y /= scale;
        self.z /= scale;
    }
}

#[test]
fn test_div() {
    assert!(is_approx_eq(&(&make_point(1.0,2.0,3.0) / 2.0), &make_point(0.5,1.0,1.5)));

    let mut a = make_vector(-2.0,3.5,5.6);
    a /= -0.5;
    assert!(is_approx_eq(&a, &make_vector(4.0,-7.0,-11.2)));
}

fn sqr(x: f64) -> f64 {
    return x * x;
}

fn abs_sqr(tuple: &Tuple) -> f64 {
    return sqr(tuple.x) + sqr(tuple.y) + sqr(tuple.z); 
}

fn abs(tuple: &Tuple) -> f64 {
    return abs_sqr(tuple).sqrt(); 
}

fn magnitude(tuple: &Tuple) -> f64 {
    return abs(tuple); 
}

#[test]
fn test_abs_tuple()
{
    assert!(is_approx_eq(&abs(&make_vector(3.0,4.0,0.0)), &5.0));
    assert!(is_approx_eq(&abs(&make_point(0.0,3.0,-4.0)), &5.0));
    assert!(is_approx_eq(&magnitude(&make_point(0.0,3.0,-4.0)), &5.0));
}

fn normalized(tuple: &Tuple) -> Tuple {
    let mag = magnitude(&tuple);
    if mag == 0.0 {
        return Tuple{x:0.0, y:0.0, z:0.0, w:tuple.w};
    }

    return Tuple{x:tuple.x/mag, y:tuple.y/mag, z:tuple.z/mag, w:tuple.w};
}

#[test]
fn test_normalized() {
    assert!(is_approx_eq(&normalized(&make_vector(1.0,0.0,0.0)), &make_vector(1.0,0.0,0.0)));
    assert!(is_approx_eq(&normalized(&make_vector(2.0,0.0,0.0)), &make_vector(1.0,0.0,0.0)));
    assert!(is_approx_eq(&normalized(&make_vector(0.0,2.0,0.0)), &make_vector(0.0,1.0,0.0)));
    assert!(is_approx_eq(&normalized(&make_vector(3.0,4.0,0.0)), &make_vector(0.6,0.8,0.0)));
    assert!(is_approx_eq(&normalized(&make_vector(0.0,3.0,4.0)), &make_vector(0.0,0.6,0.8)));
}

struct Color {
    r: f64,
    g: f64,
    b: f64,
}

fn make_color(r:f64, g:f64, b:f64) -> Color {
    return Color {r,g,b};
}

impl ApproxEq for Color {
    type Output = f64;
    fn is_approx_eq(&self, other: &Color) -> bool {
        return self.r.is_approx_eq(&other.r) && 
               self.g.is_approx_eq(&other.g) && 
               self.b.is_approx_eq(&other.b);
    }
}

impl Add for &Color {
    type Output = Color;
    fn add(self, other:&Color) -> Color {
        make_color(self.r+other.r, self.g+other.g, self.b+other.b)
    }
}

impl AddAssign<&Color> for Color {
    fn add_assign(&mut self, other:&Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

#[test]
fn test_add_color() {
    let mut c = make_color(0.1,0.2,0.3);
    assert!(is_approx_eq(&(&c + &make_color(0.2,0.3,0.1)), &make_color(0.3,0.5,0.4)));
    assert!(is_approx_eq(&(&make_color(0.9, 0.6, 0.75) + &make_color(0.7, 0.1, 0.25)), &make_color(1.6, 0.7, 1.0)));

    c += &make_color(0.2, 0.3, 0.2);
    assert!(is_approx_eq(&c, &make_color(0.3,0.5,0.5)));
}

impl Sub for &Color {
    type Output = Color;
    fn sub(self, other:&Color) -> Color {
        make_color(self.r-other.r, self.g-other.g, self.b-other.b)
    }
}

impl SubAssign<&Color> for Color {
    fn sub_assign(&mut self, other:&Self) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

#[test]
fn test_sub_color() {
    assert!(is_approx_eq(&(&make_color(0.1,0.2,0.3) - &make_color(0.4,0.5,0.6)), &make_color(-0.3,-0.3,-0.3)));

    let mut a = make_color(0.2,0.3,0.6);
    a -= &make_color(0.3,0.4,0.1);
    assert!(is_approx_eq(&a, &make_color(-0.1,-0.1,0.5)));

    assert!(is_approx_eq(&(&make_color(0.9, 0.6, 0.75) - &make_color(0.7, 0.1, 0.25)), &make_color(0.2, 0.5, 0.5)));
}

impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, scale:f64) -> Color {
        make_color(self.r*scale, self.g*scale, self.b*scale)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, scale:f64) {
        self.r *= scale;
        self.g *= scale;
        self.b *= scale;
    }
}

// Hadamard or Schur product
// Produces the result of, for example, illuminating one color with another color light
impl Mul<&Color> for &Color {
    type Output = Color;
    fn mul(self, other:&Color) -> Color {
        make_color(self.r*other.r, self.g*other.g, self.b*other.b)
    }
}

impl MulAssign<&Color> for Color {
    fn mul_assign(&mut self, other:&Color) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

#[test]
fn test_mul_color() {
    assert!(is_approx_eq(&(&make_color(0.1,0.2,0.3) * 2.0), &make_color(0.2,0.4,0.6)));

    let mut a = make_color(0.8,0.35,0.56);
    a *= 0.5;
    assert!(is_approx_eq(&a, &make_color(0.4,0.175,0.28)));

    assert!(is_approx_eq(&(&make_color(0.2, 0.3, 0.4) * 2.0), &make_color(0.4, 0.6, 0.8)));

    assert!(is_approx_eq(&(&make_color(1.0, 0.2, 0.4) * &make_color(0.9, 1.0, 0.1)), &make_color(0.9, 0.2, 0.04)));
}



fn main() {
    println!("Ray Tracer!");
}
