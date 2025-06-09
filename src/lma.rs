// use levenberg_marquardt::Problem;
use levenberg_marquardt::LeastSquaresProblem;

use nalgebra::{DVector, Dyn, IsContiguous, Matrix3, Owned, VecStorage, Vector3, U3};

struct OneDeviceProblem {
    nu0: f64,
    x0: f64,
    v0: f64,
    // not changeable
    tau0: f64,
}

/*
impl LeastSquaresProblem<f64, U3, U3> for OneDeviceProblem {
    type ParameterStorage = Owned<f64, U3>;
    type ResidualStorage = Owned<f64, U3>;
    type JacobianStorage = Owned<f64, U3, U3>;

    fn set_params(&mut self, x: &nalgebra::Vector<f64, U3, Self::ParameterStorage>) {
        
    }

    fn params(&self) -> nalgebra::Vector<f64, U3, Self::ParameterStorage> {
        self.p
    }

    fn residuals(&self) -> Option<nalgebra::Vector<f64, U3, Self::ResidualStorage>> {
        let [x, y] = [self.p.x, self.p.y];
        // vector containing residuals $r_1(\vec{x})$ and $r_2(\vec{x})$
        Some(Vector3::new(x*x + y - 11., x + y*y - 7., 0.))
    }

    fn jacobian(&self) -> Option<nalgebra::Matrix<f64, U3, U3, Self::JacobianStorage>> {
        let [x, y] = [self.p.x, self.p.y];
         
        // first row of Jacobian, derivatives of first residual
        let d1_x = 2. * x; // $\frac{\partial}{\partial x_1}r_1(\vec{x}) = \frac{\partial}{\partial x} (x^2 + y - 11) = 2x$
        let d1_y = 1.;     // $\frac{\partial}{\partial x_2}r_1(\vec{x}) = \frac{\partial}{\partial y} (x^2 + y - 11) = 1$
         
        // second row of Jacobian, derivatives of second residual
        let d2_x = 1.;     // $\frac{\partial}{\partial x_1}r_2(\vec{x}) = \frac{\partial}{\partial x} (x + y^2 - 7) = 1$
        let d2_y = 2. * y; // $\frac{\partial}{\partial x_2}r_2(\vec{x}) = \frac{\partial}{\partial y} (x + y^2 - 7) = 2y$

        Some(Matrix3::new(
            d1_x, d1_y, 0.,
            d2_x, d2_y, 0.,
            0., 0., 0.,
        ))
    }
}
*/

use nalgebra::{Vector2, U2, VectorN, Matrix2};
use levenberg_marquardt::LevenbergMarquardt;

fn _example_problem_solve() -> () {
    struct ExampleProblem {
        // holds current value of the n parameters
        p: Vector2<f64>,
    }

    // We implement a trait for every problem we want to solve
    impl LeastSquaresProblem<f64, U2, U2> for ExampleProblem {
        type ParameterStorage = Owned<f64, U2>;
        type ResidualStorage = Owned<f64, U2>;
        type JacobianStorage = Owned<f64, U2, U2>;
        
        fn set_params(&mut self, p: &VectorN<f64, U2>) {
            self.p.copy_from(p);
            // do common calculations for residuals and the Jacobian here
        }
        
        fn params(&self) -> VectorN<f64, U2> { self.p }
        
        fn residuals(&self) -> Option<Vector2<f64>> {
            let [x, y] = [self.p.x, self.p.y];
            // vector containing residuals r1(x⃗)r_1(\vec{x})r1​(x) and r2(x⃗)r_2(\vec{x})r2​(x)
            Some(Vector2::new(x*x + y - 11., x + y*y - 7.))
        }
        
        fn jacobian(&self) -> Option<Matrix2<f64>> {
            let [x, y] = [self.p.x, self.p.y];
            
            // first row of Jacobian, derivatives of first residual
            let d1_x = 2. * x; // ∂∂x1r1(x⃗)=∂∂x(x2+y−11)=2x\frac{\partial}{\partial x_1}r_1(\vec{x}) = \frac{\partial}{\partial x} (x^2 + y - 11) = 2x∂x1​∂​r1​(x)=∂x∂​(x2+y−11)=2x
            let d1_y = 1.;     // ∂∂x2r1(x⃗)=∂∂y(x2+y−11)=1\frac{\partial}{\partial x_2}r_1(\vec{x}) = \frac{\partial}{\partial y} (x^2 + y - 11) = 1∂x2​∂​r1​(x)=∂y∂​(x2+y−11)=1
            
            // second row of Jacobian, derivatives of second residual
            let d2_x = 1.;     // ∂∂x1r2(x⃗)=∂∂x(x+y2−7)=1\frac{\partial}{\partial x_1}r_2(\vec{x}) = \frac{\partial}{\partial x} (x + y^2 - 7) = 1∂x1​∂​r2​(x)=∂x∂​(x+y2−7)=1
            let d2_y = 2. * y; // ∂∂x2r2(x⃗)=∂∂y(x+y2−7)=2y\frac{\partial}{\partial x_2}r_2(\vec{x}) = \frac{\partial}{\partial y} (x + y^2 - 7) = 2y∂x2​∂​r2​(x)=∂y∂​(x+y2−7)=2y

            Some(Matrix2::new(
                d1_x, d1_y,
                d2_x, d2_y,
            ))
        }
    }

    let problem = ExampleProblem {
        // the initial guess for x⃗\vec{x}x
        p: Vector2::new(1., 1.),
    };
    let (_result, report) = LevenbergMarquardt::new().minimize(problem);
    assert!(report.termination.was_successful());
    assert!(report.objective_function.abs() < 1e-10);
}

fn get_solo_approximation(data: Vec<f64>) {
    let len = data.len();
    impl LeastSquaresProblem<f64, Dyn, Dyn> for OneDeviceProblem {
        type ParameterStorage = DVector<f64>;
        type ResidualStorage = Owned<f64, U3>;
        type JacobianStorage = Owned<f64, U3, U3>;

        fn set_params(&mut self, x: &nalgebra::Vector<f64, U3, Self::ParameterStorage>) {
            
        }

        fn params(&self) -> nalgebra::Vector<f64, U3, Self::ParameterStorage> {
            nalgebra::Vector3::new(self.nu0, self.x0, self.v0)
        }

        fn residuals(&self) -> Option<nalgebra::Vector<f64, U3, Self::ResidualStorage>> {
            todo!()
        }

        fn jacobian(&self) -> Option<nalgebra::Matrix<f64, U3, U3, Self::JacobianStorage>> {
            todo!()
        }
    }
}