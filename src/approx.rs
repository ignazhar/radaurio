use gomez::nalgebra as na;
use gomez::{Domain, Function, OptimizerDriver, Problem};
use na::{Dyn, IsContiguous};

use crate::OneDeviceSolution;

pub fn example_usage() -> () {
    // Objective function is represented by a struct.
    struct Rosenbrock {
        a: f64,
        b: f64,
    }

    impl Problem for Rosenbrock {
        // Field type, f32 or f64.
        type Field = f64;

        // Domain of the function.
        fn domain(&self) -> Domain<Self::Field> {
            Domain::unconstrained(1)
        }
    }

    impl Function for Rosenbrock {
        // Body of the function, taking x and returning f(x).
        fn apply<Sx>(&self, x: &na::Vector<Self::Field, Dyn, Sx>) -> Self::Field
        where
            Sx: na::Storage<Self::Field, Dyn> + IsContiguous,
        {
            self.a * x[0].powi(2) + self.b
        }
    }

    let f = Rosenbrock { a: 1.0, b: 1.0 };
    let mut optimizer = OptimizerDriver::builder(&f)
        .with_initial(vec![-1.0])
        .build();

    let (x, fx) = optimizer
        .find(|state| state.fx() <= 1e-6 || state.iter() >= 15)
        .expect("optimizer error");
    // .unwrap();

    println!("f(x) = {fx}\tx = {x:?}");
}

// speed of sound (T = 20 degrees Celsius)
pub const C: f64 = 343.0;

// fn find_return(optimizer: &OptimizerDriver<'_, OneDeviceProblem, TrustRegion<OneDeviceProblem>>, initial_params: vec<f64>) {

// }

#[derive(Debug)]
struct GetResult {
    result: (Vec<f64>, f64),
    iter: u32,
    error: Option<gomez::algo::trust_region::TrustRegionError>,
}

pub fn one_device_approximation(data: Vec<f64>) -> OneDeviceSolution {
    struct OneDeviceProblem {
        data: Vec<f64>,
        tau0: f64,
    }

    // (x0, d, v0)
    impl Problem for OneDeviceProblem {
        type Field = f64;

        fn domain(&self) -> Domain<Self::Field> {
            // Domain::unconstrained(2)
            Domain::rect(vec![-100000.0, -100000.0, -100000.0], vec![100000.0, 100000.0, 100000.0])
        }
    }

    impl Function for OneDeviceProblem {
        fn apply<Sx>(&self, values: &na::Vector<Self::Field, Dyn, Sx>) -> Self::Field
        where
            Sx: na::Storage<Self::Field, Dyn> + IsContiguous,
        {
            let (x0, d, v0) = (values[0], values[1], values[2]);
            let tau0 = self.tau0;

            // fn get_value(x: f64) -> f64 {
            //     nu0
            // }

            let get_value = |t: f64| -> f64 {
                let alpha = x0 + v0 * t + v0 * tau0;
                let beta = x0 + v0 * t;
                #[allow(non_snake_case)]
                let A = (d.powi(2) + alpha.powi(2)).sqrt();
                #[allow(non_snake_case)]
                let B = (d.powi(2) + beta.powi(2)).sqrt();
                let nu = C / (tau0 * C + A - B);
                nu
            };

            let mut result = 0.0;
            for (x, y) in self.data.iter().enumerate() {
                result += (get_value(x as f64) - y).powi(2);
            }
            result
        }
    }

    // d
    // nu0: f64, ///// ???
    // x0: f64,
    // v0: f64,
    // // not changeable
    // tau0: f64,

    let current_problem = OneDeviceProblem {
        data: data,
        tau0: 1.0,
    };
    let mut optimizer = OptimizerDriver::builder(&current_problem)
        .with_initial(vec![200.0, 40.0, 50.0])
        .build();

    fn find_return(optimizer: &mut OptimizerDriver<'_, OneDeviceProblem, gomez::algo::TrustRegion<OneDeviceProblem>>) -> GetResult {
        let mut last_state = GetResult { result: (optimizer.x().iter().cloned().collect(), optimizer.fx()), iter: 0, error: None };
        loop {
            println!("last_state: {:?}", last_state);
            match optimizer.next() {
                Err(e) => {
                    last_state.error = Some(e);
                    break;
                },
                Ok(new_result) => {
                    last_state.result = (new_result.0.iter().cloned().collect(), new_result.1);
                    last_state.iter += 1;
                }
            }
            // conditions: TODO: implement more general
            if last_state.result.1 < 1e-6 || last_state.iter >= 100 {
                break;
            }
        }
        last_state
    }

    /*let result = optimizer
        .find(|state| {
            println!(
                "it={} fx={} [x0, v0]={:?}",
                state.iter(),
                state.fx(),
                state.x()
            );
            // last_x = state.x();
            // last_fx = state.fx();Z
            state.fx() <= 1e-6 || state.iter() >= 100
        });
        // .expect("optimizer error")
        // .inspect_err(|e| eprintln!("error: {e}"))
    */
    
    // if let Err(e) = result {
    //     eprintln!("error: {}", e);
    // } else {
    //     let (x, fx) = result.unwrap();
    //     println!("found best : {} at {:?}", fx, x);
    // }

    let result = find_return(&mut optimizer);
    println!("final result: {:?}", result);

    println!("{}", optimizer.name());

    OneDeviceSolution { x0: result.result.0[0], d: result.result.0[1], v0: result.result.0[2], tau0: 1.0 }
}
