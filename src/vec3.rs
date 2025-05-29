use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e: [f64; 3], // Internal storage for x, y, z
}

impl Vec3 {
    // Create a new Vec3 with specific values
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    // Create a zero vector [0, 0, 0]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    // Accessors for x, y, z components
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    // Compute the vector length (magnitude)
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    // Compute the squared length
    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    // Dot product (u x v)
    pub fn dot(u: &Self, v: &Self) -> f64 {
        u.e[0]*v.e[0] + u.e[1]*v.e[1] + u.e[2]*v.e[2]
    }

    // Cross product (u x v)
    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            e: [
                u.e[1]*v.e[2] - u.e[2]*v.e[1],
                u.e[2]*v.e[0] - u.e[0]*v.e[2],
                u.e[0]*v.e[1] - u.e[1]*v.e[0],
            ]
        }
    }

    // Normalize the vector to unit length
    pub fn unit_vector(v: &Self) -> Self {
        *v / v.length()
    }
}

// Public alias for vec3
pub type Point3 = Vec3;

// Operator Overloads
// -v (subtraction)
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

// v[i] (indexing, read)
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

// v[i] = x (indexing, write)
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

// v + u
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.e[0]+other.e[0], self.e[1]+other.e[1], self.e[2]+other.e[2])
    }
}

// v - u
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.e[0]-other.e[0], self.e[1]-other.e[1], self.e[2]-other.e[2])
    }
}

// v * scalar
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self::new(self.e[0]*t, self.e[1]*t, self.e[2]*t)
    }
}

// scalar * v
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// v * v (component-wise)
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.e[0]*other.e[0], self.e[1]*other.e[1], self.e[2]*other.e[2])
    }
}

// v / scalar
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

// v += u
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

// v *= scalar
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

// v /= scalar
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}
