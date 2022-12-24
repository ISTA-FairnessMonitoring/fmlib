#[cfg(test)]
mod tests {
    use fmlib::{
        op::{UnaryOp, BinOp},
        monitors::frequentist::{
            frequentist::Frequentist,
            builder::FrequentistBuilder
        },
        envs::admission::{amvm::{Amvm, Amv}, memoryless::Ammc}
    };
    use fmlib::envs::lending::{
        markov_chain::LendingMarkovChain,
        markov_chain::{Decision, Payback},
        mapper::*,
    };
    use fmlib::envs::mc::Mc;
    use fmlib::util;
    
    fn _run(
        mc: &mut Mc,
        monitor: &mut Frequentist<i32>,
        n: i32
    ) -> Option<(f64, f64)> {
        let mut result = None;
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

    fn _run_lending_memless(
        mc: &mut LendingMarkovChain,
        mapper: &LendingVertexMapper,
        monitors: &mut Vec<&mut Frequentist<Lmv>>,
        n: i32
    ) -> Vec<Option<(f64, f64)>> {
        let mut result = vec![None; monitors.len()];
        for i in 0..n {
            let sigma = mapper.map(&mc.next().unwrap());
            if let None = sigma {
                continue;
            }

            let sigma = sigma.unwrap();

            for j in 0..monitors.len() {
                if i == 0 {
                    monitors[j].init(sigma.clone());
                } else {
                    result[j] = monitors[j].next(sigma.clone());
                }
                // println!("i: {}, sigma: {:?}, result: {:?}", i, sigma, result);
            }
        }

        result
    }

    fn _run_admission_memless(
        mc: &mut Ammc,
        mapper: &Amvm,
        monitors: &mut Vec<&mut Frequentist<Amv>>,
        n: i32
    ) -> Vec<Option<(f64, f64)>> {
        let mut result = vec![None; monitors.len()];
        for i in 0..n {
            let sigma = mapper.map(&mc.next().unwrap());
            if let None = sigma {
                continue;
            }

            let sigma = sigma.unwrap();

            for j in 0..monitors.len() {
                if i == 0 {
                    monitors[j].init(sigma.clone());
                } else {
                    result[j] = monitors[j].next(sigma.clone());
                }
                // println!("i: {}, sigma: {:?}, result: {:?}", i, sigma, result);
            }
        }

        result
    }

    #[test]
    fn test_frequentist_sum() {
        let mut mc = util::markov_chain_7_state();
        
        let mut m =
            FrequentistBuilder::<i32>::new()
            .set_delta(0.05)
            .add_freq(1, 2) // 0: p_{1, 2}
            .add_freq(4, 5) // 1: p_{4, 5}
            .add_bin_op(0, 1, BinOp::Sum) // 2: p_{1, 2} + p_{4, 5} = 1.1
            .build();
        
        let result = _run(&mut mc, &mut m, 10000);

        let (l, r) = result.unwrap();
        println!("result: [{:.6}, {:.6}]", l, r);
        assert!((l <= 1.1) && (1.1 <= r));
    }

    #[test]
    fn test_frequentist_subtract() {
        let mut mc = util::markov_chain_7_state();
        
        let mut m =
            FrequentistBuilder::<i32>::new()
            .set_delta(0.05)
            .add_freq(1, 2) // 0: p_{1, 2}
            .add_freq(4, 5) // 1: p_{4, 5}
            .add_bin_op(0, 1, BinOp::Subtract) // 2: p_{1, 2} - p_{4, 5} = -0.3
            .build();
        
        let result = _run(&mut mc, &mut m, 10000);

        let (l, r) = result.unwrap();
        println!("result: [{:.6}, {:.6}]", l, r);
        assert!((l <= -0.3) && (-0.3 <= r));
    }

    #[test]
    fn test_frequentist_prod() {
        let mut mc = util::markov_chain_7_state();
        
        let mut m =
        FrequentistBuilder::<i32>::new()
            .set_delta(0.05)
            .add_freq(1, 2) // 0: p_{1, 2}
            .add_freq(4, 5) // 1: p_{4, 5}
            .add_bin_op(0, 1, BinOp::Prod) // 2: p_{1, 2} * p_{4, 5} = 0.28
            .build();

        let result = _run(&mut mc, &mut m, 10000);
        
        let (l, r) = result.unwrap();
        println!("result: [{:.6}, {:.6}]", l, r);
        assert!((l <= 0.28) && (0.28 <= r));
    }

    #[test]
    fn test_frequentist_prod_dep() {
        let mut mc = util::markov_chain_7_state();
        let mut m =
            FrequentistBuilder::<i32>::new()
            .set_delta(0.05)
            .add_freq(1, 2) // 0: p_{1, 2}
            .add_freq(1, 3) // 1: p_{1, 3}
            .add_bin_op(0, 1, BinOp::ProdDependent) // 2: p_{1, 2} * p_{1, 3} = 0.24
            .build();
        
        let result = _run(&mut mc, &mut m, 40000);

        let (l, r) = result.unwrap();
        println!("result: [{:.6}, {:.6}]", l, r);
        assert!((l <= 0.24) && (0.24 <= r));
    }

    #[test]
    fn test_frequentist_simple() {
        let mut mc = util::markov_chain_7_state();

        let mut m =
            FrequentistBuilder::<i32>::new()
            .set_delta(0.05)
            .add_freq(1, 2) // 0: p_{1, 2}
            .add_freq(4, 5) // 1: p_{4, 5}
            .add_freq(0, 4) // 2: p_{0, 4}
            .add_bin_op(0, 1, BinOp::Prod) // 3: p_{1, 2} * p_{4, 5}
            .add_bin_op(3, 2, BinOp::Subtract) // 4: p_{1, 2} * p_{4, 5} - p_{0, 4}
            .add_unary_op(4, 2.0, UnaryOp::Sum) // 5: p_{1, 2} * p_{4, 5} - p_{0, 4} + 2 = 1.78
            .build();
        

        for v in m.vertices.borrow().iter() {
            println!("{:?}", v);
        }

        let result = _run(&mut mc, &mut m, 10000);
        
        let (l, r) = result.unwrap();
        println!("result: [{:.6}, {:.6}]", l, r);
        assert!((l <= 1.78) && (1.78 <= r));
    }

    #[test]
    fn test_frequentist_inverse_atomic() {
        let mut mc = util::markov_chain_7_state();
        let mut m = FrequentistBuilder::<i32>::new()
            .set_delta(0.05)
            .add_freq(4, 5)
            .add_unary_op(0, 0.0, UnaryOp::InverseAtomic)
            .build();

        let result = _run(& mut mc, &mut m, 100000);

        let (l, r) = result.unwrap();
        println!("result: [{:.6}, {:.6}]", l, r);
        assert!((l <= 1.42) && (1.42 <= r));
    }

    #[test]
    fn test_frequentist_demographic_lmmc() {
        let mut mc = util::markov_chain_lending_medium();
        let mapper = LendingVertexMapper {};

        let (s, s_prime) = ((0, 0), (1, 0));
        let d = Decision::Accept;

        let mut m = FrequentistBuilder::<Lmv>::new()
            .set_delta(0.05)
            .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
            .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 1
            .add_bin_op(0, 1, BinOp::Subtract)
            .build();
        
        let result = _run_lending_memless(&mut mc, &mapper, &mut vec![&mut m], 100000);

        let (l, r) = result[0].unwrap();
        println!("result: [{:.6}, {:.6}]", l, r);
        assert!((l <= 0.2) && (0.2 <= r));
    }
    
    #[test]
    fn test_frequentist_eq_opp_lmmc() {
        let mut mc = util::markov_chain_lending_medium();
        let mapper = LendingVertexMapper {};
        let (s, s_prime) = ((0, 0), (1, 0));
        let d = Decision::Accept;
        let p = Payback::Success;

        let mut m = FrequentistBuilder::<Lmv>::new()
            .set_delta(0.05)
            .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
            .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
            .add_bin_op(0, 1, BinOp::Prod) // 2
            .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
            .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
            .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
            .add_bin_op(4, 5, BinOp::Prod) // 6
            .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
            .add_bin_op(3, 7, BinOp::Subtract) // 8
            .build();
        
        let result = _run_lending_memless(&mut mc, &mapper, &mut vec![&mut m], 100000);
        
        let (l, r) = result[0].unwrap();
        println!("result: [{:.6}, {:.6}]", l, r);
        assert!((l <= 0.2) && (0.2 <= r));
    }

    #[test]
    fn test_frequentist_eq_odd_lmmc() {
        let mut mc = util::markov_chain_lending_medium();

        let mapper = LendingVertexMapper {};

        let (s, s_prime) = ((0, 0), (1, 0));
        let d = Decision::Accept;
        let (p, p_prime) = (Payback::Success, Payback::Fail);

        let (c3, c4) = (0.4, 0.4);

        let mut m1 = FrequentistBuilder::<Lmv>::new()
        .set_delta(0.05)
        .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
        .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
        .add_bin_op(0, 1, BinOp::Prod) // 2
        .add_unary_op(2, 1.0 / c3, UnaryOp::Prod) // 3
        .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
        .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
        .add_bin_op(4, 5, BinOp::Prod) // 6
        .add_unary_op(6, 1.0 / c4, UnaryOp::Prod) // 7
        .add_bin_op(3, 7, BinOp::Subtract) // 8
        .build();
        
        let mut m2 = FrequentistBuilder::<Lmv>::new()
        .set_delta(0.05)
        .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
        .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p_prime)) // 1
        .add_bin_op(0, 1, BinOp::Prod) // 2
        .add_unary_op(2, 1.0 / (1.0 - c3), UnaryOp::Prod) // 3
        .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
        .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p_prime)) // 5
        .add_bin_op(4, 5, BinOp::Prod) // 6
        .add_unary_op(6, 1.0 / (1.0 - c4), UnaryOp::Prod) // 7
        .add_bin_op(3, 7, BinOp::Subtract) // 8
        .build();

        let result = _run_lending_memless(&mut mc, &mapper, &mut vec![&mut m1, &mut m2], 100000);
        let (l1, r1) = result[0].unwrap();
        let (l2, r2) = result[1].unwrap();

        println!("result(1): [{:.6}, {:.6}]", l1, r1);
        println!("result(2): [{:.6}, {:.6}]", l2, r2);
    }

    #[test]
    fn test_frequentist_variance_sum() {
        let mut mc = util::markov_chain_7_state();

        let mut m = FrequentistBuilder::<i32>::new()
            .set_delta(0.05)
            .add_freq(1, 2) // 0: p_{1, 2}
            .add_freq(4, 5) // 1: p_{4, 5}
            .add_bin_op(0, 1, BinOp::Sum) // 2: p_{1, 2} + p_{4, 5} = 1.1
            .build();
        
        _run(&mut mc, &mut m, 10000);
        
        println!("variance: {:.6}", m.variance);
        assert!((m.variance - 0.45).abs() <= 0.07);
    }

    #[test]
    fn test_frequentist_admission_simple() {
        let mut mc = util::markov_chain_admission_small();

        let mapper = Amvm {};

        let mut m = FrequentistBuilder::<Amv>::new()
        .set_delta(0.05)
        .add_freq(Amv::Sample(true), Amv::Cost(0)) // 0
        .add_freq(Amv::Sample(true), Amv::Cost(2)) // 1
        .add_unary_op(0, 0.0, UnaryOp::Prod) // 2
        .add_unary_op(1, 2.0, UnaryOp::Prod) // 3
        .add_bin_op(2, 3, BinOp::Sum)
        .build();

        let result = _run_admission_memless(&mut mc, &mapper, &mut vec![&mut m], 100000);

        let (l, r) = result[0].unwrap();
        println!("result: [{:.6}, {:.6}]", l, r);
    }
}