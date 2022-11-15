// use crate::{
//     op,
//     op::{UnaryOp, BinOp},
//     runner,
//     util,
//     utile,
//     experiments,
//     monitors::{ 
//         frequentist::frequentist::Frequentist,
//         frequentist::builder::FrequentistBuilder,
//         fafrequentist::frequentist::FaFrequentist,
//         fafrequentist::builder::FaFrequentistBuilder,
//         bayesian::builder::BayesianBuilder,
//         cbayesian::cbayesian::CBayesian
//     },
//     monitor::Monitor,
//     envs::admission::{amvm::{Amvm, Amv}, memoryless::Ammc},
//     envs::lending::{
//             memoryless::Lmmc,
//             lv::{Decision, Payback},
//             lmvm::*,
//         },
//     envs::mc::Mc,
// };
// use std::time::{Duration, SystemTime};

// pub fn run() {
//     // For logging purposes
//     // _run_frequentist_mc();
//     // _run_fafrequentist_mc();n.pow(20)
//     let n = 100000;
//     let it = 4;
//     // println!(r#">>>> {{"experiment": "frequentist_lending_fair", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_eq_opp_lmmc(n, true);
//     // }
//     // println!(r#">>>> {{"experiment": "bayes_lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_lending(n, false);
//     // }



//     // println!(r#">>>> {{"algorithm": "base_frequentist", "experiment": "lending_dp", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_fafrequentist_lending_dp(n, false);
//     // }
//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "lending_dp", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_lending_dp(n, false);
//     // }



//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "lending_dp", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_lending_dp(n, false);
//     // }

    

//     // println!(r#">>>> {{"algorithm": "bayes_bp", "experiment": "lending_dp_bp", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_lending_dp_bp(n, false);
//     // }


//     // println!(r#">>>> {{"algorithm": "base_frequentist", "experiment": "admission_lv", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_fafrequentist_admission(n, true);
//     // }


//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_lv", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission(n, true);
//     // }


//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_a", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_medium_lv(n,true, vec![0.0, 1.0, 1.0, 1.0, 1.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_a", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_medium_lv(n,true, vec![0.0, 1.0, 1.0, 1.0, 1.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_b", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_medium_lv(n,true, vec![0.0, 0.25, 0.5, 0.75, 1.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_b", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_medium_lv(n,true, vec![0.0, 0.25, 0.5, 0.75, 1.0]);
//     // }


//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_c", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_medium_lv(n,true, vec![0.0, 0.0001, 0.0001, 0.0001, 1.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_c", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_medium_lv(n,true, vec![0.0, 0.0001, 0.0001, 0.0001, 1.0]);
//     // }


//     println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_a", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_bayesian_admission_medium_lv(n,true, vec![0.0, 1.0, 1.0, 1.0, 1.0]);
//     }

//     println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_a", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_frequentist_admission_medium_lv(n,true, vec![0.0, 1.0, 1.0, 1.0, 1.0]);
//     }

//     println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_b", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_bayesian_admission_medium_lv(n,true, vec![0.0, 0.25, 0.5, 0.75, 1.0]);
//     }

//     println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_b", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_frequentist_admission_medium_lv(n,true, vec![0.0, 0.25, 0.5, 0.75, 1.0]);
//     }


//     println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_c", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_bayesian_admission_medium_lv(n,true, vec![0.0, 0.0001, 0.0001, 0.0001, 1.0]);
//     }

//     println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_c", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_frequentist_admission_medium_lv(n,true, vec![0.0, 0.0001, 0.0001, 0.0001, 1.0]);
//     }

//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_mv_ss", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_small_lv(n,true, vec![0.0, 5.0, 10.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_mv_ss", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_small_lv(n,true, vec![0.0, 5.0, 10.0]);
//     // }


//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_lv_ss", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_small_lv(n,true, vec![0.0, 50000.0, 100000.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_lv_ss", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_small_lv(n,true, vec![0.0, 50000.0, 100000.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_sv_ms", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_small_lv(n,true, vec![0.0, 0.1, 1.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_sv_ms", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_small_lv(n,true, vec![0.0, 0.1, 1.0]);
//     // }


//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_mv_ms", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_small_lv(n,true, vec![0.0, 10.0, 100.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_mv_ms", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_small_lv(n,true, vec![0.0, 10.0, 100.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_lv_ms", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_small_lv(n,true, vec![0.0, 10000.0, 100000.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_lv_ms", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_small_lv(n,true, vec![0.0, 10000.0, 100000.0]);
//     // }


//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_sv_ls", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_small_lv(n,true, vec![0.0, 0.0000000, 1.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_sv_ls", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_small_lv(n,true, vec![0.0,  0.0000000, 1.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission_small_lv_ls", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_small_lv(n,true, vec![0.0, 10000.0, 10000000.0]);
//     // }

//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission_small_lv_ls", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission_small_lv(n,true, vec![0.0, 10000.0, 10000000.0]);
//     // }

   
//     // println!(r#">>>> {{"algorithm": "bayes_bp", "experiment": "admission_small_lv", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission_small_lv_bias(n,true);
//     // }

 

//     // println!(r#">>>> {{"algorithm": "frequentis_base", "experiment": "admission_small_lv", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_fafrequentist_admission_small_lv(n,true);
//     // }


//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_lending(n, false);
//     // }

    

//     // println!(r#">>>> {{"algorithm": "bayes_bp", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_lending_bp(n, false);
//     // }

//     // println!(r#">>>> {{"algorithm": "base_frequentist", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_fafrequentist_lending(n, false);
//     // }
//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_lending(n, false);
//     // }



//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_lending(n, false);
//     // }


//     // println!(r#">>>> {{"algorithm": "base_frequentist", "experiment": "admission", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_fafrequentist_admission(n, true);
//     // }


//     // println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_frequentist_admission(n, true);
//     // }


//     // println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     // for i in 0..it{
//     //     println!(r#">>>> {{"run": {:?}}}"#, i);
//     //     exp_bayesian_admission(n,true);
//     // }

    
// }

// fn exp_frequentist_lending(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;

//     let mut m = FrequentistBuilder::<Lmv>::new()
//         .set_delta(0.05)
//         .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//         .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
//         .add_bin_op(0, 1, BinOp::Prod) // 2
//         .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//         .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
//         .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
//         .add_bin_op(4, 5, BinOp::Prod) // 6
//         .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//         .add_bin_op(3, 7, BinOp::Subtract) // 8
//         .build();
    
//     println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//         1.0 / 0.4,
//         (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
//         (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
//         1.0/ 0.4,
//         (Lmv::Sample(s), Lmv::Decision(s, d)),
//         (Lmv::Decision(s, d), Lmv::Payback(s, p))
//     );
//     println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
//     let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// fn exp_fafrequentist_lending(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;

//     let mut m = FaFrequentistBuilder::<Lmv>::new()
//         .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//         .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
//         .add_bin_op(0, 1, BinOp::Prod) // 2
//         .add_constant(1.0 / 0.4) // 3
//         .add_bin_op(2, 3, BinOp::Prod) // 4
//         .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 5
//         .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 6
//         .add_bin_op(5, 6, BinOp::Prod) // 7
//         .add_constant(1.0 / 0.4) // 8
//         .add_bin_op(7, 8, BinOp::Prod) // 9
//         .add_bin_op(4, 9, BinOp::Subtract) // 10
//         .set_delta(0.05)
//         .build();
    
//     println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//         1.0 / 0.4,
//         (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
//         (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
//         1.0/ 0.4,
//         (Lmv::Sample(s), Lmv::Decision(s, d)),
//         (Lmv::Decision(s, d), Lmv::Payback(s, p))
//     );
//     println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
//     let result = _run_fafreq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// fn exp_bayesian_lending(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 100.0;

//     let mut exp_monitor =
//     BayesianBuilder::<Lmv>::new()
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), num_vertices,1.0)     // 0
//     .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p), num_vertices,1.0) // 1
//     .add_bin_op(0, 1, BinOp::Prod)             // 2: 0*1
//     .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3: 0*1 * 2.5
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d), num_vertices,1.0)     // 4
//     .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p), num_vertices,1.0) // 5
//     .add_bin_op(4, 5, BinOp::Prod)             // 6: 4*5
//     .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7: 4*5 * 2.5
//     .add_bin_op(3, 7, BinOp::Subtract)         // 8: 3 - 7 = 2.5 * (0*1 - 4*5)
//     .build();

//     let mut exp2_monitor =
//     // BayesianBuilder::<Lmv>::new()
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), num_vertices,1.0) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p), num_vertices,1.0) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d), num_vertices,1.0) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p), num_vertices,1.0) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 3, BinOp::Prod) // 8
//     // .add_bin_op(7, 7, BinOp::Prod) // 9
//     // .add_bin_op(3, 7, BinOp::Prod) // 10
//     // .add_unary_op(10, 2.0, UnaryOp::Prod) // 11
//     // .add_bin_op(8, 9, BinOp::Sum) // 12
//     // .add_bin_op(12, 11, BinOp::Subtract) // 12
//     // .build();
//     BayesianBuilder::<Lmv>::new()
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), num_vertices,1.0)     // 0
//     .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p), num_vertices,1.0) // 1
//     .add_unary_op(0, 1.0, UnaryOp::Square) // 2: 0^2
//     .add_unary_op(1, 1.0, UnaryOp::Square) // 3: 1^2
//     .add_bin_op(2, 3, BinOp::Prod)         // 4: 2*3 = 0^2 * 1^2
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d), num_vertices,1.0)     // 5
//     .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p), num_vertices,1.0) // 6
//     .add_unary_op(5, 1.0, UnaryOp::Square) //  7: 5^2
//     .add_unary_op(6, 1.0, UnaryOp::Square) //  8: 6^2
//     .add_bin_op(7, 8, BinOp::Prod)         //  9: 7*8 = 5^2 * 6^2
//     .add_bin_op(0, 1, BinOp::Prod)         // 10: 0*1
//     .add_bin_op(5, 6, BinOp::Prod)         // 11: 5*6
//     .add_bin_op(10, 11, BinOp::Prod)       // 12: 10*11 = 0*1 * 5*6
//     .add_unary_op(12, 2.0, UnaryOp::Prod)  // 13: 2*12 = 2 * 0*1*5*6
//     .add_bin_op(4, 9, BinOp::Sum)          // 14: 4+9 = 0^2*1^2 + 5^2*6^2
//     .add_bin_op(14, 13, BinOp::Subtract)   // 15: 14 - 13 = 0^2*1^2 + 5^2*6^2 - 2 * 0*1*5*6
//     .add_unary_op(15, 1.0 / (0.4*0.4), UnaryOp::Prod) // 16: 6.25 * 15
//     .build();

//     let mut m = CBayesian::<Lmv> {
//         delta, exp_monitor, exp2_monitor
//     };

//     let result = _run_bayes_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }


// fn exp_bayesian_lending_bp(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 100.0;

//     let mut exp_monitor =
//     BayesianBuilder::<Lmv>::new()
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), num_vertices+1000.0,1000.0)     // 0
//     .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p), num_vertices,1.0) // 1
//     .add_bin_op(0, 1, BinOp::Prod)             // 2: 0*1
//     .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3: 0*1 * 2.5
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d), num_vertices+1000.0,1.0)     // 4
//     .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p), num_vertices,1.0) // 5
//     .add_bin_op(4, 5, BinOp::Prod)             // 6: 4*5
//     .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7: 4*5 * 2.5
//     .add_bin_op(3, 7, BinOp::Subtract)         // 8: 3 - 7 = 2.5 * (0*1 - 4*5)
//     .build();

//     let mut exp2_monitor =
//     // BayesianBuilder::<Lmv>::new()
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), num_vertices,1.0) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p), num_vertices,1.0) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d), num_vertices,1.0) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p), num_vertices,1.0) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 3, BinOp::Prod) // 8
//     // .add_bin_op(7, 7, BinOp::Prod) // 9
//     // .add_bin_op(3, 7, BinOp::Prod) // 10
//     // .add_unary_op(10, 2.0, UnaryOp::Prod) // 11
//     // .add_bin_op(8, 9, BinOp::Sum) // 12
//     // .add_bin_op(12, 11, BinOp::Subtract) // 12
//     // .build();
//     BayesianBuilder::<Lmv>::new()
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), num_vertices+1000.0,1000.0)     // 0
//     .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p), num_vertices,1.0) // 1
//     .add_unary_op(0, 1.0, UnaryOp::Square) // 2: 0^2
//     .add_unary_op(1, 1.0, UnaryOp::Square) // 3: 1^2
//     .add_bin_op(2, 3, BinOp::Prod)         // 4: 2*3 = 0^2 * 1^2
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d), num_vertices+1000.0,1.0)     // 5
//     .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p), num_vertices,1.0) // 6
//     .add_unary_op(5, 1.0, UnaryOp::Square) //  7: 5^2
//     .add_unary_op(6, 1.0, UnaryOp::Square) //  8: 6^2
//     .add_bin_op(7, 8, BinOp::Prod)         //  9: 7*8 = 5^2 * 6^2
//     .add_bin_op(0, 1, BinOp::Prod)         // 10: 0*1
//     .add_bin_op(5, 6, BinOp::Prod)         // 11: 5*6
//     .add_bin_op(10, 11, BinOp::Prod)       // 12: 10*11 = 0*1 * 5*6
//     .add_unary_op(12, 2.0, UnaryOp::Prod)  // 13: 2*12 = 2 * 0*1*5*6
//     .add_bin_op(4, 9, BinOp::Sum)          // 14: 4+9 = 0^2*1^2 + 5^2*6^2
//     .add_bin_op(14, 13, BinOp::Subtract)   // 15: 14 - 13 = 0^2*1^2 + 5^2*6^2 - 2 * 0*1*5*6
//     .add_unary_op(15, 1.0 / (0.4*0.4), UnaryOp::Prod) // 16: 6.25 * 15
//     .build();

//     let mut m = CBayesian::<Lmv> {
//         delta, exp_monitor, exp2_monitor
//     };

//     let result = _run_bayes_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }





// fn exp_frequentist_admission(n:i32, big:bool) {
//     let mut mc = util::markov_chain_admission_large();
//     if big {
//         mc = util::markov_chain_admission_large_costs();
//     }
//     let mapper = Amvm {};

//     let mut m = FrequentistBuilder::<Amv>::new()
//     .set_delta(0.05)
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0)) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0)) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0)) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(2.0)) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(3.0)) // 4
//     .add_freq(Amv::Sample(true), Amv::Cost(5.0)) // 5
//     .add_freq(Amv::Sample(true), Amv::Cost(8.0)) // 6
//     .add_freq(Amv::Sample(true), Amv::Cost(13.0)) // 7
//     .add_freq(Amv::Sample(true), Amv::Cost(21.0)) // 8
//     .add_freq(Amv::Sample(true), Amv::Cost(34.0)) // 9
//     .add_freq(Amv::Sample(true), Amv::Cost(55.0)) // 10
//     .add_unary_op(0, 0.0, UnaryOp::Prod) // 11
//     .add_unary_op(1, 1.0, UnaryOp::Prod) // 12
//     .add_unary_op(2, 1.0, UnaryOp::Prod) // 13
//     .add_unary_op(3, 2.0, UnaryOp::Prod) // 14
//     .add_unary_op(4, 3.0, UnaryOp::Prod) // 15
//     .add_unary_op(5, 5.0, UnaryOp::Prod) // 16
//     .add_unary_op(6, 8.0, UnaryOp::Prod) // 17
//     .add_unary_op(7, 13.0, UnaryOp::Prod) // 18
//     .add_unary_op(8, 21.0, UnaryOp::Prod) // 19
//     .add_unary_op(9, 34.0, UnaryOp::Prod) // 20
//     .add_unary_op(10, 55.0, UnaryOp::Prod) // 21
//     .add_bin_op(11, 12, BinOp::Sum) // 22
//     .add_bin_op(22, 13, BinOp::Sum) // 23
//     .add_bin_op(23, 14, BinOp::Sum) // 24
//     .add_bin_op(24, 15, BinOp::Sum) // 25
//     .add_bin_op(25, 16, BinOp::Sum) // 26
//     .add_bin_op(26, 17, BinOp::Sum) // 27
//     .add_bin_op(27, 18, BinOp::Sum) // 28
//     .add_bin_op(28, 19, BinOp::Sum) // 29
//     .add_bin_op(29, 20, BinOp::Sum) // 30
//     .add_bin_op(30, 21, BinOp::Sum) // 31
//     .build();

//     let result = _run_freq_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);
//     // println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     //     1.0 / 0.4,
//     //     (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
//     //     (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
//     //     1.0/ 0.4,
//     //     (Lmv::Sample(s), Lmv::Decision(s, d)),
//     //     (Lmv::Decision(s, d), Lmv::Payback(s, p))
//     // );
//     // println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     // 1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
//     // let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// fn exp_fafrequentist_admission(n:i32, big:bool) {
//     let mut mc = util::markov_chain_admission_large();
//     if big {
//         mc = util::markov_chain_admission_large_costs();
//     }

//     let mapper = Amvm {};

//     let mut m = FaFrequentistBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0)) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0)) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0)) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(2.0)) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(3.0)) // 4
//     .add_freq(Amv::Sample(true), Amv::Cost(5.0)) // 5
//     .add_freq(Amv::Sample(true), Amv::Cost(8.0)) // 6
//     .add_freq(Amv::Sample(true), Amv::Cost(13.0)) // 7
//     .add_freq(Amv::Sample(true), Amv::Cost(21.0)) // 8
//     .add_freq(Amv::Sample(true), Amv::Cost(34.0)) // 9
//     .add_freq(Amv::Sample(true), Amv::Cost(55.0)) // 10
//     .add_constant(0.0) // 11
//     .add_constant(1.0) // 12
//     .add_constant(1.0) // 13
//     .add_constant(2.0) // 14
//     .add_constant(3.0) // 15
//     .add_constant(5.0) // 16
//     .add_constant(8.0) // 17
//     .add_constant(13.0) // 18
//     .add_constant(21.0) // 19
//     .add_constant(34.0) // 20
//     .add_constant(55.0) // 21
//     .add_bin_op(0, 11, BinOp::Prod) // 22
//     .add_bin_op(1, 12, BinOp::Prod) // 23
//     .add_bin_op(2, 13, BinOp::Prod) // 24
//     .add_bin_op(3, 14, BinOp::Prod) // 25
//     .add_bin_op(4, 15, BinOp::Prod) // 26
//     .add_bin_op(5, 16, BinOp::Prod) // 27
//     .add_bin_op(6, 17, BinOp::Prod) // 28
//     .add_bin_op(7, 18, BinOp::Prod) // 29
//     .add_bin_op(8, 19, BinOp::Prod) // 30
//     .add_bin_op(9, 20, BinOp::Prod) // 31
//     .add_bin_op(10, 21, BinOp::Prod) // 32

//     .add_bin_op(22, 23, BinOp::Sum) // 33
//     .add_bin_op(33, 24, BinOp::Sum) // 34
//     .add_bin_op(34, 25, BinOp::Sum) // 35
//     .add_bin_op(35, 26, BinOp::Sum) // 36
//     .add_bin_op(36, 27, BinOp::Sum) // 37
//     .add_bin_op(37, 28, BinOp::Sum) // 38
//     .add_bin_op(38, 29, BinOp::Sum) // 39
//     .add_bin_op(39, 30, BinOp::Sum) // 40
//     .add_bin_op(40, 31, BinOp::Sum) // 41
//     .add_bin_op(41, 32, BinOp::Sum) // 42
//     .set_delta(0.05)
//     .build();

//     let result = _run_fafreq_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);
//     // println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     //     1.0 / 0.4,
//     //     (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
//     //     (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
//     //     1.0/ 0.4,
//     //     (Lmv::Sample(s), Lmv::Decision(s, d)),
//     //     (Lmv::Decision(s, d), Lmv::Payback(s, p))
//     // );
//     // println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     // 1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
//     // let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// fn exp_bayesian_admission(n:i32, big:bool) {
//     let mut mc = util::markov_chain_admission_large();
//     if big {
//         mc = util::markov_chain_admission_large_costs();
//     }

    
//     let mapper = Amvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 61.0;

//     let mut exp_monitor =
//     BayesianBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0),num_vertices,1.0) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0),num_vertices,1.0) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0),num_vertices,1.0) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(2.0),num_vertices,1.0) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(3.0),num_vertices,1.0) // 4
//     .add_freq(Amv::Sample(true), Amv::Cost(5.0),num_vertices,1.0) // 5
//     .add_freq(Amv::Sample(true), Amv::Cost(8.0),num_vertices,1.0) // 6
//     .add_freq(Amv::Sample(true), Amv::Cost(13.0),num_vertices,1.0) // 7
//     .add_freq(Amv::Sample(true), Amv::Cost(21.0),num_vertices,1.0) // 8
//     .add_freq(Amv::Sample(true), Amv::Cost(34.0),num_vertices,1.0) // 9
//     .add_freq(Amv::Sample(true), Amv::Cost(55.0),num_vertices,1.0) // 10
//     .add_unary_op(0, 0.0, UnaryOp::Prod) // 11
//     .add_unary_op(1, 100.0, UnaryOp::Prod) // 12
//     .add_unary_op(2, 100.0, UnaryOp::Prod) // 13
//     .add_unary_op(3, 200.0, UnaryOp::Prod) // 14
//     .add_unary_op(4, 300.0, UnaryOp::Prod) // 15
//     .add_unary_op(5, 500.0, UnaryOp::Prod) // 16
//     .add_unary_op(6, 800.0, UnaryOp::Prod) // 17
//     .add_unary_op(7, 1300.0, UnaryOp::Prod) // 18
//     .add_unary_op(8, 2100.0, UnaryOp::Prod) // 19
//     .add_unary_op(9, 3400.0, UnaryOp::Prod) // 20
//     .add_unary_op(10, 5500.0, UnaryOp::Prod) // 21
//     .add_bin_op(11, 12, BinOp::Sum) // 22
//     .add_bin_op(22, 13, BinOp::Sum) // 23
//     .add_bin_op(23, 14, BinOp::Sum) // 24
//     .add_bin_op(24, 15, BinOp::Sum) // 25
//     .add_bin_op(25, 16, BinOp::Sum) // 26
//     .add_bin_op(26, 17, BinOp::Sum) // 27
//     .add_bin_op(27, 18, BinOp::Sum) // 28
//     .add_bin_op(28, 19, BinOp::Sum) // 29
//     .add_bin_op(29, 20, BinOp::Sum) // 30
//     .add_bin_op(30, 21, BinOp::Sum) // 31
//     .build();


//     let mut exp2_monitor =
//     // BayesianBuilder::<Lmv>::new()
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), num_vertices,1.0) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p), num_vertices,1.0) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d), num_vertices,1.0) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p), num_vertices,1.0) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 3, BinOp::Prod) // 8
//     // .add_bin_op(7, 7, BinOp::Prod) // 9
//     // .add_bin_op(3, 7, BinOp::Prod) // 10
//     // .add_unary_op(10, 2.0, UnaryOp::Prod) // 11
//     // .add_bin_op(8, 9, BinOp::Sum) // 12
//     // .add_bin_op(12, 11, BinOp::Subtract) // 12
//     // .build();
//     BayesianBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0),num_vertices,1.0) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0),num_vertices,1.0) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0),num_vertices,1.0) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(2.0),num_vertices,1.0) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(3.0),num_vertices,1.0) // 4
//     .add_freq(Amv::Sample(true), Amv::Cost(5.0),num_vertices,1.0) // 5
//     .add_freq(Amv::Sample(true), Amv::Cost(8.0),num_vertices,1.0) // 6
//     .add_freq(Amv::Sample(true), Amv::Cost(13.0),num_vertices,1.0) // 7
//     .add_freq(Amv::Sample(true), Amv::Cost(21.0),num_vertices,1.0) // 8
//     .add_freq(Amv::Sample(true), Amv::Cost(34.0),num_vertices,1.0) // 9
//     .add_freq(Amv::Sample(true), Amv::Cost(55.0),num_vertices,1.0) // 10
//     .add_unary_op(0, 0.0, UnaryOp::Prod) // 11
// .add_unary_op(1, 100.0, UnaryOp::Prod) // 12
// .add_unary_op(2, 100.0, UnaryOp::Prod) // 13
// .add_unary_op(3, 200.0, UnaryOp::Prod) // 14
// .add_unary_op(4, 300.0, UnaryOp::Prod) // 15
// .add_unary_op(5, 500.0, UnaryOp::Prod) // 16
// .add_unary_op(6, 800.0, UnaryOp::Prod) // 17
// .add_unary_op(7, 1300.0, UnaryOp::Prod) // 18
// .add_unary_op(8, 2100.0, UnaryOp::Prod) // 19
// .add_unary_op(9, 3400.0, UnaryOp::Prod) // 20
// .add_unary_op(10, 5500.0, UnaryOp::Prod) // 21
// .add_unary_op(11, 1.0, UnaryOp::Square) // 22
// .add_unary_op(12, 1.0, UnaryOp::Square) // 23
// .add_unary_op(13, 1.0, UnaryOp::Square) // 24
// .add_unary_op(14, 1.0, UnaryOp::Square) // 25
// .add_unary_op(15, 1.0, UnaryOp::Square) // 26
// .add_unary_op(16, 1.0, UnaryOp::Square) // 27
// .add_unary_op(17, 1.0, UnaryOp::Square) // 28
// .add_unary_op(18, 1.0, UnaryOp::Square) // 29
// .add_unary_op(19, 1.0, UnaryOp::Square) // 30
// .add_unary_op(20, 1.0, UnaryOp::Square) // 31
// .add_unary_op(21, 1.0, UnaryOp::Square) // 32
// .add_bin_op(11, 12, BinOp::Prod) // 33
// .add_bin_op(11, 13, BinOp::Prod) // 34
// .add_bin_op(11, 14, BinOp::Prod) // 35
// .add_bin_op(11, 15, BinOp::Prod) // 36
// .add_bin_op(11, 16, BinOp::Prod) // 37
// .add_bin_op(11, 17, BinOp::Prod) // 38
// .add_bin_op(11, 18, BinOp::Prod) // 39
// .add_bin_op(11, 19, BinOp::Prod) // 40
// .add_bin_op(11, 20, BinOp::Prod) // 41
// .add_bin_op(11, 21, BinOp::Prod) // 42
// .add_bin_op(12, 13, BinOp::Prod) // 43
// .add_bin_op(12, 14, BinOp::Prod) // 44
// .add_bin_op(12, 15, BinOp::Prod) // 45
// .add_bin_op(12, 16, BinOp::Prod) // 46
// .add_bin_op(12, 17, BinOp::Prod) // 47
// .add_bin_op(12, 18, BinOp::Prod) // 48
// .add_bin_op(12, 19, BinOp::Prod) // 49
// .add_bin_op(12, 20, BinOp::Prod) // 50
// .add_bin_op(12, 21, BinOp::Prod) // 51
// .add_bin_op(13, 14, BinOp::Prod) // 52
// .add_bin_op(13, 15, BinOp::Prod) // 53
// .add_bin_op(13, 16, BinOp::Prod) // 54
// .add_bin_op(13, 17, BinOp::Prod) // 55
// .add_bin_op(13, 18, BinOp::Prod) // 56
// .add_bin_op(13, 19, BinOp::Prod) // 57
// .add_bin_op(13, 20, BinOp::Prod) // 58
// .add_bin_op(13, 21, BinOp::Prod) // 59
// .add_bin_op(14, 15, BinOp::Prod) // 60
// .add_bin_op(14, 16, BinOp::Prod) // 61
// .add_bin_op(14, 17, BinOp::Prod) // 62
// .add_bin_op(14, 18, BinOp::Prod) // 63
// .add_bin_op(14, 19, BinOp::Prod) // 64
// .add_bin_op(14, 20, BinOp::Prod) // 65
// .add_bin_op(14, 21, BinOp::Prod) // 66
// .add_bin_op(15, 16, BinOp::Prod) // 67
// .add_bin_op(15, 17, BinOp::Prod) // 68
// .add_bin_op(15, 18, BinOp::Prod) // 69
// .add_bin_op(15, 19, BinOp::Prod) // 70
// .add_bin_op(15, 20, BinOp::Prod) // 71
// .add_bin_op(15, 21, BinOp::Prod) // 72
// .add_bin_op(16, 17, BinOp::Prod) // 73
// .add_bin_op(16, 18, BinOp::Prod) // 74
// .add_bin_op(16, 19, BinOp::Prod) // 75
// .add_bin_op(16, 20, BinOp::Prod) // 76
// .add_bin_op(16, 21, BinOp::Prod) // 77
// .add_bin_op(17, 18, BinOp::Prod) // 78
// .add_bin_op(17, 19, BinOp::Prod) // 79
// .add_bin_op(17, 20, BinOp::Prod) // 80
// .add_bin_op(17, 21, BinOp::Prod) // 81
// .add_bin_op(18, 19, BinOp::Prod) // 82
// .add_bin_op(18, 20, BinOp::Prod) // 83
// .add_bin_op(18, 21, BinOp::Prod) // 84
// .add_bin_op(19, 20, BinOp::Prod) // 85
// .add_bin_op(19, 21, BinOp::Prod) // 86
// .add_bin_op(20, 21, BinOp::Prod) // 87
// .add_unary_op(33, 2.0, UnaryOp::Prod) // 88
// .add_unary_op(34, 2.0, UnaryOp::Prod) // 89
// .add_unary_op(35, 2.0, UnaryOp::Prod) // 90
// .add_unary_op(36, 2.0, UnaryOp::Prod) // 91
// .add_unary_op(37, 2.0, UnaryOp::Prod) // 92
// .add_unary_op(38, 2.0, UnaryOp::Prod) // 93
// .add_unary_op(39, 2.0, UnaryOp::Prod) // 94
// .add_unary_op(40, 2.0, UnaryOp::Prod) // 95
// .add_unary_op(41, 2.0, UnaryOp::Prod) // 96
// .add_unary_op(42, 2.0, UnaryOp::Prod) // 97
// .add_unary_op(43, 2.0, UnaryOp::Prod) // 98
// .add_unary_op(44, 2.0, UnaryOp::Prod) // 99
// .add_unary_op(45, 2.0, UnaryOp::Prod) // 100
// .add_unary_op(46, 2.0, UnaryOp::Prod) // 101
// .add_unary_op(47, 2.0, UnaryOp::Prod) // 102
// .add_unary_op(48, 2.0, UnaryOp::Prod) // 103
// .add_unary_op(49, 2.0, UnaryOp::Prod) // 104
// .add_unary_op(50, 2.0, UnaryOp::Prod) // 105
// .add_unary_op(51, 2.0, UnaryOp::Prod) // 106
// .add_unary_op(52, 2.0, UnaryOp::Prod) // 107
// .add_unary_op(53, 2.0, UnaryOp::Prod) // 108
// .add_unary_op(54, 2.0, UnaryOp::Prod) // 109
// .add_unary_op(55, 2.0, UnaryOp::Prod) // 110
// .add_unary_op(56, 2.0, UnaryOp::Prod) // 111
// .add_unary_op(57, 2.0, UnaryOp::Prod) // 112
// .add_unary_op(58, 2.0, UnaryOp::Prod) // 113
// .add_unary_op(59, 2.0, UnaryOp::Prod) // 114
// .add_unary_op(60, 2.0, UnaryOp::Prod) // 115
// .add_unary_op(61, 2.0, UnaryOp::Prod) // 116
// .add_unary_op(62, 2.0, UnaryOp::Prod) // 117
// .add_unary_op(63, 2.0, UnaryOp::Prod) // 118
// .add_unary_op(64, 2.0, UnaryOp::Prod) // 119
// .add_unary_op(65, 2.0, UnaryOp::Prod) // 120
// .add_unary_op(66, 2.0, UnaryOp::Prod) // 121
// .add_unary_op(67, 2.0, UnaryOp::Prod) // 122
// .add_unary_op(68, 2.0, UnaryOp::Prod) // 123
// .add_unary_op(69, 2.0, UnaryOp::Prod) // 124
// .add_unary_op(70, 2.0, UnaryOp::Prod) // 125
// .add_unary_op(71, 2.0, UnaryOp::Prod) // 126
// .add_unary_op(72, 2.0, UnaryOp::Prod) // 127
// .add_unary_op(73, 2.0, UnaryOp::Prod) // 128
// .add_unary_op(74, 2.0, UnaryOp::Prod) // 129
// .add_unary_op(75, 2.0, UnaryOp::Prod) // 130
// .add_unary_op(76, 2.0, UnaryOp::Prod) // 131
// .add_unary_op(77, 2.0, UnaryOp::Prod) // 132
// .add_unary_op(78, 2.0, UnaryOp::Prod) // 133
// .add_unary_op(79, 2.0, UnaryOp::Prod) // 134
// .add_unary_op(80, 2.0, UnaryOp::Prod) // 135
// .add_unary_op(81, 2.0, UnaryOp::Prod) // 136
// .add_unary_op(82, 2.0, UnaryOp::Prod) // 137
// .add_unary_op(83, 2.0, UnaryOp::Prod) // 138
// .add_unary_op(84, 2.0, UnaryOp::Prod) // 139
// .add_unary_op(85, 2.0, UnaryOp::Prod) // 140
// .add_unary_op(86, 2.0, UnaryOp::Prod) // 141
// .add_unary_op(87, 2.0, UnaryOp::Prod) // 142
// .add_bin_op(22, 23, BinOp::Sum) // 143
// .add_bin_op(143, 24, BinOp::Sum) // 144
// .add_bin_op(144, 25, BinOp::Sum) // 145
// .add_bin_op(145, 26, BinOp::Sum) // 146
// .add_bin_op(146, 27, BinOp::Sum) // 147
// .add_bin_op(147, 28, BinOp::Sum) // 148
// .add_bin_op(148, 29, BinOp::Sum) // 149
// .add_bin_op(149, 30, BinOp::Sum) // 150
// .add_bin_op(150, 31, BinOp::Sum) // 151
// .add_bin_op(151, 32, BinOp::Sum) // 152
// .add_bin_op(152, 88, BinOp::Sum) // 153
// .add_bin_op(153, 89, BinOp::Sum) // 154
// .add_bin_op(154, 90, BinOp::Sum) // 155
// .add_bin_op(155, 91, BinOp::Sum) // 156
// .add_bin_op(156, 92, BinOp::Sum) // 157
// .add_bin_op(157, 93, BinOp::Sum) // 158
// .add_bin_op(158, 94, BinOp::Sum) // 159
// .add_bin_op(159, 95, BinOp::Sum) // 160
// .add_bin_op(160, 96, BinOp::Sum) // 161
// .add_bin_op(161, 97, BinOp::Sum) // 162
// .add_bin_op(162, 98, BinOp::Sum) // 163
// .add_bin_op(163, 99, BinOp::Sum) // 164
// .add_bin_op(164, 100, BinOp::Sum) // 165
// .add_bin_op(165, 101, BinOp::Sum) // 166
// .add_bin_op(166, 102, BinOp::Sum) // 167
// .add_bin_op(167, 103, BinOp::Sum) // 168
// .add_bin_op(168, 104, BinOp::Sum) // 169
// .add_bin_op(169, 105, BinOp::Sum) // 170
// .add_bin_op(170, 106, BinOp::Sum) // 171
// .add_bin_op(171, 107, BinOp::Sum) // 172
// .add_bin_op(172, 108, BinOp::Sum) // 173
// .add_bin_op(173, 109, BinOp::Sum) // 174
// .add_bin_op(174, 110, BinOp::Sum) // 175
// .add_bin_op(175, 111, BinOp::Sum) // 176
// .add_bin_op(176, 112, BinOp::Sum) // 177
// .add_bin_op(177, 113, BinOp::Sum) // 178
// .add_bin_op(178, 114, BinOp::Sum) // 179
// .add_bin_op(179, 115, BinOp::Sum) // 180
// .add_bin_op(180, 116, BinOp::Sum) // 181
// .add_bin_op(181, 117, BinOp::Sum) // 182
// .add_bin_op(182, 118, BinOp::Sum) // 183
// .add_bin_op(183, 119, BinOp::Sum) // 184
// .add_bin_op(184, 120, BinOp::Sum) // 185
// .add_bin_op(185, 121, BinOp::Sum) // 186
// .add_bin_op(186, 122, BinOp::Sum) // 187
// .add_bin_op(187, 123, BinOp::Sum) // 188
// .add_bin_op(188, 124, BinOp::Sum) // 189
// .add_bin_op(189, 125, BinOp::Sum) // 190
// .add_bin_op(190, 126, BinOp::Sum) // 191
// .add_bin_op(191, 127, BinOp::Sum) // 192
// .add_bin_op(192, 128, BinOp::Sum) // 193
// .add_bin_op(193, 129, BinOp::Sum) // 194
// .add_bin_op(194, 130, BinOp::Sum) // 195
// .add_bin_op(195, 131, BinOp::Sum) // 196
// .add_bin_op(196, 132, BinOp::Sum) // 197
// .add_bin_op(197, 133, BinOp::Sum) // 198
// .add_bin_op(198, 134, BinOp::Sum) // 199
// .add_bin_op(199, 135, BinOp::Sum) // 200
// .add_bin_op(200, 136, BinOp::Sum) // 201
// .add_bin_op(201, 137, BinOp::Sum) // 202
// .add_bin_op(202, 138, BinOp::Sum) // 203
// .add_bin_op(203, 139, BinOp::Sum) // 204
// .add_bin_op(204, 140, BinOp::Sum) // 205
// .add_bin_op(205, 141, BinOp::Sum) // 206
// .add_bin_op(206, 142, BinOp::Sum) // 207
//     // .add_unary_op(0, 0.0, UnaryOp::Prod) // 11
//     // .add_unary_op(1, 100.0, UnaryOp::Prod) // 12
//     // .add_unary_op(2, 100.0, UnaryOp::Prod) // 13
//     // .add_unary_op(3, 200.0, UnaryOp::Prod) // 14
//     // .add_unary_op(4, 300.0, UnaryOp::Prod) // 15
//     // .add_unary_op(5, 500.0, UnaryOp::Prod) // 16
//     // .add_unary_op(6, 800.0, UnaryOp::Prod) // 17
//     // .add_unary_op(7, 1300.0, UnaryOp::Prod) // 18
//     // .add_unary_op(8, 2100.0, UnaryOp::Prod) // 19
//     // .add_unary_op(9, 3400.0, UnaryOp::Prod) // 20
//     // .add_unary_op(10, 5500.0, UnaryOp::Prod) // 21
//     // .add_unary_op(11, 1.0, UnaryOp::Square) // 22
//     // .add_unary_op(12, 1.0, UnaryOp::Square) // 23
//     // .add_unary_op(13, 1.0, UnaryOp::Square) // 24
//     // .add_unary_op(14, 1.0, UnaryOp::Square) // 25
//     // .add_unary_op(15, 1.0, UnaryOp::Square) // 26
//     // .add_unary_op(16, 1.0, UnaryOp::Square) // 27
//     // .add_unary_op(17, 1.0, UnaryOp::Square) // 28
//     // .add_unary_op(18, 1.0, UnaryOp::Square) // 29
//     // .add_unary_op(19, 1.0, UnaryOp::Square) // 30
//     // .add_unary_op(20, 1.0, UnaryOp::Square) // 31
//     // .add_unary_op(21, 1.0, UnaryOp::Square) // 32
//     // .add_bin_op(11, 12, BinOp::Prod) // 33   
//     // .add_bin_op(11, 13, BinOp::Prod) // 34   
//     // .add_bin_op(11, 14, BinOp::Prod) // 35   
//     // .add_bin_op(11, 15, BinOp::Prod) // 36   
//     // .add_bin_op(11, 16, BinOp::Prod) // 37   
//     // .add_bin_op(11, 17, BinOp::Prod) // 38   
//     // .add_bin_op(11, 18, BinOp::Prod) // 39   
//     // .add_bin_op(11, 19, BinOp::Prod) // 40        
//     // .add_bin_op(11, 20, BinOp::Prod) // 41   
//     // .add_bin_op(11, 21, BinOp::Prod) // 42  
//     // .add_bin_op(12, 13, BinOp::Prod) // 43   
//     // .add_bin_op(12, 14, BinOp::Prod) // 44   !!!! 
//     // .add_bin_op(12, 15, BinOp::Prod) // 45   
//     // .add_bin_op(12, 16, BinOp::Prod) // 46   
//     // .add_bin_op(12, 17, BinOp::Prod) // 47   
//     // .add_bin_op(12, 18, BinOp::Prod) // 48   
//     // .add_bin_op(12, 19, BinOp::Prod) // 49   
//     // .add_bin_op(12, 20, BinOp::Prod) // 50        
//     // .add_bin_op(12, 21, BinOp::Prod) // 51   
//     // .add_bin_op(13, 14, BinOp::Prod) // 52   
//     // .add_bin_op(13, 15, BinOp::Prod) // 53   
//     // .add_bin_op(13, 16, BinOp::Prod) // 54   
//     // .add_bin_op(13, 17, BinOp::Prod) // 55   
//     // .add_bin_op(13, 18, BinOp::Prod) // 56   
//     // .add_bin_op(13, 19, BinOp::Prod) // 57   
//     // .add_bin_op(13, 20, BinOp::Prod) // 58        
//     // .add_bin_op(13, 21, BinOp::Prod) // 59     
//     // .add_bin_op(14, 15, BinOp::Prod) // 60   
//     // .add_bin_op(14, 16, BinOp::Prod) // 61   
//     // .add_bin_op(14, 17, BinOp::Prod) // 62   
//     // .add_bin_op(14, 18, BinOp::Prod) // 63   
//     // .add_bin_op(14, 19, BinOp::Prod) // 64   
//     // .add_bin_op(14, 20, BinOp::Prod) // 65        
//     // .add_bin_op(14, 21, BinOp::Prod) // 66  
//     // .add_bin_op(15, 16, BinOp::Prod) // 67   
//     // .add_bin_op(15, 17, BinOp::Prod) // 68   
//     // .add_bin_op(15, 18, BinOp::Prod) // 69   
//     // .add_bin_op(15, 19, BinOp::Prod) // 70   
//     // .add_bin_op(15, 20, BinOp::Prod) // 71        
//     // .add_bin_op(15, 21, BinOp::Prod) // 72 
//     // .add_bin_op(16, 17, BinOp::Prod) // 73   
//     // .add_bin_op(16, 18, BinOp::Prod) // 74   
//     // .add_bin_op(16, 19, BinOp::Prod) // 75   
//     // .add_bin_op(16, 20, BinOp::Prod) // 76        
//     // .add_bin_op(16, 21, BinOp::Prod) // 77   
//     // .add_bin_op(17, 18, BinOp::Prod) // 78   
//     // .add_bin_op(17, 19, BinOp::Prod) // 79   
//     // .add_bin_op(17, 20, BinOp::Prod) // 80        
//     // .add_bin_op(17, 21, BinOp::Prod) // 81   
//     // .add_bin_op(18, 19, BinOp::Prod) // 82   
//     // .add_bin_op(18, 20, BinOp::Prod) // 83        
//     // .add_bin_op(18, 21, BinOp::Prod) // 84   
//     // .add_bin_op(19, 20, BinOp::Prod) // 85        
//     // .add_bin_op(19, 21, BinOp::Prod) // 86   
//     // .add_bin_op(20, 21, BinOp::Prod) // 87   
//     // .add_unary_op(33, 2.0, UnaryOp::Prod) // 88
//     // .add_unary_op(34, 2.0, UnaryOp::Prod) // 89
//     // .add_unary_op(35, 2.0, UnaryOp::Prod) // 90
//     // .add_unary_op(36, 2.0, UnaryOp::Prod) // 91
//     // .add_unary_op(37, 2.0, UnaryOp::Prod) // 92
//     // .add_unary_op(38, 2.0, UnaryOp::Prod) // 93
//     // .add_unary_op(39, 2.0, UnaryOp::Prod) // 94
//     // .add_unary_op(40, 2.0, UnaryOp::Prod) // 95
//     // .add_unary_op(41, 2.0, UnaryOp::Prod) // 96
//     // .add_unary_op(42, 2.0, UnaryOp::Prod) // 97
//     // .add_unary_op(43, 2.0, UnaryOp::Prod) // 98
//     // .add_unary_op(44, 2.0, UnaryOp::Prod) // 99
//     // .add_unary_op(45, 2.0, UnaryOp::Prod) // 100 !!!!!
//     // .add_unary_op(46, 2.0, UnaryOp::Prod) // 101
//     // .add_unary_op(47, 2.0, UnaryOp::Prod) // 102
//     // .add_unary_op(48, 2.0, UnaryOp::Prod) // 103
//     // .add_unary_op(49, 2.0, UnaryOp::Prod) // 104
//     // .add_unary_op(50, 2.0, UnaryOp::Prod) // 105
//     // .add_unary_op(51, 2.0, UnaryOp::Prod) // 106
//     // .add_unary_op(52, 2.0, UnaryOp::Prod) // 107
//     // .add_unary_op(53, 2.0, UnaryOp::Prod) // 108
//     // .add_unary_op(54, 2.0, UnaryOp::Prod) // 109
//     // .add_unary_op(55, 2.0, UnaryOp::Prod) // 110
//     // .add_unary_op(56, 2.0, UnaryOp::Prod) // 111
//     // .add_unary_op(57, 2.0, UnaryOp::Prod) // 112
//     // .add_unary_op(58, 2.0, UnaryOp::Prod) // 113
//     // .add_unary_op(59, 2.0, UnaryOp::Prod) // 114
//     // .add_unary_op(60, 2.0, UnaryOp::Prod) // 115
//     // .add_unary_op(61, 2.0, UnaryOp::Prod) // 116
//     // .add_unary_op(62, 2.0, UnaryOp::Prod) // 117
//     // .add_unary_op(63, 2.0, UnaryOp::Prod) // 118
//     // .add_unary_op(64, 2.0, UnaryOp::Prod) // 119
//     // .add_unary_op(65, 2.0, UnaryOp::Prod) // 120
//     // .add_unary_op(66, 2.0, UnaryOp::Prod) // 121
//     // .add_unary_op(67, 2.0, UnaryOp::Prod) // 122
//     // .add_unary_op(68, 2.0, UnaryOp::Prod) // 123
//     // .add_unary_op(69, 2.0, UnaryOp::Prod) // 124
//     // .add_unary_op(70, 2.0, UnaryOp::Prod) // 125
//     // .add_unary_op(71, 2.0, UnaryOp::Prod) // 126
//     // .add_unary_op(72, 2.0, UnaryOp::Prod) // 127
//     // .add_unary_op(73, 2.0, UnaryOp::Prod) // 128
//     // .add_unary_op(74, 2.0, UnaryOp::Prod) // 129
//     // .add_unary_op(75, 2.0, UnaryOp::Prod) // 130
//     // .add_unary_op(76, 2.0, UnaryOp::Prod) // 131
//     // .add_unary_op(77, 2.0, UnaryOp::Prod) // 132
//     // .add_unary_op(78, 2.0, UnaryOp::Prod) // 133
//     // .add_unary_op(79, 2.0, UnaryOp::Prod) // 134
//     // .add_unary_op(80, 2.0, UnaryOp::Prod) // 135
//     // .add_unary_op(81, 2.0, UnaryOp::Prod) // 136
//     // .add_unary_op(82, 2.0, UnaryOp::Prod) // 137
//     // .add_unary_op(83, 2.0, UnaryOp::Prod) // 138
//     // .add_unary_op(84, 2.0, UnaryOp::Prod) // 139
//     // .add_unary_op(85, 2.0, UnaryOp::Prod) // 140
//     // .add_unary_op(86, 2.0, UnaryOp::Prod) // 141 
//     // .add_unary_op(87, 2.0, UnaryOp::Prod) // 142
//     // .add_unary_op(88, 2.0, UnaryOp::Prod) // 143
//     // .add_bin_op(22, 23, BinOp::Sum) // 144
//     // .add_bin_op(144, 24, BinOp::Sum) // 145
//     // .add_bin_op(145, 25, BinOp::Sum) // 146
//     // .add_bin_op(146, 26, BinOp::Sum) // 147
//     // .add_bin_op(147, 27, BinOp::Sum) // 148
//     // .add_bin_op(148, 28, BinOp::Sum) // 149
//     // .add_bin_op(149, 29, BinOp::Sum) // 151
//     // .add_bin_op(150, 30, BinOp::Sum) // 152
//     // .add_bin_op(151, 31, BinOp::Sum) // 153
//     // .add_bin_op(152, 32, BinOp::Sum) // 154
//     // .add_bin_op(153, 33, BinOp::Sum) // 155
//     // .add_bin_op(154, 88, BinOp::Sum) // 155
//     // .add_bin_op(155, 89, BinOp::Sum) // 156
//     // .add_bin_op(156, 90, BinOp::Sum) // 157
//     // .add_bin_op(157, 91, BinOp::Sum) // 158
//     // .add_bin_op(158, 92, BinOp::Sum) // 159
//     // .add_bin_op(159, 93, BinOp::Sum) // 160
//     // .add_bin_op(160, 94, BinOp::Sum) // 161
//     // .add_bin_op(161, 95, BinOp::Sum) // 162
//     // .add_bin_op(162, 96, BinOp::Sum) // 163
//     // .add_bin_op(163, 97, BinOp::Sum) // 164
//     // .add_bin_op(164, 98, BinOp::Sum) // 165
//     // .add_bin_op(165, 99, BinOp::Sum) // 166
//     // .add_bin_op(166, 100, BinOp::Sum) // 167
//     // .add_bin_op(167, 101, BinOp::Sum) // 168
//     // .add_bin_op(168, 102, BinOp::Sum) // 169
//     // .add_bin_op(169, 103, BinOp::Sum) // 170
//     // .add_bin_op(170, 104, BinOp::Sum) // 171
//     // .add_bin_op(171, 105, BinOp::Sum) // 172
//     // .add_bin_op(172, 106, BinOp::Sum) // 173
//     // .add_bin_op(173, 107, BinOp::Sum) // 174
//     // .add_bin_op(174, 108, BinOp::Sum) // 175
//     // .add_bin_op(175, 109, BinOp::Sum) // 176
//     // .add_bin_op(176, 110, BinOp::Sum) // 177
//     // .add_bin_op(177, 111, BinOp::Sum) // 178
//     // .add_bin_op(178, 112, BinOp::Sum) // 179
//     // .add_bin_op(179, 113, BinOp::Sum) // 180
//     // .add_bin_op(180, 114, BinOp::Sum) // 181
//     // .add_bin_op(181, 115, BinOp::Sum) // 182
//     // .add_bin_op(182, 116, BinOp::Sum) // 183
//     // .add_bin_op(183, 117, BinOp::Sum) // 184
//     // .add_bin_op(184, 118, BinOp::Sum) // 185
//     // .add_bin_op(185, 119, BinOp::Sum) // 186
//     // .add_bin_op(186, 120, BinOp::Sum) // 187
//     // .add_bin_op(187, 121, BinOp::Sum) // 188
//     // .add_bin_op(188, 122, BinOp::Sum) // 189
//     // .add_bin_op(189, 123, BinOp::Sum) // 190
//     // .add_bin_op(190, 124, BinOp::Sum) // 191
//     // .add_bin_op(191, 125, BinOp::Sum) // 192
//     // .add_bin_op(192, 126, BinOp::Sum) // 193
//     // .add_bin_op(193, 127, BinOp::Sum) // 194
//     // .add_bin_op(194, 128, BinOp::Sum) // 195
//     // .add_bin_op(195, 129, BinOp::Sum) // 196
//     // .add_bin_op(196, 130, BinOp::Sum) // 197
//     // .add_bin_op(197, 131, BinOp::Sum) // 198
//     // .add_bin_op(198, 132, BinOp::Sum) // 199
//     // .add_bin_op(199, 133, BinOp::Sum) // 200
//     // .add_bin_op(200, 134, BinOp::Sum) // 201
//     // .add_bin_op(201, 135, BinOp::Sum) // 202
//     // .add_bin_op(202, 136, BinOp::Sum) // 203
//     // .add_bin_op(203, 137, BinOp::Sum) // 204
//     // .add_bin_op(204, 138, BinOp::Sum) // 205
//     // .add_bin_op(205, 139, BinOp::Sum) // 206
//     // .add_bin_op(206, 140, BinOp::Sum) // 207
//     // .add_bin_op(207, 141, BinOp::Sum) // 208
//     // .add_bin_op(208, 142, BinOp::Sum) // 209
//     // .add_bin_op(209, 143, BinOp::Sum) // 210
//     // // .add_bin_op(210, 144, BinOp::Sum) // 211
//     .build();

//     let mut m = CBayesian::<Amv> {
//         delta, exp_monitor, exp2_monitor
//     };

//     let result = _run_bayes_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }




// fn exp_frequentist_admission_small_lv(n:i32, big:bool, cost:Vec<f64>) {
//     let mut mc = util::markov_chain_admission_small_lv();
//     if big {
//         mc = util::markov_chain_admission_small_lv();
//     }
//     let mapper = Amvm {};

//     let mut m = FrequentistBuilder::<Amv>::new()
//     .set_delta(0.05)
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0)) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(2000.0)) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(20000.0)) // 2
//     .add_unary_op(0, cost[0], UnaryOp::Prod) // 3
//     .add_unary_op(1, cost[1], UnaryOp::Prod) // 4
//     .add_unary_op(2, cost[2], UnaryOp::Prod) // 5
//     .add_bin_op(3, 4, BinOp::Sum) // 6
//     .add_bin_op(6, 5, BinOp::Sum) // 7
//     .build();

//     let result = _run_freq_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);
//     // println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     //     1.0 / 0.4,
//     //     (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
//     //     (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
//     //     1.0/ 0.4,
//     //     (Lmv::Sample(s), Lmv::Decision(s, d)),
//     //     (Lmv::Decision(s, d), Lmv::Payback(s, p))
//     // );
//     // println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     // 1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
//     // let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// fn exp_fafrequentist_admission_small_lv(n:i32, big:bool) {
//     let mut mc = util::markov_chain_admission_small_lv();
//     if big {
//         mc = util::markov_chain_admission_small_lv();
//     }

//     let mapper = Amvm {};

//     let mut m = FaFrequentistBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0)) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(2000.0)) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(20000.0)) // 2
//     .add_constant(0.0) // 3
//     .add_constant(2000.0) // 4
//     .add_constant(20000.0) // 5
//     .add_bin_op(0, 3, BinOp::Prod) // 6
//     .add_bin_op(1, 4, BinOp::Prod) // 7
//     .add_bin_op(2, 5, BinOp::Prod) // 8
//     .add_bin_op(6, 7, BinOp::Sum) // 9
//     .add_bin_op(9, 8, BinOp::Sum) // 10
//     .set_delta(0.05)
//     .build();

//     let result = _run_fafreq_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);
//     // println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     //     1.0 / 0.4,
//     //     (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
//     //     (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
//     //     1.0/ 0.4,
//     //     (Lmv::Sample(s), Lmv::Decision(s, d)),
//     //     (Lmv::Decision(s, d), Lmv::Payback(s, p))
//     // );
//     // println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     // 1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
//     // let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// // fn exp_frequentist_admission_small_lv(n:i32, big:bool, cost:Vec<f64>) {
// //     let mut mc = util::markov_chain_admission_small_lv();
// //     if big {
// //         mc = util::markov_chain_admission_small_lv();
// //     }
// //     let mapper = Amvm {};

// //     let mut m = FrequentistBuilder::<Amv>::new()
// //     .set_delta(0.05)
// //     .add_freq(Amv::Sample(true), Amv::Cost(0.0)) // 0
// //     .add_freq(Amv::Sample(true), Amv::Cost(2000.0)) // 1
// //     .add_freq(Amv::Sample(true), Amv::Cost(20000.0)) // 2
// //     .add_unary_op(0, cost[0], UnaryOp::Prod) // 3
// //     .add_unary_op(1, cost[1], UnaryOp::Prod) // 4
// //     .add_unary_op(2, cost[2], UnaryOp::Prod) // 5
// //     .add_bin_op(3, 4, BinOp::Sum) // 6
// //     .add_bin_op(6, 5, BinOp::Sum) // 7
// //     .build();

// //     let result = _run_freq_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);
// //     // println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
// //     //     1.0 / 0.4,
// //     //     (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
// //     //     (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
// //     //     1.0/ 0.4,
// //     //     (Lmv::Sample(s), Lmv::Decision(s, d)),
// //     //     (Lmv::Decision(s, d), Lmv::Payback(s, p))
// //     // );
// //     // println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
// //     // 1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
// //     // let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// // }

// fn exp_bayesian_admission_small_lv(n:i32, big:bool,cost:Vec<f64>) {
//     let mut mc = util::markov_chain_admission_small_lv();
//     if big {
//         mc = util::markov_chain_admission_small_lv();
//     }

    
//     let mapper = Amvm {};
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 19.0;

//     let mut exp_monitor =
//     BayesianBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0),num_vertices,1.0) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(2000.0),num_vertices,1.0) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(20000.0),num_vertices,1.0) // 2
//     .add_unary_op(0, cost[0], UnaryOp::Prod) // 3
//     .add_unary_op(1, cost[1], UnaryOp::Prod) // 4
//     .add_unary_op(2, cost[2], UnaryOp::Prod) // 5
//     .add_bin_op(3, 4, BinOp::Sum) // 6
//     .add_bin_op(6, 5, BinOp::Sum) // 7
//     .build();


//     let mut exp2_monitor =
//     BayesianBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0),num_vertices,1.0) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(2000.0),num_vertices,1.0) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(20000.0),num_vertices,1.0) // 2
//     .add_unary_op(0, 1.0, UnaryOp::Square) // 3
//     .add_unary_op(1, 1.0, UnaryOp::Square) // 4
//     .add_unary_op(2, 1.0, UnaryOp::Square) // 5
//     .add_bin_op(0, 1, BinOp::Prod) // 6
//     .add_bin_op(0, 2, BinOp::Prod) // 7
//     .add_bin_op(1, 2, BinOp::Prod) // 8
//     .add_unary_op(3, cost[0]*cost[0], UnaryOp::Prod) // 9
//     .add_unary_op(4, cost[1]*cost[1], UnaryOp::Prod) // 10
//     .add_unary_op(5, cost[2]*cost[2], UnaryOp::Prod) // 11
//     .add_unary_op(6, 2.0*cost[0]*cost[1], UnaryOp::Prod) // 12
//     .add_unary_op(7, 2.0*cost[0]*cost[2], UnaryOp::Prod) // 13
//     .add_unary_op(8, 2.0*cost[1]*cost[2], UnaryOp::Prod) // 14
//     .add_bin_op(9, 10, BinOp::Sum) // 15
//     .add_bin_op(15, 11, BinOp::Sum) // 16
//     .add_bin_op(16, 12, BinOp::Sum) // 17
//     .add_bin_op(17, 13, BinOp::Sum) // 18
//     .add_bin_op(18, 14, BinOp::Sum) // 19
//     .build();

//     let mut m = CBayesian::<Amv> {
//         delta, exp_monitor, exp2_monitor
//     };

//     let result = _run_bayes_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }




// fn exp_frequentist_admission_medium_lv(n:i32, big:bool, cost:Vec<f64>) {
//     let mut mc = util::markov_chain_admission_medium();
//     if big {
//         mc = util::markov_chain_admission_medium();
//     }
//     let mapper = Amvm {};

//     let mut m = FrequentistBuilder::<Amv>::new()
//     .set_delta(0.05)
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0)) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0)) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(2.0)) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(3.0)) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(4.0)) // 4
//     .add_unary_op(0, cost[0], UnaryOp::Prod) // 5
//     .add_unary_op(1, cost[1], UnaryOp::Prod) // 6
//     .add_unary_op(3, cost[2], UnaryOp::Prod) // 7
//     .add_unary_op(4, cost[3], UnaryOp::Prod) // 8
//     .add_unary_op(5, cost[4], UnaryOp::Prod) // 9
//     .add_bin_op(5, 6, BinOp::Sum) // 10
//     .add_bin_op(10, 7, BinOp::Sum) // 11
//     .add_bin_op(11, 8, BinOp::Sum) // 12
//     .add_bin_op(12, 9, BinOp::Sum) // 13
//     .build();

//     let result = _run_freq_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);
//     // println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     //     1.0 / 0.4,
//     //     (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
//     //     (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
//     //     1.0/ 0.4,
//     //     (Lmv::Sample(s), Lmv::Decision(s, d)),
//     //     (Lmv::Decision(s, d), Lmv::Payback(s, p))
//     // );
//     // println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     // 1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
//     // let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }


// fn exp_bayesian_admission_medium_lv(n:i32, big:bool,cost:Vec<f64>) {
//     let mut mc = util::markov_chain_admission_medium();
//     if big {
//         mc = util::markov_chain_admission_medium();
//     }

    
//     let mapper = Amvm {};
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 31.0;

//     let mut exp_monitor =
//     BayesianBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0), num_vertices, 1.0) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0), num_vertices, 1.0) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(2.0), num_vertices, 1.0) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(3.0), num_vertices, 1.0) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(4.0), num_vertices, 1.0) // 4
//     .add_unary_op(0, cost[0], UnaryOp::Prod) // 5
//     .add_unary_op(1, cost[1], UnaryOp::Prod) // 6
//     .add_unary_op(3, cost[2], UnaryOp::Prod) // 7
//     .add_unary_op(4, cost[3], UnaryOp::Prod) // 8
//     .add_unary_op(5, cost[4], UnaryOp::Prod) // 9
//     .add_bin_op(5, 6, BinOp::Sum) // 10
//     .add_bin_op(10, 7, BinOp::Sum) // 11
//     .add_bin_op(11, 8, BinOp::Sum) // 12
//     .add_bin_op(12, 9, BinOp::Sum) // 13
//     .build();


//     let mut exp2_monitor =
//     BayesianBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0), num_vertices, 1.0) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1.0), num_vertices, 1.0) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(2.0), num_vertices, 1.0) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(3.0), num_vertices, 1.0) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(4.0), num_vertices, 1.0) // 4
//     .add_unary_op(0, 1.0, UnaryOp::Square) // 5
//     .add_unary_op(1, 1.0, UnaryOp::Square) // 6
//     .add_unary_op(2, 1.0, UnaryOp::Square) // 7
//     .add_unary_op(3, 1.0, UnaryOp::Square) // 8
//     .add_unary_op(4, 1.0, UnaryOp::Square) // 9
//     .add_bin_op(0, 1, BinOp::Prod) // 10
//     .add_bin_op(0, 2, BinOp::Prod) // 11
//     .add_bin_op(0, 3, BinOp::Prod) // 12
//     .add_bin_op(0, 4, BinOp::Prod) // 13
//     .add_bin_op(1, 2, BinOp::Prod) // 14
//     .add_bin_op(1, 3, BinOp::Prod) // 15
//     .add_bin_op(1, 4, BinOp::Prod) // 16
//     .add_bin_op(2, 3, BinOp::Prod) // 17
//     .add_bin_op(2, 4, BinOp::Prod) // 18
//     .add_bin_op(3, 4, BinOp::Prod) // 19
//     .add_unary_op(5, cost[0]*cost[0], UnaryOp::Prod) // 20
//     .add_unary_op(6, cost[1]*cost[1], UnaryOp::Prod) // 21
//     .add_unary_op(7, cost[2]*cost[2], UnaryOp::Prod) // 22
//     .add_unary_op(8, cost[3]*cost[3], UnaryOp::Prod) // 23
//     .add_unary_op(9, cost[4]*cost[4], UnaryOp::Prod) // 24
//     .add_unary_op(10, 2.0*cost[0]*cost[1], UnaryOp::Prod) // 25
//     .add_unary_op(11, 2.0*cost[0]*cost[2], UnaryOp::Prod) // 26
//     .add_unary_op(12, 2.0*cost[0]*cost[3], UnaryOp::Prod) // 27
//     .add_unary_op(13, 2.0*cost[0]*cost[4], UnaryOp::Prod) // 28
//     .add_unary_op(14, 2.0*cost[1]*cost[2], UnaryOp::Prod) // 29
//     .add_unary_op(15, 2.0*cost[1]*cost[3], UnaryOp::Prod) // 30
//     .add_unary_op(16, 2.0*cost[1]*cost[4], UnaryOp::Prod) // 31
//     .add_unary_op(17, 2.0*cost[2]*cost[3], UnaryOp::Prod) // 32
//     .add_unary_op(18, 2.0*cost[2]*cost[4], UnaryOp::Prod) // 33
//     .add_unary_op(19, 2.0*cost[3]*cost[4], UnaryOp::Prod) // 34
//     .add_bin_op(20, 21, BinOp::Sum) // 35
//     .add_bin_op(35, 22, BinOp::Sum) // 36
//     .add_bin_op(36, 23, BinOp::Sum) // 37
//     .add_bin_op(37, 24, BinOp::Sum) // 38
//     .add_bin_op(38, 25, BinOp::Sum) // 39
//     .add_bin_op(39, 26, BinOp::Sum) // 40
//     .add_bin_op(40, 27, BinOp::Sum) // 41
//     .add_bin_op(41, 28, BinOp::Sum) // 42
//     .add_bin_op(42, 29, BinOp::Sum) // 43
//     .add_bin_op(43, 30, BinOp::Sum) // 44
//     .add_bin_op(44, 31, BinOp::Sum) // 45
//     .add_bin_op(45, 32, BinOp::Sum) // 46
//     .add_bin_op(46, 33, BinOp::Sum) // 47
//     .add_bin_op(47, 34, BinOp::Sum) // 48
//     .build();

//     let mut m = CBayesian::<Amv> {
//         delta, exp_monitor, exp2_monitor
//     };

//     let result = _run_bayes_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// fn exp_bayesian_admission_small_lv_bias(n:i32, big:bool) {
//     let mut mc = util::markov_chain_admission_small_lv();
//     if big {
//         mc = util::markov_chain_admission_small_lv();
//     }

    
//     let mapper = Amvm {};
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 19.0;

//     let mut exp_monitor =
//     BayesianBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0),num_vertices,1.0) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(2000.0),num_vertices+10000.0,10000.0) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(20000.0),num_vertices+10000.0,1.0) // 2
//     .add_unary_op(0, 0.0, UnaryOp::Prod) // 3
//     .add_unary_op(1, 2000.0, UnaryOp::Prod) // 4
//     .add_unary_op(2, 20000.0, UnaryOp::Prod) // 5
//     .add_bin_op(3, 4, BinOp::Sum) // 6
//     .add_bin_op(6, 5, BinOp::Sum) // 7
//     .build();


//     let mut exp2_monitor =
//     BayesianBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0.0),num_vertices,1.0) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(2000.0),num_vertices,1.0) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(20000.0),num_vertices,1.0) // 2
//     .add_unary_op(0, 1.0, UnaryOp::Square) // 3
//     .add_unary_op(1, 1.0, UnaryOp::Square) // 4
//     .add_unary_op(2, 1.0, UnaryOp::Square) // 5
//     .add_bin_op(0, 1, BinOp::Prod) // 6
//     .add_bin_op(0, 2, BinOp::Prod) // 7
//     .add_bin_op(1, 2, BinOp::Prod) // 8
//     .add_unary_op(3, 0.0, UnaryOp::Prod) // 9
//     .add_unary_op(4, 2000.0*2000.0, UnaryOp::Prod) // 10
//     .add_unary_op(5, 20000.0*20000.0, UnaryOp::Prod) // 11
//     .add_unary_op(6, 2.0*0.0*2000.0, UnaryOp::Prod) // 12
//     .add_unary_op(7, 2.0*0.0*20000.0, UnaryOp::Prod) // 13
//     .add_unary_op(8, 2.0*2000.0*20000.0, UnaryOp::Prod) // 14
//     .add_bin_op(9, 10, BinOp::Sum) // 15
//     .add_bin_op(15, 11, BinOp::Sum) // 16
//     .add_bin_op(16, 12, BinOp::Sum) // 17
//     .add_bin_op(17, 13, BinOp::Sum) // 18
//     .add_bin_op(18, 14, BinOp::Sum) // 18
//     .build();

//     let mut m = CBayesian::<Amv> {
//         delta, exp_monitor, exp2_monitor
//     };

//     let result = _run_bayes_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }


// fn exp_frequentist_lending_dp(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;

//     let mut m = FrequentistBuilder::<Lmv>::new()
//     .set_delta(0.05)
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 1
//     .add_bin_op(0, 1, BinOp::Divide)
//     .build();
    
//     println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//         1.0 / 0.4,
//         (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
//         (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
//         1.0/ 0.4,
//         (Lmv::Sample(s), Lmv::Decision(s, d)),
//         (Lmv::Decision(s, d), Lmv::Payback(s, p))
//     );
//     println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
//     let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// fn exp_fafrequentist_lending_dp(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;

//     let mut m = FaFrequentistBuilder::<Lmv>::new()
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 1
//     .add_bin_op(0, 1, BinOp::Subtract)
//         .set_delta(0.05)
//         .build();
    
//     println!(r#">>>> {{"specification_mapped": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//         1.0 / 0.4,
//         (Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)), 
//         (Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)),
//         1.0/ 0.4,
//         (Lmv::Sample(s), Lmv::Decision(s, d)),
//         (Lmv::Decision(s, d), Lmv::Payback(s, p))
//     );
//     println!(r#">>>> {{"specification": "{:?} * v{:?} * v{:?} - {:?} * v{:?} * v{:?}"}}"#,
//     1.0 / 0.4,(s_prime,(s_prime, d)),((s_prime,d),(s_prime, p)), 1.0/ 0.4,(s,(s, d)),((s,d),(s, p)));
//     let result = _run_fafreq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// fn exp_bayesian_lending_dp(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 100.0;

//     let mut exp_monitor = BayesianBuilder::<Lmv>::new()
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d),num_vertices,1.0) // 0
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d),num_vertices,1.0) // 1
//     .add_bin_op(0, 1, BinOp::Subtract)
//     .build();

//     let mut exp2_monitor =
//     // BayesianBuilder::<Lmv>::new()
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), num_vertices,1.0) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p), num_vertices,1.0) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d), num_vertices,1.0) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p), num_vertices,1.0) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 3, BinOp::Prod) // 8
//     // .add_bin_op(7, 7, BinOp::Prod) // 9
//     // .add_bin_op(3, 7, BinOp::Prod) // 10
//     // .add_unary_op(10, 2.0, UnaryOp::Prod) // 11
//     // .add_bin_op(8, 9, BinOp::Sum) // 12
//     // .add_bin_op(12, 11, BinOp::Subtract) // 12
//     // .build();
//     BayesianBuilder::<Lmv>::new()
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d),num_vertices,1.0) // 0
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d),num_vertices,1.0) // 1
//     .add_bin_op(0, 1, BinOp::Subtract)
//     .add_unary_op(2, 1.0, UnaryOp::Square)
//     .build();

//     let mut m = CBayesian::<Lmv> {
//         delta, exp_monitor, exp2_monitor
//     };

//     let result = _run_bayes_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }



// fn exp_bayesian_lending_dp_bp(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 100.0;

//     let mut exp_monitor =
//     BayesianBuilder::<Lmv>::new()
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d),num_vertices+1000.0,1000.0) // 0
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d),num_vertices+1000.0,1.0) // 1
//     .add_bin_op(0, 1, BinOp::Subtract)
//     .build();

//     let mut exp2_monitor =
//     // BayesianBuilder::<Lmv>::new()
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), num_vertices,1.0) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p), num_vertices,1.0) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d), num_vertices,1.0) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p), num_vertices,1.0) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 3, BinOp::Prod) // 8
//     // .add_bin_op(7, 7, BinOp::Prod) // 9
//     // .add_bin_op(3, 7, BinOp::Prod) // 10
//     // .add_unary_op(10, 2.0, UnaryOp::Prod) // 11
//     // .add_bin_op(8, 9, BinOp::Sum) // 12
//     // .add_bin_op(12, 11, BinOp::Subtract) // 12
//     // .build();
//     BayesianBuilder::<Lmv>::new()
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d),num_vertices+1000.0,1000.0) // 0
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d),num_vertices+1000.0,1.0) // 1
//     .add_bin_op(0, 1, BinOp::Subtract)
//     .add_unary_op(2, 1.0, UnaryOp::Square)
//     .build();

//     let mut m = CBayesian::<Lmv> {
//         delta, exp_monitor, exp2_monitor
//     };

//     let result = _run_bayes_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }




// fn _run_freq_lending_memless(
//     mc: &mut Lmmc,
//     mapper: &Lmvm,
//     monitors: &mut Vec<&mut Frequentist<Lmv>>,
//     n: i32
// ) -> Vec<Option<(f64, f64)>> {
//     let mut result = vec![None; monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }

//         let sigma = sigma.unwrap();

//         for j in 0..monitors.len() {
//             if i == 0 {
//                 monitors[j].init(sigma.clone());
//             } else {
//                 start_time = SystemTime::now();
//                 result[j] = monitors[j].next(sigma.clone());
//                 end_time = SystemTime::now();
//                 duration = end_time.duration_since(start_time).unwrap().as_nanos();
//             }
//             if result[j].is_some(){
//                 if (result[j].unwrap().0 != out_res.0) || (result[j].unwrap().1 != out_res.1) {
//                     out_change = true;
//                 } else {
//                     out_change = false;
//                 }
//                 out_res =(result[j].unwrap().0,result[j].unwrap().1);
//             }
//             println!(r#">>>> {{"i": {}, "sigma": "{:?}", "change": {:?}, "start_time": "{:?}", "end_time": "{:?}", "duration": {:?},"result": [{:?},{:?}]}}"#, i, sigma, out_change, start_time, end_time,duration, out_res.0, out_res.1);
//         } 
//     }
//     result
// }



// fn _run_fafreq_lending_memless(
//     mc: &mut Lmmc,
//     mapper: &Lmvm,
//     monitors: &mut Vec<&mut FaFrequentist<Lmv>>,
//     n: i32
// ) -> Vec<Option<(f64, f64)>> {
//     let mut result = vec![None; monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }

//         let sigma = sigma.unwrap();

//         for j in 0..monitors.len() {
//             if i == 0 {
//                 monitors[j].init(sigma.clone());
//             } else {
//                 start_time = SystemTime::now();
//                 let locres =  monitors[j].next(sigma.clone());
//                 result[j] =  Some((locres.value - locres.epsilon, locres.value + locres.epsilon));
//                 end_time = SystemTime::now();
//                 duration = end_time.duration_since(start_time).unwrap().as_nanos();
//             }
//             if result[j].is_some(){
//                 if (result[j].unwrap().0 != out_res.0) || (result[j].unwrap().1 != out_res.1) {
//                     out_change = true;
//                 } else {
//                     out_change = false;
//                 }
//                 out_res =(result[j].unwrap().0,result[j].unwrap().1);
//             }
//             println!(r#">>>> {{"i": {}, "sigma": "{:?}", "change": {:?}, "start_time": "{:?}", "end_time": "{:?}", "duration": {:?},"result": [{:?},{:?}]}}"#, i, sigma, out_change, start_time, end_time,duration, out_res.0, out_res.1);
//         } 
//     }
//     result
// }

// fn _run_bayes_lending_memless(
//     mc: &mut Lmmc,
//     mapper: &Lmvm,
//     monitors: &mut Vec<&mut CBayesian<Lmv>>,
//     n: i32
// ) -> Vec<(f64, f64)> {
//     let mut result = vec![(0.0,0.0); monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }

//         let sigma = sigma.unwrap();

//         for j in 0..monitors.len() {
//             if i == 0 {
//                 monitors[j].init(sigma.clone());
//             } else {
//                 start_time = SystemTime::now();
//                 result[j] = monitors[j].next(sigma.clone());
//                 end_time = SystemTime::now();
//                 duration = end_time.duration_since(start_time).unwrap().as_nanos();
//             }
//             if (result[j].0 != out_res.0) || (result[j].1 != out_res.1) {
//                 out_change = true;
//             } else {
//                 out_change = false;
//             }
//             out_res =(result[j].0,result[j].1);
//             println!(r#">>>> {{"i": {}, "sigma": "{:?}", "change": {:?}, "start_time": "{:?}", "end_time": "{:?}", "duration": {:?},"result": [{:?},{:?}]}}"#, i, sigma, out_change, start_time, end_time,duration, out_res.0, out_res.1);
//         } 
//     }
//     result
// }


// fn _run_bayes_admission_memless(
//     mc: &mut Ammc,
//     mapper: &Amvm,
//     monitors: &mut Vec<&mut CBayesian<Amv>>,
//     n: i32
// ) -> Vec<(f64, f64)> {
//     let mut result = vec![(0.0,0.0); monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }

//         let sigma = sigma.unwrap();

//         for j in 0..monitors.len() {
//             if i == 0 {
//                 monitors[j].init(sigma.clone());
//             } else {
//                 start_time = SystemTime::now();
//                 result[j] = monitors[j].next(sigma.clone());
//                 end_time = SystemTime::now();
//                 duration = end_time.duration_since(start_time).unwrap().as_nanos();
//             }
//             if (result[j].0 != out_res.0) || (result[j].1 != out_res.1) {
//                 out_change = true;
//             } else {
//                 out_change = false;
//             }
//             out_res =(result[j].0,result[j].1);
//             println!(r#">>>> {{"i": {}, "sigma": "{:?}", "change": {:?}, "start_time": "{:?}", "end_time": "{:?}", "duration": {:?},"result": [{:?},{:?}]}}"#, i, sigma, out_change, start_time, end_time,duration, out_res.0, out_res.1);
//         } 
//     }
//     result
// }


// fn _run_freq_admission_memless(
//     mc: &mut Ammc,
//     mapper: &Amvm,
//     monitors: &mut Vec<&mut Frequentist<Amv>>,
//     n: i32
// ) -> Vec<Option<(f64, f64)>> {
//     let mut result = vec![None; monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }

//         let sigma = sigma.unwrap();

//         for j in 0..monitors.len() {
//             if i == 0 {
//                 monitors[j].init(sigma.clone());
//             } else {
//                 start_time = SystemTime::now();
//                 result[j] = monitors[j].next(sigma.clone());
//                 end_time = SystemTime::now();
//                 duration = end_time.duration_since(start_time).unwrap().as_nanos();
//             }
//             if result[j].is_some(){
//                 if (result[j].unwrap().0 != out_res.0) || (result[j].unwrap().1 != out_res.1) {
//                     out_change = true;
//                 } else {
//                     out_change = false;
//                 }
//                 out_res =(result[j].unwrap().0,result[j].unwrap().1);
//             }
//             println!(r#">>>> {{"i": {}, "sigma": "{:?}", "change": {:?}, "start_time": "{:?}", "end_time": "{:?}", "duration": {:?},"result": [{:?},{:?}]}}"#, i, sigma, out_change, start_time, end_time,duration, out_res.0, out_res.1);
//         } 
//     }
//     result
// }



// fn _run_fafreq_admission_memless(
//     mc: &mut Ammc,
//     mapper: &Amvm,
//     monitors: &mut Vec<&mut FaFrequentist<Amv>>,
//     n: i32
// ) -> Vec<Option<(f64, f64)>> {
//     let mut result = vec![None; monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }

//         let sigma = sigma.unwrap();

//         for j in 0..monitors.len() {
//             if i == 0 {
//                 monitors[j].init(sigma.clone());
//             } else {
//                 start_time = SystemTime::now();
//                 let locres =  monitors[j].next(sigma.clone());
//                 result[j] =  Some((locres.value - locres.epsilon, locres.value + locres.epsilon));
//                 end_time = SystemTime::now();
//                 duration = end_time.duration_since(start_time).unwrap().as_nanos();
//             }
//             if result[j].is_some(){
//                 if (result[j].unwrap().0 != out_res.0) || (result[j].unwrap().1 != out_res.1) {
//                     out_change = true;
//                 } else {
//                     out_change = false;
//                 }
//                 out_res =(result[j].unwrap().0,result[j].unwrap().1);
//             }
//             println!(r#">>>> {{"i": {}, "sigma": "{:?}", "change": {:?}, "start_time": "{:?}", "end_time": "{:?}", "duration": {:?},"result": [{:?},{:?}]}}"#, i, sigma, out_change, start_time, end_time,duration, out_res.0, out_res.1);
//         } 
//     }
//     result
// }