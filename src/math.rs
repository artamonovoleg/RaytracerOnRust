use std::ops;

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
}

impl ops::Sub<Vector3> for Vector3 
{
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3
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

    fn sub(self, rhs: f64) -> Vector3
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

impl ops::Add<Vector3> for Vector3 
{
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3
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

    fn add(self, rhs: f64) -> Vector3
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

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ {} {} {} ]", self.x(), self.y(), self.z())
    }
}