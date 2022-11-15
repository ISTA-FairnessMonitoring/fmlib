#[cfg(test)]
mod tests {
    use fmlib::{monitors::bayesian_exp::{bayesian_exp::{BayesianExp, ANFExpr}, builder::{ANFTermBuilder, BayesianExpBuilder}}, envs::mc::Mc, util};

    fn _run(
        mc: &mut Mc,
        monitor: &mut BayesianExp<i32>,
        n: i32
    ) -> Option<f64> {
        let mut result = None;
        for i in 0..n {
            let sigma = mc.next().unwrap();
            if i == 0 {
                monitor.init(sigma);
            } else {
                result = monitor.next(sigma);
            }
            if i % 1 == 0 {
                // println!("i: {}, sigma: {}, result: {:?}", i, sigma, result);
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

        let phi: ANFExpr<i32> = ANFExpr { terms: vec![t1, t2] };

        let mut m =
        BayesianExpBuilder::<i32>::new()
        .set_expr(phi)
        .set_sym_const(1, 7)
        .set_sym_const(4, 7)
        .set_sym_const(0, 7)
        .set_trans_const(1, 2, 1)
        .set_trans_const(4, 5, 1)
        .set_trans_const(0, 4, 1)
        .build();
        
        let result = _run(&mut mc, &mut m, 5000).unwrap();
        println!("result: {:?}", result);
    }
}