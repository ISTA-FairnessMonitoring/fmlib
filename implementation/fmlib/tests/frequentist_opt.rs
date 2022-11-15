// #[cfg(test)]
// mod tests {
//     use fmlib::{envs::mc::Mc, monitors::frequentist_opt:{frequentist_opt::FrequentistOpt, builder::FrequentistOptBuilder}, op, util};

//     fn _run(
//         mc: &mut Mc,
//         monitor: &mut FrequentistOpt<i32>,
//         n: i32
//     ) -> (f64, f64) {
//         let mut result = (0.0, 0.0);
//         monitor.init();
//         for _ in 0..n {
//             let sigma = mc.next().unwrap();
//             result = monitor.next(sigma);
//             // println!("i: {}, sigma: {}, result: {:?}", i, sigma, result);
//         }

//         result
//     }

//     #[test]
//     fn test_frequentist_prod() {
//         let mut mc = util::markov_chain_7_state();
        
//         let mut m =
//         FrequentistOptBuilder::<i32>::new()
//         .set_delta(0.05)
//         .add_var(1, 2) // 0: p_{1, 2}
//         .add_var(4, 5) // 1: p_{4, 5}
//         .add_bin_op(0, 1, op::BinOp::Prod) // 2: p_{1, 2} * p_{4, 5} = 0.28
//         .build();

//         let result = _run(&mut mc, &mut m, 2000);
        
//         let (l, r) = result;
//         println!("result: [{:.6}, {:.6}]", l, r);
//         assert!(l <= 0.28 && 0.28 <= r);
//     }

//     #[test]
//     fn test_frequentist_prod_dep() {
//         let mut mc = util::markov_chain_7_state();

//         let mut m =
//         FrequentistOptBuilder::<i32>::new()
//         .set_delta(0.05)
//         .add_var(1, 2) // 0: p_{1, 2}
//         .add_var(1, 3) // 1: p_{1, 3}
//         .add_bin_op(0, 1, op::BinOp::ProdDependent) // 2: p_{1, 2} * p_{1, 3} = 0.24
//         .build();

//         let result = _run(&mut mc, &mut m, 4000);
        
//         let (l, r) = result;
//         println!("result: [{:.6}, {:.6}]", l, r);
//         assert!(l <= 0.24 && 0.24 <= r);
//     }

//     #[test]
//     fn test_frequentist_simple() {
//         let mut mc = util::markov_chain_7_state();

//         let mut m =
//         FrequentistOptBuilder::<i32>::new()
//         .set_delta(0.05)
//         .add_var(1, 2) // 0: p_{1, 2}
//         .add_var(4, 5) // 1: p_{4, 5}
//         .add_var(0, 4) // 2: p_{0, 4}
//         .add_bin_op(0, 1, op::BinOp::Prod) // 3: p_{1, 2} * p_{4, 5}
//         .add_bin_op(3, 2, op::BinOp::Subtract) // 4: p_{1, 2} * p_{4, 5} - p_{0, 4} = -0.22
//         .build();
        
//         let result = _run(&mut mc, &mut m, 2000);
        
//         let (l, r) = result;
//         println!("result: [{:.6}, {:.6}]", l, r);
//         assert!(l <= -0.22 && -0.22 <= r);
//     }

//     #[test]
//     fn test_frequentist_prod_dep_square() {
//         let mut mc = util::markov_chain_7_state();

//         let mut m =
//         FrequentistOptBuilder::<i32>::new()
//         .set_delta(0.05)
//         .add_var(1, 2) // 0: p_{1, 2}
//         .add_var(1, 2) // 1: p_{1, 3}
//         .add_bin_op(0, 1, op::BinOp::ProdDependent) // 2: p_{1, 2} * p_{1, 2} = 0.16
//         .add_var(4, 5) // 3: p_{4, 5}
//         .add_var(4, 5) // 4: p_{4, 5}
//         .add_bin_op(3, 4, op::BinOp::ProdDependent) // 5: p_{4, 5} * p_{4, 5} = 0.49
//         .add_bin_op(2, 5, op::BinOp::Prod) // 6: 2*5 = 0.0784
//         .build();

//         let result = _run(&mut mc, &mut m, 4000);
        
//         let (l, r) = result;
//         println!("result: [{:.6}, {:.6}]", l, r);
//         assert!(l <= 0.0784 && 0.0784 <= r);
//     }

//     #[test]
//     fn test_frequentist_prod_dep_cube() {
//         let mut mc = util::markov_chain_7_state();

//         let mut m =
//         FrequentistOptBuilder::<i32>::new()
//         .set_delta(0.05)
//         .add_var(4, 6) // 0: p_{4, 6}
//         .add_var(4, 6) // 1: p_{4, 6}
//         .add_var(4, 5) // 2: p_{4, 5}
//         .add_bin_op(0, 1, op::BinOp::ProdDependent) // 3: p_{4, 6}^2 = 0.09
//         .add_bin_op(2, 3, op::BinOp::ProdDependent) // 4: p_{4, 6}^2 * p_{4, 5} = 0.063
//         .build();

//         let result = _run(&mut mc, &mut m, 6000);
        
//         let (l, r) = result;
//         println!("result: [{:.6}, {:.6}]", l, r);
//         assert!(l <= 0.063 && 0.063 <= r);
//     }
// }