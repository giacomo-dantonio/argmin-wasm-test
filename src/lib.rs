mod test_function;

use argmin::prelude::*;
use argmin::solver::gradientdescent::SteepestDescent;
use argmin::solver::linesearch::MoreThuenteLineSearch;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve() {
    // Define cost function (must implement `ArgminOperator`)
    let cost = test_function::TestFunction { a: 1.0, b: 1.0, c: 1.0 };
    
    // Define initial parameter vector
    let init_param: Vec<f64> = vec![-1.2, 1.0];
    
    // Set up line search
    let linesearch = MoreThuenteLineSearch::new();
    
    // Set up solver
    let solver = SteepestDescent::new(linesearch);
    
    // Run solver
    let res = Executor::new(cost, solver, init_param)
        // Add an observer which will log all iterations to the terminal
        // .add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
        // Set maximum iterations to 10
        .max_iters(10)
        // run the solver on the defined problem
        .run()
        .unwrap();
}