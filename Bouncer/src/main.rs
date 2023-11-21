use sysinfo::{ProcessExt, UserExt};
use bouncer::Bouncer;
pub mod bouncer;

fn main() {
    let mut bouncer = Bouncer::hire();
    // let possible_shells = vec!["bash", "sh", "dash", "rbash", "zsh", "fish", "csh", "tcsh", "ksh", "ash", "busybox", "sysctl"];

    let guests = bouncer.guest_list();

    println!("users:");
    // // print the name of each guest
    for guest in guests.keys() {
        println!("{}", guest);
    }

    println!("\nuser processes:");
    for process in bouncer.get_user_processes(guests.get("jcvit").unwrap().id().to_string()) {
        println!("{}", process.name());
    }

    // loop {
    //     system.refresh_processes();

    //     for (pid, process) in system.processes() {

    //     }
    // }
}
