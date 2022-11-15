#[cfg(test)]
mod tests {
    use fmlib::{monitors::{bayesian_confint::bayesian_confint::BayesianConfInt, bayesian_exp::{builder::{ANFTermBuilder, BayesianExpBuilder}, bayesian_exp::ANFExpr}}, envs::mc::Mc, util};

    fn _run(
        mc: &mut Mc,
        monitor: &mut BayesianConfInt<i32>,
        n: i32
    ) -> (f64, f64) {
        let mut result = (0.0, 0.0);
        for i in 0..n {
            let sigma = mc.next().unwrap();
            if i == 0 {
                monitor.init(sigma);
            } else {
                result = monitor.next(sigma);
            }
            if i % 1 == 0 {
                println!("i: {}, sigma: {}, result: {:?}", i, sigma, result);
            }
        }

        result
    }

    #[test]
    fn test_bayesian_simple() {
        let mut mc = util::markov_chain_7_state();

        let t1 = 
        ANFTermBuilder::<i32>::new()
        .add_var(1, 2, 1)
        .add_var(4, 5, 1)
        .set_constant(1.0)
        .build();

        let t2 =
        ANFTermBuilder::<i32>::new()
        .add_var(0, 4, 1)
        .set_constant(-1.0)
        .build();

        let phi: ANFExpr<i32> = ANFExpr { terms: vec![t1, t2] }; // phi = v_{12} * v_{45} - v_{01}

        let exp_monitor =
        BayesianExpBuilder::<i32>::new()
        .set_expr(phi)
        .set_sym_const(1, 7)
        .set_sym_const(4, 7)
        .set_sym_const(0, 7)
        .set_trans_const(1, 2, 1)
        .set_trans_const(4, 5, 1)
        .set_trans_const(0, 4, 1)
        .build();
        
        let tp_1 =
        ANFTermBuilder::<i32>::new()
        .add_var(1, 2, 2)
        .add_var(4, 5, 2)
        .set_constant(1.0)
        .build();

        let tp_2 = 
        ANFTermBuilder::<i32>::new()
        .add_var(0, 4, 2)
        .set_constant(1.0)
        .build();

        let tp_3 =
        ANFTermBuilder::<i32>::new()
        .add_var(1, 2, 1)
        .add_var(4, 5, 1)
        .add_var(0, 4, 1)
        .set_constant(-2.0)
        .build();

        let phi_p = ANFExpr { terms: vec![tp_1, tp_2, tp_3] };

        let exp2_monitor =
        BayesianExpBuilder::<i32>::new()
        .set_expr(phi_p)
        .set_sym_const(1, 7)
        .set_sym_const(4, 7)
        .set_sym_const(0, 7)
        .set_trans_const(1, 2, 1)
        .set_trans_const(4, 5, 1)
        .set_trans_const(0, 4, 1)
        .build();

        let delta = 0.05;

        let mut m = BayesianConfInt {
            exp_monitor, exp2_monitor, delta, verdict: (0.0, 0.0)
        };

        let (l, r) = _run(&mut mc, &mut m, 10000);
        println!("result: ({}, {})", l, r);
        assert!(l <= -0.22 && -0.22 <= r);
    }
}