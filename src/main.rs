extern crate postgres;

// use time crate
extern crate time;
extern crate chrono;
use time::PreciseTime;

// multiple producer, single consumer.
use std::sync::mpsc;

// for multithreading
use std::thread;

mod company;
mod leo;
mod nspl;

fn sequential_read() {

    let start = PreciseTime::now();

    company::retrieve_company();
    leo::retrieve_leo();
    nspl::retrieve_nspl();

    let end = PreciseTime::now();
    let duration = start.to(end);

    println!(
        " {} seconds on retrieve all the data SEQUENTIALLY. ",
        duration
    );

}

fn concurrent_read() {

    let start = PreciseTime::now();

    // transmitter and receiver over the channel
    let (leo_tx, leo_rx) = mpsc::channel();
    let (company_tx, company_rx) = mpsc::channel();
    let (nspl_tx, nspl_rx) = mpsc::channel();

    thread::spawn(move || {

        let company = company::retrieve_company();
        company_tx.send(company).unwrap();
    });

    thread::spawn(move || {

        let leo = leo::retrieve_leo();
        leo_tx.send(leo).unwrap();
    });

    thread::spawn(move || {

        let nspl = nspl::retrieve_nspl();
        nspl_tx.send(nspl).unwrap();
    });

    let _leo_channel = leo_rx.recv().unwrap();
    let _company_channel = company_rx.recv().unwrap();
    let _nspl_channel = nspl_rx.recv().unwrap();

    let end = PreciseTime::now();
    let duration = start.to(end);

    println!(
        " {} seconds on retrieve all the data CONCURRENTLY. ",
        duration
    );
}


fn main() {

    concurrent_read();
    sequential_read();

}
