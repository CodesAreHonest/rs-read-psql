extern crate postgres;

// use time crate
extern crate time;
extern crate chrono;

mod company;
mod leo;
mod nspl;


fn main() {
    company::retrieve_company();
    leo::retrieve_leo();
    nspl::retrieve_nspl();
}
