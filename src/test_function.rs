use argmin::prelude::*;

/// First, create a struct for your problem
pub struct TestFunction {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

/// Implement `ArgminOp` for `Rosenbrock`
impl ArgminOp for TestFunction {
    /// Type of the parameter vector
    type Param = Vec<f64>;
    /// Type of the return value computed by the cost function
    type Output = f64;
    /// Type of the Hessian. Can be `()` if not needed.
    type Hessian = ();
    /// Type of the Jacobian. Can be `()` if not needed.
    type Jacobian = ();
    /// Floating point precision
    type Float = f64;

    /// Apply the cost function to a parameter `p`
    fn apply(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        if p.len() == 3 {
            Ok (self.a * p[0].powi(2) + self.b * p[1].powi(2) + self.c * p[2].powi(2))
        }
        else {
            Err(Error::msg("Wrong parameter"))
        }
    }

    /// Compute the gradient at parameter `p`.
    fn gradient(&self, p: &Self::Param) -> Result<Self::Param, Error> {
        if p.len() == 3 {
            Ok (vec![2.0 * self.a * p[0],  2.0 * self.b * p[1], 2.0 * self.c * p[2]])
        }
        else {
            Err(Error::msg("Wrong parameter"))
        }
    }
}