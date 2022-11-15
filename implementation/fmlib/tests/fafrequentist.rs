#[cfg(test)]
mod tests {
    use fmlib::{
        monitors::fafrequentist::{
            builder::FaFrequentistBuilder,
            frequentist::FaFrequentist,
            tv::EvalResult,
        },
        op::BinOp,
        util,
    };
    use fmlib::envs::mc::Mc;


    fn _run(
        mc: &mut Mc,
        monitor: &mut FaFrequentist<i32>,
        n: i32
    ) -> EvalResult {
        let mut result = EvalResult { ..Default::default() };
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
    fn test_frequentist_sum() {
        let mut mc = util::markov_chain_7_state();
        
        let mut m =
            FaFrequentistBuilder::<i32>::new()
            .add_freq(1, 2) // 0: p_{1, 2}
            .add_freq(4, 5) // 1: p_{4, 5}
            .add_bin_op(0, 1, BinOp::Sum) // 2: p_{1, 2} + p_{4, 5} = 1.1
            .set_delta(0.05)
            .build();
        
        let result = _run(&mut mc, &mut m, 10000);

        println!("result: {:?}", result);
    }

    #[test]
    fn test_frequentist_simple() {
        let mut mc = util::markov_chain_7_state();

        let mut m =
            FaFrequentistBuilder::<i32>::new()
            .add_freq(1, 2) // 0: p_{1, 2}
            .add_freq(4, 5) // 1: p_{4, 5}
            .add_freq(0, 4) // 2: p_{0, 4}
            .add_bin_op(0, 1, BinOp::Prod) // 3: p_{1, 2} * p_{4, 5}
            .add_bin_op(3, 2, BinOp::Subtract) // 4: p_{1, 2} * p_{4, 5} - p_{0, 4}
            .add_constant(2.0) // 5: 2.0
            .add_bin_op(4, 5, BinOp::Sum) // 6: p_{1, 2} * p_{4, 5} - p_{0, 4} + 2.0 = 1.78
            .set_delta(0.05)
            .build();
        

        for v in m.vertices.borrow().iter() {
            println!("{:?}", v);
        }

        let result = _run(&mut mc, &mut m, 10000);
        println!("result: {:?}", result);
    }
}