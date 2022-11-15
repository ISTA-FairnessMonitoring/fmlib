#[cfg(test)]
mod tests {
    use fmlib::{
        monitors::bayesian::{
            bayesian::Bayesian,
            builder::BayesianBuilder,
            tv::Fraction
        },
        op::BinOp,
        util
    };
    use fmlib::envs::mc::Mc;

    fn _run(
        mc: &mut Mc,
        monitor: &mut Bayesian<i32>,
        n: i32
    ) -> Fraction {
        let mut result = Fraction { num: 0.0, denom: 1.0 };
        for i in 0..n {
            let sigma = mc.next().unwrap();
            if i == 0 {
                monitor.init(sigma);
            } else {
                result = monitor.next(sigma);
            }
            // println!("i: {}, sigma: {}, result: {:?}", i, sigma, result);
        }

        result
    }

    #[test]
    fn test_bayesian_sum() {
        let mut mc = util::markov_chain_7_state();
        
        let mut m =
            BayesianBuilder::<i32>::new()
            .add_freq(1, 2, 7.0, 1.0) // 0: p_{1, 2}
            .add_freq(4, 5, 7.0, 1.0) // 1: p_{4, 5}
            .add_bin_op(0, 1, BinOp::Sum) // 2: p_{1, 2} + p_{4, 5} = 1.1
            .build();
        
        let result = _run(&mut mc, &mut m, 10000);
        println!("result: {:?}", result.to_number());
        assert!((result.to_number() - 1.1).abs() <= 0.05);
    }

    #[test]
    fn test_bayesian_simple() {
        let mut mc = util::markov_chain_7_state();

        let mut m =
            BayesianBuilder::<i32>::new()
            .add_freq(1, 2, 1.0, 2.0) // 0: p_{1, 2}
            .add_freq(4, 5, 1.0, 2.0) // 1: p_{4, 5}
            .add_freq(0, 4, 1.0, 2.0) // 2: p_{0, 4}
            .add_bin_op(0, 1, BinOp::Prod) // 3: p_{1, 2} * p_{4, 5}
            .add_bin_op(3, 2, BinOp::Subtract) // 4: p_{1, 2} * p_{4, 5} - p_{0, 4} = -0.22
            .build();

        let result = _run(&mut mc, &mut m, 10000);
        println!("result: {:?}", result.to_number());
        assert!((result.to_number() + 0.22).abs() <= 0.025);
    }
}