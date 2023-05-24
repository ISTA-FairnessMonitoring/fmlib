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
//         cbayesian::cbayesian::CBayesian,
//         bayesian_confint::bayesian_confint::BayesianConfInt,
//         bayesian_exp::{builder::{ANFTermBuilder, BayesianExpBuilder}, bayesian_exp::ANFExpr},
//         frequentist_opt::frequentist_opt::FrequentistOpt,
//         frequentist_opt::builder::FrequentistOptBuilder,

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
//     let n = 150000;
//     let it = 10;
//     // exp_bayesian_admission(n,false);

//     println!(r#">>>> {{"algorithm": "frequentist_opt", "experiment": "lending_dp", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_frequentistopt_lending_dp(n,false);
//     }


//     println!(r#">>>> {{"algorithm": "bayes", "experiment": "lending_dp", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_bayesian_lending_dp(n,false);
//     }
//     println!(r#">>>> {{"algorithm": "bayes_bp", "experiment": "lending_dp", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_bayesian_lending_bp_dp(n,false);
//     }

//     println!(r#">>>> {{"algorithm": "frequentist", "experiment": "lending_dp", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_frequentist_lending_dp(n,false);
//     }
//     println!(r#">>>> {{"algorithm": "fafrequentist", "experiment": "lending_dp", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_fafrequentist_lending_dp(n,false);
//     }



//     println!(r#">>>> {{"algorithm": "frequentist_opt", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_frequentistopt_lending(n,false);
//     }


//     println!(r#">>>> {{"algorithm": "bayes", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_bayesian_lending(n,false);
//     }
//     println!(r#">>>> {{"algorithm": "bayes_bp", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_bayesian_lending_bp(n,false);
//     }

//     println!(r#">>>> {{"algorithm": "frequentist", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_frequentist_lending(n,false);
//     }
//     println!(r#">>>> {{"algorithm": "fafrequentist", "experiment": "lending", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_fafrequentist_lending(n,false);
//     }

//     println!(r#">>>> {{"algorithm": "frequentist_opt", "experiment": "admission", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_frequentistopt_admission(n,true);
//     }

    
//     println!(r#">>>> {{"algorithm": "bayes", "experiment": "admission", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_bayesian_admission(n,true);
//     }
//     println!(r#">>>> {{"algorithm": "bayes_bp", "experiment": "admission", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_bayesian_admission_bp(n,true);
//     }
//     println!(r#">>>> {{"algorithm": "frequentist", "experiment": "admission", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_frequentist_admission(n,true);
//     }
//     println!(r#">>>> {{"algorithm": "fafrequentist", "experiment": "admission", "iterations": {:?}, "run_length": {:?}}}"#, n, it);
//     for i in 0..it{
//         println!(r#">>>> {{"run": {:?}}}"#, i);
//         exp_fafrequentist_admission(n,true);
//     }
 

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
//         .set_delta(0.05)
//         .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//         .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 1
//         .add_bin_op(0, 1, BinOp::Subtract)
//         .build();

//     let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }

// fn exp_frequentistopt_lending_dp(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;

//     let mut m = FrequentistOptBuilder::<Lmv>::new()
//     .set_delta(0.05)
//     .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 1
//     .add_bin_op(0, 1, BinOp::Subtract)
//     .build();


//     let result = _run_freqopt_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

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
//     .set_delta(0.05)
//     .build();
    
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
//     let num_vertices = 100;

//     // .set_delta(0.05)
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 7, BinOp::Subtract) // 8
//     // .build();

//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .set_constant(1.0)
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .set_constant(-1.0)
//     .build();

//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2] }; 

//     let exp_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices)
//     .set_sym_const(Lmv::Sample(s), num_vertices)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .build();
    
//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,2)
//     .set_constant(1.0)
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,2)
//     .set_constant(1.0)
//     .build();

//     let t3 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .set_constant(-2.0)
//     .build();


//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2, t3] }; 

//     let exp2_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices)
//     .set_sym_const(Lmv::Sample(s), num_vertices)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .build();
    
    
//     let delta = 0.05;

//     let mut m = BayesianConfInt {
//         exp_monitor, exp2_monitor, delta, verdict: (0.0, 0.0)
//     };

//     let result = _run_bayes_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);
// }





// fn exp_bayesian_lending_bp_dp(n:i32, fair:bool) {

//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let num_vertices = 100;

//     // .set_delta(0.05)
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 7, BinOp::Subtract) // 8
//     // .build();

//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .set_constant(1.0)
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .set_constant(-1.0)
//     .build();

//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2] }; 

//     let exp_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices+1000)
//     .set_sym_const(Lmv::Sample(s), num_vertices+1000)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1001)
//     .build();
    
//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,2)
//     .set_constant(1.0)
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,2)
//     .set_constant(1.0)
//     .build();

//     let t3 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .set_constant(-2.0)
//     .build();


//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2, t3] }; 

//     let exp2_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices+1000)
//     .set_sym_const(Lmv::Sample(s), num_vertices+1000)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1001)
//     .build();
    
    
    
//     let delta = 0.05;

//     let mut m = BayesianConfInt {
//         exp_monitor, exp2_monitor, delta, verdict: (0.0, 0.0)
//     };

//     let result = _run_bayes_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);
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

//     let result = _run_freq_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }

// fn exp_frequentistopt_lending(n:i32, fair:bool) {
//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;

//     let mut m = FrequentistOptBuilder::<Lmv>::new()
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

//     let result = _run_freqopt_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);

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
//     let num_vertices = 100;

//     // .set_delta(0.05)
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 7, BinOp::Subtract) // 8
//     // .build();

//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .add_var(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_constant((1.0/0.4))
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .add_var(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .set_constant(-1.0*(1.0/0.4))
//     .build();

//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2] }; 

//     let exp_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices)
//     .set_sym_const(Lmv::Decision(s_prime, d), num_vertices)
//     .set_sym_const(Lmv::Sample(s), num_vertices)
//     .set_sym_const(Lmv::Decision(s, d), num_vertices)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .set_trans_const(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .build();
    
//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,2)
//     .add_var(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,2)
//     .set_constant((1.0/0.4)*(1.0/0.4))
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,2)
//     .add_var(Lmv::Decision(s, d), Lmv::Payback(s, p) ,2)
//     .set_constant((1.0/0.4)*(1.0/0.4))
//     .build();

//     let t3 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .add_var(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .add_var(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_constant((-2.0)*(1.0/0.4)*(1.0/0.4))
//     .build();


//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2, t3] }; 

//     let exp2_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices)
//     .set_sym_const(Lmv::Decision(s_prime, d), num_vertices)
//     .set_sym_const(Lmv::Sample(s), num_vertices)
//     .set_sym_const(Lmv::Decision(s, d), num_vertices)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .set_trans_const(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .build();
    
//     let delta = 0.05;

//     let mut m = BayesianConfInt {
//         exp_monitor, exp2_monitor, delta, verdict: (0.0, 0.0)
//     };

//     let result = _run_bayes_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);
// }




// fn exp_bayesian_lending_false(n:i32, fair:bool) {

//     let mut mc = util::markov_chain_lending_large();
//     if fair {
//         mc = util::markov_chain_lending_large_fair();
//     }
    
//     let mapper = Lmvm {};
//     let (s, s_prime) = ((0, 5), (1, 5));
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let num_vertices = 100;

//     // .set_delta(0.05)
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 7, BinOp::Subtract) // 8
//     // .build();

//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .add_var(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_constant((1.0/0.4))
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .add_var(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .set_constant(-1.0*(1.0/0.4))
//     .build();

//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2] }; 

//     let exp_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices)
//     .set_sym_const(Lmv::Decision(s_prime, d), num_vertices)
//     .set_sym_const(Lmv::Sample(s), num_vertices)
//     .set_sym_const(Lmv::Decision(s, d), num_vertices)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .set_trans_const(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .build();
    
//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,2)
//     .add_var(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,2)
//     .set_constant((1.0/0.4)*(1.0/0.4))
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,2)
//     .add_var(Lmv::Decision(s, d), Lmv::Payback(s, p) ,2)
//     .set_constant((1.0/0.4)*(1.0/0.4))
//     .build();

//     let t3 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .add_var(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .add_var(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_constant((2.0)*(1.0/0.4)*(1.0/0.4))
//     .build();


//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2, t3] }; 

//     let exp2_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices)
//     .set_sym_const(Lmv::Decision(s_prime, d), num_vertices)
//     .set_sym_const(Lmv::Sample(s), num_vertices)
//     .set_sym_const(Lmv::Decision(s, d), num_vertices)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .set_trans_const(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .build();
    
//     let delta = 0.05;

//     let mut m = BayesianConfInt {
//         exp_monitor, exp2_monitor, delta, verdict: (0.0, 0.0)
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
//     let num_vertices = 100;

//     // .set_delta(0.05)
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 7, BinOp::Subtract) // 8
//     // .build();

//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .add_var(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_constant((1.0/0.4))
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .add_var(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .set_constant(-1.0*(1.0/0.4))
//     .build();

//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2] }; 

//     let exp_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices+1000)
//     .set_sym_const(Lmv::Decision(s_prime, d), num_vertices)
//     .set_sym_const(Lmv::Sample(s), num_vertices+1000)
//     .set_sym_const(Lmv::Decision(s, d), num_vertices)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1001)
//     .set_trans_const(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .build();
    
//     let t1 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,2)
//     .add_var(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,2)
//     .set_constant((1.0/0.4)*(1.0/0.4))
//     .build();

//     let t2 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,2)
//     .add_var(Lmv::Decision(s, d), Lmv::Payback(s, p) ,2)
//     .set_constant((1.0/0.4)*(1.0/0.4))
//     .build();

//     let t3 = 
//     ANFTermBuilder::<Lmv>::new()
//     .add_var(Lmv::Sample(s), Lmv::Decision(s, d) ,1)
//     .add_var(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .add_var(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d) ,1)
//     .add_var(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_constant((-2.0)*(1.0/0.4)*(1.0/0.4))
//     .build();


//     let phi: ANFExpr<Lmv> = ANFExpr { terms: vec![t1, t2, t3] }; 

//     let exp2_monitor =
//     BayesianExpBuilder::<Lmv>::new()
//     .set_expr(phi)
//     .set_sym_const(Lmv::Sample(s_prime), num_vertices+1000)
//     .set_sym_const(Lmv::Decision(s_prime, d), num_vertices)
//     .set_sym_const(Lmv::Sample(s), num_vertices+1000)
//     .set_sym_const(Lmv::Decision(s, d), num_vertices)
//     .set_trans_const(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d), 1)
//     .set_trans_const(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p) ,1)
//     .set_trans_const(Lmv::Sample(s), Lmv::Decision(s, d) ,1001)
//     .set_trans_const(Lmv::Decision(s, d), Lmv::Payback(s, p) ,1)
//     .build();
    
//     let delta = 0.05;

//     let mut m = BayesianConfInt {
//         exp_monitor, exp2_monitor, delta, verdict: (0.0, 0.0)
//     };

//     let result = _run_bayes_lending_memless(&mut mc, &mapper, &mut vec![&mut m], n);
// }





// fn exp_frequentist_admission(n:i32, big:bool) {
//     let mut mc = util::markov_chain_admission_large();
//     if big {
//         mc = util::markov_chain_admission_large();
//     }
//     let mapper = Amvm {};
//     let cost = vec![0.0, 0.01, 0.04, 0.09, 0.16, 0.25, 0.36, 0.49, 0.64, 0.81, 1.0];

//     let mut m = FrequentistBuilder::<Amv>::new()
//     .set_delta(0.05)
//     .add_freq(Amv::Sample(true), Amv::Cost(0)) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1)) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(2)) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(3)) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(4)) // 4
//     .add_freq(Amv::Sample(true), Amv::Cost(5)) // 5
//     .add_freq(Amv::Sample(true), Amv::Cost(6)) // 6
//     .add_freq(Amv::Sample(true), Amv::Cost(7)) // 7
//     .add_freq(Amv::Sample(true), Amv::Cost(8)) // 8
//     .add_freq(Amv::Sample(true), Amv::Cost(9)) // 9
//     .add_freq(Amv::Sample(true), Amv::Cost(10)) // 10
//     .add_unary_op(0, cost[0], UnaryOp::Prod) // 11
//     .add_unary_op(1, cost[1], UnaryOp::Prod) // 12
//     .add_unary_op(2, cost[2], UnaryOp::Prod) // 13
//     .add_unary_op(3, cost[3], UnaryOp::Prod) // 14
//     .add_unary_op(4, cost[4], UnaryOp::Prod) // 15
//     .add_unary_op(5, cost[5], UnaryOp::Prod) // 16
//     .add_unary_op(6, cost[6], UnaryOp::Prod) // 17
//     .add_unary_op(7, cost[7], UnaryOp::Prod) // 18
//     .add_unary_op(8, cost[8], UnaryOp::Prod) // 19
//     .add_unary_op(9, cost[9], UnaryOp::Prod) // 20
//     .add_unary_op(10, cost[10], UnaryOp::Prod) // 21
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


// fn exp_frequentistopt_admission(n:i32, big:bool) {
//     let mut mc = util::markov_chain_admission_large();
//     if big {
//         mc = util::markov_chain_admission_large();
//     }
//     let mapper = Amvm {};
//     let cost = vec![0.0, 0.01, 0.04, 0.09, 0.16, 0.25, 0.36, 0.49, 0.64, 0.81, 1.0];

//     let mut m = FrequentistOptBuilder::<Amv>::new()
//     .set_delta(0.05)
//     .add_freq(Amv::Sample(true), Amv::Cost(0)) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1)) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(2)) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(3)) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(4)) // 4
//     .add_freq(Amv::Sample(true), Amv::Cost(5)) // 5
//     .add_freq(Amv::Sample(true), Amv::Cost(6)) // 6
//     .add_freq(Amv::Sample(true), Amv::Cost(7)) // 7
//     .add_freq(Amv::Sample(true), Amv::Cost(8)) // 8
//     .add_freq(Amv::Sample(true), Amv::Cost(9)) // 9
//     .add_freq(Amv::Sample(true), Amv::Cost(10)) // 10
//     .add_unary_op(0, cost[0], UnaryOp::Prod) // 11
//     .add_unary_op(1, cost[1], UnaryOp::Prod) // 12
//     .add_unary_op(2, cost[2], UnaryOp::Prod) // 13
//     .add_unary_op(3, cost[3], UnaryOp::Prod) // 14
//     .add_unary_op(4, cost[4], UnaryOp::Prod) // 15
//     .add_unary_op(5, cost[5], UnaryOp::Prod) // 16
//     .add_unary_op(6, cost[6], UnaryOp::Prod) // 17
//     .add_unary_op(7, cost[7], UnaryOp::Prod) // 18
//     .add_unary_op(8, cost[8], UnaryOp::Prod) // 19
//     .add_unary_op(9, cost[9], UnaryOp::Prod) // 20
//     .add_unary_op(10, cost[10], UnaryOp::Prod) // 21
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

//     let result = _run_freqopt_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);
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
//         mc = util::markov_chain_admission_large();
//     }

//     let mapper = Amvm {};
//     let cost = vec![0.0, 0.01, 0.04, 0.09, 0.16, 0.25, 0.36, 0.49, 0.64, 0.81, 1.0];

//     let mut m = FaFrequentistBuilder::<Amv>::new()
//     .add_freq(Amv::Sample(true), Amv::Cost(0)) // 0
//     .add_freq(Amv::Sample(true), Amv::Cost(1)) // 1
//     .add_freq(Amv::Sample(true), Amv::Cost(2)) // 2
//     .add_freq(Amv::Sample(true), Amv::Cost(3)) // 3
//     .add_freq(Amv::Sample(true), Amv::Cost(4)) // 4
//     .add_freq(Amv::Sample(true), Amv::Cost(5)) // 5
//     .add_freq(Amv::Sample(true), Amv::Cost(6)) // 6
//     .add_freq(Amv::Sample(true), Amv::Cost(7)) // 7
//     .add_freq(Amv::Sample(true), Amv::Cost(8)) // 8
//     .add_freq(Amv::Sample(true), Amv::Cost(9)) // 9
//     .add_freq(Amv::Sample(true), Amv::Cost(10)) // 10
//     .add_constant(cost[0]) // 11
//     .add_constant(cost[1]) // 12
//     .add_constant(cost[2]) // 13
//     .add_constant(cost[3]) // 14
//     .add_constant(cost[4]) // 15
//     .add_constant(cost[5]) // 16
//     .add_constant(cost[6]) // 17
//     .add_constant(cost[7]) // 18
//     .add_constant(cost[8]) // 19
//     .add_constant(cost[9]) // 20
//     .add_constant(cost[10]) // 21
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
//         mc = util::markov_chain_admission_large();
//     }

//     let mapper = Amvm {};
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 14;
//     let cost = vec![0.0, 0.01, 0.04, 0.09, 0.16, 0.25, 0.36, 0.49, 0.64, 0.81, 1.0];

//     // .set_delta(0.05)
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 7, BinOp::Subtract) // 8
//     // .build();
//     let t0 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .set_constant(cost[0])
//     .build();
    

//     let t1 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .set_constant(cost[1])
//     .build();
    

//     let t2 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_constant(cost[2])
//     .build();
    

//     let t3 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_constant(cost[3])
//     .build();
    

//     let t4 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(cost[4])
//     .build();
    

//     let t5 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(cost[5])
//     .build();
    

//     let t6 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(cost[6])
//     .build();
    

//     let t7 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(cost[7])
//     .build();
    

//     let t8 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(cost[8])
//     .build();
    

//     let t9 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(cost[9])
//     .build();
    

//     let t10 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(cost[10])
//     .build();
    


//     let phi: ANFExpr<Amv> = ANFExpr { terms: vec![t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10] }; 

//     let exp_monitor =
//     BayesianExpBuilder::<Amv>::new()
//     .set_expr(phi)
//     .set_sym_const(Amv::Sample(true) ,num_vertices)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(0) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(1) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(10) ,1)
//     .build();
//     let t0 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,2)
//     .set_constant(cost[0]*cost[0])
//     .build();
    

//     let t1 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,2)
//     .set_constant(cost[1]*cost[1])
//     .build();
    

//     let t2 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,2)
//     .set_constant(cost[2]*cost[2])
//     .build();
    

//     let t3 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,2)
//     .set_constant(cost[3]*cost[3])
//     .build();
    

//     let t4 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,2)
//     .set_constant(cost[4]*cost[4])
//     .build();
    

//     let t5 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,2)
//     .set_constant(cost[5]*cost[5])
//     .build();
    

//     let t6 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,2)
//     .set_constant(cost[6]*cost[6])
//     .build();
    

//     let t7 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,2)
//     .set_constant(cost[7]*cost[7])
//     .build();
    

//     let t8 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,2)
//     .set_constant(cost[8]*cost[8])
//     .build();
    

//     let t9 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,2)
//     .set_constant(cost[9]*cost[9])
//     .build();
    

//     let t10 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,2)
//     .set_constant(cost[10]*cost[10])
//     .build();
    

//     let t11 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .set_constant(2.0*cost[0]*cost[1])
//     .build();
    

//     let t12 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_constant(2.0*cost[0]*cost[2])
//     .build();
    

//     let t13 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_constant(2.0*cost[0]*cost[3])
//     .build();
    

//     let t14 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(2.0*cost[0]*cost[4])
//     .build();
    

//     let t15 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[0]*cost[5])
//     .build();
    

//     let t16 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[0]*cost[6])
//     .build();
    

//     let t17 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[0]*cost[7])
//     .build();
    

//     let t18 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[0]*cost[8])
//     .build();
    

//     let t19 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[0]*cost[9])
//     .build();
    

//     let t20 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[0]*cost[10])
//     .build();
    

//     let t21 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_constant(2.0*cost[1]*cost[2])
//     .build();
    

//     let t22 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_constant(2.0*cost[1]*cost[3])
//     .build();
    

//     let t23 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(2.0*cost[1]*cost[4])
//     .build();
    

//     let t24 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[1]*cost[5])
//     .build();
    

//     let t25 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[1]*cost[6])
//     .build();
    

//     let t26 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[1]*cost[7])
//     .build();
    

//     let t27 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[1]*cost[8])
//     .build();
    

//     let t28 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[1]*cost[9])
//     .build();
    

//     let t29 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[1]*cost[10])
//     .build();
    

//     let t30 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_constant(2.0*cost[2]*cost[3])
//     .build();
    

//     let t31 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(2.0*cost[2]*cost[4])
//     .build();
    

//     let t32 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[2]*cost[5])
//     .build();
    

//     let t33 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[2]*cost[6])
//     .build();
    

//     let t34 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[2]*cost[7])
//     .build();
    

//     let t35 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[2]*cost[8])
//     .build();
    

//     let t36 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[2]*cost[9])
//     .build();
    

//     let t37 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[2]*cost[10])
//     .build();
    

//     let t38 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(2.0*cost[3]*cost[4])
//     .build();
    

//     let t39 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[3]*cost[5])
//     .build();
    

//     let t40 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[3]*cost[6])
//     .build();
    

//     let t41 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[3]*cost[7])
//     .build();
    

//     let t42 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[3]*cost[8])
//     .build();
    

//     let t43 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[3]*cost[9])
//     .build();
    

//     let t44 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[3]*cost[10])
//     .build();
    

//     let t45 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[4]*cost[5])
//     .build();
    

//     let t46 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[4]*cost[6])
//     .build();
    

//     let t47 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[4]*cost[7])
//     .build();
    

//     let t48 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[4]*cost[8])
//     .build();
    

//     let t49 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[4]*cost[9])
//     .build();
    

//     let t50 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[4]*cost[10])
//     .build();
    

//     let t51 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[5]*cost[6])
//     .build();
    

//     let t52 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[5]*cost[7])
//     .build();
    

//     let t53 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[5]*cost[8])
//     .build();
    

//     let t54 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[5]*cost[9])
//     .build();
    

//     let t55 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[5]*cost[10])
//     .build();
    

//     let t56 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[6]*cost[7])
//     .build();
    

//     let t57 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[6]*cost[8])
//     .build();
    

//     let t58 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[6]*cost[9])
//     .build();
    

//     let t59 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[6]*cost[10])
//     .build();
    

//     let t60 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[7]*cost[8])
//     .build();
    

//     let t61 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[7]*cost[9])
//     .build();
    

//     let t62 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[7]*cost[10])
//     .build();
    

//     let t63 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[8]*cost[9])
//     .build();
    

//     let t64 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[8]*cost[10])
//     .build();
    

//     let t65 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[9]*cost[10])
//     .build();
    

//     let phi: ANFExpr<Amv> = ANFExpr { terms: vec![t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59, t60, t61, t62, t63, t64, t65] }; 

//     let exp2_monitor =
//     BayesianExpBuilder::<Amv>::new()
//     .set_expr(phi)
//     .set_sym_const(Amv::Sample(true) ,num_vertices)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(0) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(1) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(10) ,1)
//     .build();
    
//     let delta = 0.05;

//     let mut m = BayesianConfInt {
//         exp_monitor, exp2_monitor, delta, verdict: (0.0, 0.0)
//     };

//     let result = _run_bayes_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }


// fn exp_bayesian_admission_bp(n:i32, big:bool) {
//     let mut mc = util::markov_chain_admission_large();
//     if big {
//         mc = util::markov_chain_admission_large();
//     }

//     let mapper = Amvm {};
//     let d = Decision::Accept;
//     let p = Payback::Success;
//     let delta: f64 = 0.05;
//     let num_vertices = 14;
//     let cost = vec![0.0, 0.01, 0.04, 0.09, 0.16, 0.25, 0.36, 0.49, 0.64, 0.81, 1.0];

//     // .set_delta(0.05)
//     // .add_freq(Lmv::Sample(s_prime), Lmv::Decision(s_prime, d)) // 0
//     // .add_freq(Lmv::Decision(s_prime, d), Lmv::Payback(s_prime, p)) // 1
//     // .add_bin_op(0, 1, BinOp::Prod) // 2
//     // .add_unary_op(2, 1.0 / 0.4, UnaryOp::Prod) // 3
//     // .add_freq(Lmv::Sample(s), Lmv::Decision(s, d)) // 4
//     // .add_freq(Lmv::Decision(s, d), Lmv::Payback(s, p)) // 5
//     // .add_bin_op(4, 5, BinOp::Prod) // 6
//     // .add_unary_op(6, 1.0 / 0.4, UnaryOp::Prod) // 7
//     // .add_bin_op(3, 7, BinOp::Subtract) // 8
//     // .build();
//     let t0 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .set_constant(cost[0])
//     .build();
    

//     let t1 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .set_constant(cost[1])
//     .build();
    

//     let t2 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_constant(cost[2])
//     .build();
    

//     let t3 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_constant(cost[3])
//     .build();
    

//     let t4 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(cost[4])
//     .build();
    

//     let t5 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(cost[5])
//     .build();
    

//     let t6 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(cost[6])
//     .build();
    

//     let t7 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(cost[7])
//     .build();
    

//     let t8 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(cost[8])
//     .build();
    

//     let t9 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(cost[9])
//     .build();
    

//     let t10 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(cost[10])
//     .build();
    


//     let phi: ANFExpr<Amv> = ANFExpr { terms: vec![t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10] }; 

//     let exp_monitor =
//     BayesianExpBuilder::<Amv>::new()
//     .set_expr(phi)
//     .set_sym_const(Amv::Sample(true) ,num_vertices+1000)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(0) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(1) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(8) ,1000)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(10),1)
//     .build();
//     let t0 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,2)
//     .set_constant(cost[0]*cost[0])
//     .build();
    

//     let t1 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,2)
//     .set_constant(cost[1]*cost[1])
//     .build();
    

//     let t2 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,2)
//     .set_constant(cost[2]*cost[2])
//     .build();
    

//     let t3 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,2)
//     .set_constant(cost[3]*cost[3])
//     .build();
    

//     let t4 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,2)
//     .set_constant(cost[4]*cost[4])
//     .build();
    

//     let t5 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,2)
//     .set_constant(cost[5]*cost[5])
//     .build();
    

//     let t6 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,2)
//     .set_constant(cost[6]*cost[6])
//     .build();
    

//     let t7 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,2)
//     .set_constant(cost[7]*cost[7])
//     .build();
    

//     let t8 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,2)
//     .set_constant(cost[8]*cost[8])
//     .build();
    

//     let t9 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,2)
//     .set_constant(cost[9]*cost[9])
//     .build();
    

//     let t10 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,2)
//     .set_constant(cost[10]*cost[10])
//     .build();
    

//     let t11 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .set_constant(2.0*cost[0]*cost[1])
//     .build();
    

//     let t12 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_constant(2.0*cost[0]*cost[2])
//     .build();
    

//     let t13 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_constant(2.0*cost[0]*cost[3])
//     .build();
    

//     let t14 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(2.0*cost[0]*cost[4])
//     .build();
    

//     let t15 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[0]*cost[5])
//     .build();
    

//     let t16 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[0]*cost[6])
//     .build();
    

//     let t17 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[0]*cost[7])
//     .build();
    

//     let t18 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[0]*cost[8])
//     .build();
    

//     let t19 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[0]*cost[9])
//     .build();
    

//     let t20 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(0) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[0]*cost[10])
//     .build();
    

//     let t21 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_constant(2.0*cost[1]*cost[2])
//     .build();
    

//     let t22 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_constant(2.0*cost[1]*cost[3])
//     .build();
    

//     let t23 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(2.0*cost[1]*cost[4])
//     .build();
    

//     let t24 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[1]*cost[5])
//     .build();
    

//     let t25 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[1]*cost[6])
//     .build();
    

//     let t26 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[1]*cost[7])
//     .build();
    

//     let t27 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[1]*cost[8])
//     .build();
    

//     let t28 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[1]*cost[9])
//     .build();
    

//     let t29 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(1) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[1]*cost[10])
//     .build();
    

//     let t30 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_constant(2.0*cost[2]*cost[3])
//     .build();
    

//     let t31 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(2.0*cost[2]*cost[4])
//     .build();
    

//     let t32 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[2]*cost[5])
//     .build();
    

//     let t33 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[2]*cost[6])
//     .build();
    

//     let t34 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[2]*cost[7])
//     .build();
    

//     let t35 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[2]*cost[8])
//     .build();
    

//     let t36 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[2]*cost[9])
//     .build();
    

//     let t37 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(2) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[2]*cost[10])
//     .build();
    

//     let t38 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_constant(2.0*cost[3]*cost[4])
//     .build();
    

//     let t39 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[3]*cost[5])
//     .build();
    

//     let t40 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[3]*cost[6])
//     .build();
    

//     let t41 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[3]*cost[7])
//     .build();
    

//     let t42 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[3]*cost[8])
//     .build();
    

//     let t43 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[3]*cost[9])
//     .build();
    

//     let t44 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(3) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[3]*cost[10])
//     .build();
    

//     let t45 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_constant(2.0*cost[4]*cost[5])
//     .build();
    

//     let t46 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[4]*cost[6])
//     .build();
    

//     let t47 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[4]*cost[7])
//     .build();
    

//     let t48 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[4]*cost[8])
//     .build();
    

//     let t49 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[4]*cost[9])
//     .build();
    

//     let t50 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(4) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[4]*cost[10])
//     .build();
    

//     let t51 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_constant(2.0*cost[5]*cost[6])
//     .build();
    

//     let t52 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[5]*cost[7])
//     .build();
    

//     let t53 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[5]*cost[8])
//     .build();
    

//     let t54 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[5]*cost[9])
//     .build();
    

//     let t55 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(5) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[5]*cost[10])
//     .build();
    

//     let t56 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_constant(2.0*cost[6]*cost[7])
//     .build();
    

//     let t57 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[6]*cost[8])
//     .build();
    

//     let t58 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[6]*cost[9])
//     .build();
    

//     let t59 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(6) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[6]*cost[10])
//     .build();
    

//     let t60 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .set_constant(2.0*cost[7]*cost[8])
//     .build();
    

//     let t61 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[7]*cost[9])
//     .build();
    

//     let t62 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(7) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[7]*cost[10])
//     .build();
    

//     let t63 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_constant(2.0*cost[8]*cost[9])
//     .build();
    

//     let t64 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(8) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[8]*cost[10])
//     .build();
    

//     let t65 = ANFTermBuilder::<Amv>::new()
//     .add_var(Amv::Sample(true), Amv::Cost(9) ,1)
//     .add_var(Amv::Sample(true), Amv::Cost(10) ,1)
//     .set_constant(2.0*cost[9]*cost[10])
//     .build();
    

//     let phi: ANFExpr<Amv> = ANFExpr { terms: vec![t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39, t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59, t60, t61, t62, t63, t64, t65] }; 

//     let exp2_monitor =
//     BayesianExpBuilder::<Amv>::new()
//     .set_expr(phi)
//     .set_sym_const(Amv::Sample(true) ,num_vertices+1000)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(0) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(1) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(2) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(3) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(4) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(5) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(6) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(7) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(8) ,1000)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(9) ,1)
//     .set_trans_const(Amv::Sample(true), Amv::Cost(10) ,1)
//     .build();
    
//     let delta = 0.05;

//     let mut m = BayesianConfInt {
//         exp_monitor, exp2_monitor, delta, verdict: (0.0, 0.0)
//     };

//     let result = _run_bayes_admission_memless(&mut mc, &mapper, &mut vec![&mut m], n);

// }

// // ############################################################################################################################################################
// // ############################################################################################################################################################
// // ############################################################################################################################################################
// // ############################################################################################################################################################
// // ############################################################################################################################################################
// // ############################################################################################################################################################

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


// fn _run_freqopt_lending_memless(
//     mc: &mut Lmmc,
//     mapper: &Lmvm,
//     monitors: &mut Vec<&mut FrequentistOpt<Lmv>>,
//     n: i32
// ) -> Vec<(f64, f64)> {
//     let mut result = vec![(0.0,0.0); monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     let mut is_init = false;
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }

//         let sigma = sigma.unwrap();

//         for j in 0..monitors.len() {
            
//             if !is_init {
//                 monitors[j].init();
//                 is_init = true;
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
//     monitors: &mut Vec<&mut BayesianConfInt<Lmv>>,
//     n: i32
// ) -> Vec<(f64, f64)> {
//     let mut result = vec![(0.0,0.0); monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     let mut is_init = false;
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }
//         let sigma = sigma.unwrap();
//         for j in 0..monitors.len() {
           
//             if !is_init {
//                 monitors[j].init(sigma.clone());
//                 is_init = true;
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
//     monitors: &mut Vec<&mut BayesianConfInt<Amv>>,
//     n: i32
// ) -> Vec<(f64, f64)> {
//     let mut result = vec![(0.0,0.0); monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     let mut is_init = false;
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }

//         let sigma = sigma.unwrap();

//         for j in 0..monitors.len() {
//             if !is_init {
//                 monitors[j].init(sigma.clone());
//                 is_init = true;
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


// fn _run_freqopt_admission_memless(
//     mc: &mut Ammc,
//     mapper: &Amvm,
//     monitors: &mut Vec<&mut FrequentistOpt<Amv>>,
//     n: i32
// ) -> Vec<(f64, f64)> {
//     let mut result = vec![(0.0,0.0); monitors.len()];
//     let mut out_res = (f64::MIN, f64::MAX);
//     let mut out_change = false;
//     let mut start_time = SystemTime::now();
//     let mut end_time = SystemTime::now();
//     let mut is_init = false;
//     let mut duration = end_time.duration_since(start_time).unwrap().as_nanos();
//     for i in 0..n {
//         let sigma = mapper.map(&mc.next().unwrap());
//         if let None = sigma {
//             continue;
//         }

//         let sigma = sigma.unwrap();

//         for j in 0..monitors.len() {
//             if !is_init {
//                 monitors[j].init();
//                 is_init = true;
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
