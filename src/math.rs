use std::{ops, f64};

#[derive(Copy, Clone)]
pub struct Vector3
{
    e: [f64; 3]
}

impl Vector3
{
    #[allow(dead_code)]
    pub fn origin() -> Self
    {
        Self { e: [0.0; 3] }
    }

    #[allow(dead_code)]
    pub fn new(x : f64, y : f64, z : f64) -> Self
    {
        Self { e: [x, y, z] }
    }

    #[allow(dead_code)]
    pub fn x(&self) -> f64
    {
        self.e[0]
    }

    #[allow(dead_code)]
    pub fn y(&self) -> f64
    {
        self.e[1]
    }

    #[allow(dead_code)]
    pub fn z(&self) -> f64
    {
        self.e[2]
    }

    #[allow(dead_code)]
    pub fn length(&self) -> f64
    {
        self.length_squared().sqrt()
    }

    #[allow(dead_code)]
    pub fn length_squared(&self) -> f64
    {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

/// Substract
impl ops::Sub<Vector3> for Vector3 
{
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output
    {
        Vector3::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2])
    }
}

impl ops::SubAssign<Vector3> for Vector3
{
    fn sub_assign(&mut self, rhs: Vector3) 
    {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl ops::Sub<f64> for Vector3
{
    type Output = Vector3;

    fn sub(self, rhs: f64) -> Self::Output
    {
        Vector3::new(self.e[0] - rhs, self.e[1] - rhs, self.e[2] - rhs)
    }
}

impl ops::SubAssign<f64> for Vector3
{
    fn sub_assign(&mut self, rhs: f64) 
    {
        self.e[0] -= rhs;
        self.e[1] -= rhs;
        self.e[2] -= rhs;
    }
}

/// Add
impl ops::Add<Vector3> for Vector3 
{
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output
    {
        Vector3::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
    }
}

impl ops::AddAssign<Vector3> for Vector3
{
    fn add_assign(&mut self, rhs: Vector3) 
    {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Add<f64> for Vector3
{
    type Output = Vector3;

    fn add(self, rhs: f64) -> Self::Output
    {
        Vector3::new(self.e[0] + rhs, self.e[1] + rhs, self.e[2] + rhs)
    }
}

impl ops::AddAssign<f64> for Vector3
{
    fn add_assign(&mut self, rhs: f64) 
    {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}

/// Multiply
impl ops::Mul<Vector3> for Vector3
{
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output 
    {
        Vector3::new(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
    }
}

impl ops::MulAssign<Vector3> for Vector3
{
    fn mul_assign(&mut self, rhs: Vector3) 
    {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];    
        self.e[2] *= rhs.e[2];    
    }
}

impl ops::Mul<f64> for Vector3 
{
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output 
    {
        Vector3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl ops::Mul<Vector3> for f64
{
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output 
    {
        Vector3::new(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2])
    }
}

impl ops::MulAssign<f64> for Vector3
{
    fn mul_assign(&mut self, rhs: f64)
    {
        self.e[0] *= rhs;
        self.e[1] *= rhs;    
        self.e[2] *= rhs;
    }
}

/// Divide
impl ops::Div<Vector3> for Vector3 
{
    type Output = Vector3;
    
    fn div(self, rhs: Vector3) -> Self::Output 
    {
        Vector3::new(self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2])    
    }
}

impl ops::DivAssign<Vector3> for Vector3 
{
    fn div_assign(&mut self, rhs: Vector3) 
    {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];    
        self.e[2] /= rhs.e[2];    
    }
}

impl ops::Div<f64> for Vector3
{
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output 
    {
        Vector3::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)    
    }
}

impl ops::DivAssign<f64> for Vector3
{
    fn div_assign(&mut self, rhs: f64)
    {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

#[allow(dead_code)]
pub fn dot(u: Vector3, v: Vector3) -> f64
{
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

#[allow(dead_code)]
pub fn cross(u: Vector3, v: Vector3) -> Vector3
{
    Vector3::new
    (
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2], 
        u.e[0] * v.e[1] - u.e[1] * v.e[0]
    )
}

#[allow(dead_code)]
pub fn unit_vector(v: Vector3) -> Vector3
{
    v / v.length()
}

/// IO
impl std::fmt::Display for Vector3 
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(f, "[ {} {} {} ]", self.e[0], self.e[1], self.e[2])
    }
}