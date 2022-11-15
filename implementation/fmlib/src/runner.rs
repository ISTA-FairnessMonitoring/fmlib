use std::sync::mpsc::sync_channel;
use std::{thread, time};
use crate::envs::mc::MarkovChain;
use crate::monitor::Monitor;
use log::info;

// This value can be adjusted for different experiments.
const MONITOR_CHANNEL_SIZE: usize = 1000;

pub fn run_mc(
    mut mc: MarkovChain,
    mut monitor: Monitor<i32>,
    n: usize
) {
    // A bounded-size, synchronous channel for thread-level communication.
    // Tx, rx are send and receive ends of the channel, respectively.
    let (tx, rx) = sync_channel::<i32>(MONITOR_CHANNEL_SIZE);
    
    let mc_thread = thread::spawn(move || {
        for i in 0..n {
            // To mimic a real-world system,
            // we delay the Markov chain at each step
            thread::sleep(time::Duration::from_micros(10));
            let sigma = mc.next().unwrap();
            
            // The `try_send` method will send the message in a non-blocking manner;
            // that is, if the channel is full, the chain will not wait until
            // the channel can accept the message, and drops it.
            match tx.try_send(sigma) {
                Ok(_) => {
                    info!("MARKOV_CHAIN: sigma[{}]={}", i, sigma);
                },
                Err(error) => {
                    info!("MARKOV_CHAIN: error={:?}", error);
                }
            }
        }
    });

    let monitor_thread = thread::spawn(move || {
        let mut init = false;
        
        // Read from the channel as long as there are incoming messages
        for (i, sigma) in rx.iter().enumerate() {
            if !init {
                monitor.init(sigma);
                info!("MONITOR:INIT: sigma[{i}]={}", sigma);
                init = true;
            } else {
                let p = monitor.next(sigma);
                info!("MONITOR:NEXT: sigma[{i}]={}, p={:?}", sigma, p);
            }
        }
    });

    // Wait for both threads to finish.
    mc_thread.join().unwrap();
    monitor_thread.join().unwrap();
}
