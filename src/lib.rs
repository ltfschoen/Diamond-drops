//! ![uml](ml.svg)

// External crates
extern crate ethereum_types;
extern crate tiny_keccak;

// Module declarations
pub mod cli;
pub mod proposer;
pub mod notary;
pub mod smc_listener;
pub mod collation;
pub mod message;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn run(config: cli::config::Config) -> () {
    /// The main function to run the node.  
    /// 
    /// # Inputs
    /// 
    /// config - A struct containing the configuration values for the client

    let proposer_vector = vec![1, 2, 3];
    let notary_vector = vec![1, 2, 3];

    println!("Client Mode: {:?}", config.mode);

    // Create the channel for the notary and smc listener
    let (notary_sender, notary_receiver) = mpsc::channel();

    // Create the SMC listener
    let smc_listener = smc_listener::SMCListener::new(notary_sender);

    // Create the proposer and notary
    let mut proposer = proposer::Proposer::new();
    let mut notary = notary::Notary::new(notary_receiver);

    // Get thread handles
    let mut proposer_handle: Option<thread::JoinHandle<(u32)>> = None;
    let mut notary_handle: Option<thread::JoinHandle<(u32)>> = None;

    match config.mode {
        cli::config::Mode::Proposer => {
            println!("Running as a proposer");
            // Start a thread to run the proposer
            proposer_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Proposer.value())
                .spawn(move || -> u32 {
                    // `move` transfers "ownership" from Main Thread
                    // to child thread for any variables used in child thread
                    show_thread_welcome_message();
                    let count = show_thread_sleep_count(proposer_vector);
                    proposer.run();

                    count
                })
                .expect("Failed to spawn a proposer thread")
            );
        },
        cli::config::Mode::Notary => {
            println!("Running as a notary");
            // Start a thread to run the notary
            notary_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Notary.value())
                .spawn(move || -> u32 {
                    // `move` transfers "ownership" from Main Thread
                    // to child thread for any variables used in child thread
                    show_thread_welcome_message();
                    let count = show_thread_sleep_count(notary_vector);

                    // notary.run();

                    count
                })
                .expect("Failed to spawn a notary thread")
            );
        },
        cli::config::Mode::Both => {
            println!("Running as both a proposer and notary");
            // Start threads for both proposer and notary
            proposer_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Proposer.value())
                .spawn(move || -> u32 {
                    // `move` transfers "ownership" from Main Thread
                    // to child thread for any variables used in child thread
                    show_thread_welcome_message();
                    let count = show_thread_sleep_count(proposer_vector);

                    proposer.run();

                    count
                })
                .expect("Failed to spawn a proposer thread")
            );
            notary_handle = Some(thread::Builder::new()
                .name(cli::config::Mode::Notary.value())
                .spawn(move || -> u32 {
                    // `move` transfers "ownership" from Main Thread
                    // to child thread for any variables used in child thread
                    show_thread_welcome_message();
                    let count = show_thread_sleep_count(notary_vector);

                    // notary.run();

                    count
                })
                .expect("Failed to spawn a notary thread")
            );
        }
    }

    let mut final_sum: u32 = 0;
    let proposer_sum: u32 = match proposer_handle {
        Some(handle) => {
            match handle.join() {
                Ok(sum) => {
                    println!("Final value after successful proposer thread join {:?}", sum);
                    sum
                },
                Err(e) => {
                    panic!("Failed proposer thread join {:?}", e);
                }
            }
        },
        None => {
            println!("No proposer thread handler present");
            0
        }
    };

    final_sum += proposer_sum;

    let notary_sum: u32 = match notary_handle {
        Some(handle) => {
            match handle.join() {
                Ok(sum) => {
                    println!("Final value after successful notary thread join {:?}", sum);
                    sum
                },
                Err(e) => {
                    panic!("Failed notary thread join {:?}", e);
                }
            }
        },
        None => {
            println!("No proposer thread handler present");
            0
        }
    };

    final_sum += notary_sum;

    for i in 1..3 {
        println!("Main thread: Sleep number {}", i);
        // Force thread to sleep for duration to allow different thread to run
        thread::sleep(Duration::from_millis(1));
    }

    println!("Final sum: {}", final_sum);
    assert_eq!(final_sum, 4);
}

fn show_thread_sleep_count(proposer_vector: Vec<i32>) -> u32 {
    println!("{} thread with vector: {:?}", thread::current().name().unwrap(), proposer_vector);
    let mut count: u32 = 0;
    for i in 1..3 {
        count += 1;
        println!("{} thread: Sleep number {} with count {}", thread::current().name().unwrap(), i, count);
        // Force thread to sleep for duration to allow different thread to run
        thread::sleep(Duration::from_millis(1));
    }
    count
}

fn show_thread_welcome_message() {
    println!("Welcome to {:?} thread: ", thread::current().name().unwrap());
}



