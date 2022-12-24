use fmlib::{
    op,
    runner,
    util,
    // experiments,
    // experiments_2,
    monitors::{
        frequentist::builder::FrequentistBuilder,
        fafrequentist::builder::FaFrequentistBuilder,
        bayesian::builder::BayesianBuilder,
    },
    monitor::Monitor
};

fn main() {
    // For logging purposes
    env_logger::init();
    // experiments_2::run();
    // _run_frequentist_mc();
    // _run_fafrequentist_mc();
  
}

fn _run_frequentist_mc() {
    let mc = util::markov_chain_7_state();
    // Build a monitor by specifying the syntax tree of the formula
    let f = FrequentistBuilder::<i32>::new()
    .set_delta(0.05)
    .add_freq(1, 2) // 0: p_{1, 2}
    .add_freq(4, 5) // 1: p_{4, 5}
    .add_bin_op(0, 1, op::BinOp::Sum) // 2: p_{1, 2} + p_{4, 5} = 1.1
    .build();

    let monitor = Monitor::Frequentist(f);
    runner::run_mc(mc, monitor, 10000);
}

fn _run_fafrequentist_mc() {
    let mc = util::markov_chain_7_state();
    let f = FaFrequentistBuilder::<i32>::new()
    .add_freq(1, 2) // 0: p_{1, 2}
    .add_freq(4, 5) // 1: p_{4, 5}
    .add_bin_op(0, 1, op::BinOp::Sum) // 2: p_{1, 2} + p_{4, 5} = 1.1
    .set_delta(0.05)
    .build();

    let monitor = Monitor::FairnessAwareFrequentist(f);
    runner::run_mc(mc, monitor, 10000);
}

fn _run_bayesian_mc() {
    let mc = util::markov_chain_7_state();
    let f = BayesianBuilder::<i32>::new()
        .add_freq(1, 2, 1.0, 2.0) // 0: p_{1, 2}
        .add_freq(4, 5, 1.0, 2.0) // 1: p_{4, 5}
        .add_bin_op(0, 1, op::BinOp::Sum) // 2: p_{1, 2} + p_{4, 5} = 1.1
        .build();

    let monitor = Monitor::Bayesian(f);
    runner::run_mc(mc, monitor, 10000);
}
