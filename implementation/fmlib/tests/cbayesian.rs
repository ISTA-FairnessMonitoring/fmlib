#[cfg(test)]
mod tests {
    use fmlib::{
        monitors::{
            cbayesian::cbayesian::CBayesian,
            bayesian::builder::BayesianBuilder
        },
        envs::mc::Mc,
        util,
        op::{BinOp, UnaryOp}
    };


    fn _run(
        mc: &mut Mc,
        monitor: &mut CBayesian<i32>,
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
            println!("i: {}, sigma: {}, result: {:?}", i, sigma, result);
        }

        result
    }

    #[test]
    fn test_cbayesian_subtract() {
        let mut mc = util::markov_chain_7_state();
        
        let exp_monitor =
        BayesianBuilder::<i32>::new()
        .add_freq(1, 2, 7.0, 1.0)          // 0: p_{1, 2}
        .add_freq(4, 5, 7.0, 1.0)          // 1: p_{4, 5}
        .add_bin_op(0, 1, BinOp::Subtract) // 2: p_{1, 2} - p_{4, 5} = -0.3
        .build();
    
        let exp2_monitor =
        BayesianBuilder::<i32>::new()
        .add_freq(1, 2, 7.0, 1.0)               // 0: p_{1, 2}
        .add_unary_op(0, 1.0, UnaryOp::Square)  // 1: p_{1, 2}^2
        .add_freq(4, 5, 7.0, 1.0)               // 2: p_{4, 5}
        .add_unary_op(2, 1.0, UnaryOp::Square)  // 3: p_{4, 5}^2
        .add_bin_op(0, 2, BinOp::Prod)          // 4: p_{1, 2} * p_{4, 5}
        .add_unary_op(4, 2.0, UnaryOp::Prod)    // 5: 2 * p_{1, 2} * p_{4, 5}
        .add_bin_op(1, 3, BinOp::Sum)           // 6: p_{1, 2}^2 + p_{4, 5}^2
        .add_bin_op(6, 5, BinOp::Subtract)      // 7: p_{1, 2}^2 + p_{4, 5}^2 - 2 * p_{1, 2} * p_{4, 5} = 0.09
        .build();

        let delta: f64 = 0.05;
        
        let mut m = CBayesian {
            delta, exp_monitor, exp2_monitor
        };

        let (l, r) = _run(&mut mc, &mut m, 10000);

        println!("result: [{:.6}, {:.6}]", l, r);
    }

}