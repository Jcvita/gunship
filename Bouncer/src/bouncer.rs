use sysinfo::{User, Process, ProcessExt, UserExt, Uid, System, SystemExt};
use std::{thread, process::Command, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, collections::HashMap, time};

// We remove the lifetime annotations from the struct because we will not store references.
pub struct Bouncer {
    pub worksite: System,
    pub flagged_guests_list: Vec<String>,
    pub banned_guests_list: Vec<String>,
    pub flagged_happenings: Vec<String>,
    pub watching_guest_activities: Arc<Mutex<HashMap<Uid, Process>>>,
    stop_watching: Arc<AtomicBool>,
}

impl Bouncer {
    // We take ownership of the Party instead of a mutable reference.
    pub fn hire() -> Self {
        let mut sys = System::new_all();
        sys.refresh_users_list();
        sys.refresh_processes();
        Self {
            worksite: sys,
            flagged_guests_list: Vec::new(),
            banned_guests_list: Vec::new(),
            flagged_happenings: Vec::new(),
            watching_guest_activities: Arc::new(Mutex::new(HashMap::new())),
            stop_watching: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn guest_list(&self) -> HashMap<String, &User> {
        let mut guests: HashMap<String, &User> = HashMap::new();
        for guest in self.worksite.users() {
            guests.insert(guest.name().to_string(), guest);
        }
        guests
    }

    // We clone the User to store it instead of a reference.
    pub fn flag_guest(&mut self, uname: String) {
        self.flagged_guests_list.push(uname);
    }

    // We clone the Process to store it instead of a reference.
    pub fn flag_happening(&mut self, process: Process) {
        let pid: String = process.pid().to_string();
        self.flagged_happenings.push(pid);
    }

    pub fn update_users(&mut self) {
        self.worksite.refresh_users_list();
    }

    pub fn update_processes(&mut self) {
        self.worksite.refresh_processes();
    }

    // // We need to remove the mutable reference to self because we can't send it across threads safely.
    // // Instead, we will use the Arc<Mutex<>> pattern to share and modify data safely between threads.
    // pub fn watch_flagged_guests(&self) {
    //     let watching = Arc::clone(&self.watching_guest_activities);
    //     let stop_watching = Arc::clone(&self.stop_watching);
    //     let worksite = self.worksite.clone(); // Clone the worksite to use in the thread.
    //
    //     thread::spawn(move || {
    //         while !stop_watching.load(Ordering::Relaxed) {
    //             let watched_users = watching.lock().unwrap().keys().cloned().collect::<Vec<_>>(); // Clone the keys to avoid lifetime issues.
    //             for guest in watched_users {
    //                 let procs = worksite.whats_happening_dude(&guest); // Use the cloned worksite.
    //                 for proc in procs {
    //                     watching.lock().unwrap().insert(guest, proc);
    //                 }
    //             }
    //             thread::sleep(time::Duration::from_secs(1));
    //         }
    //     });
    // }

    pub fn get_user_processes(&self, uid: String) -> Vec<&Process> {
        let mut procs: Vec<&Process> = self.worksite.processes().values().collect();
        procs.retain(|proc| proc.user_id().unwrap().to_string() == uid);
        procs
    }

    // This method does not need to be mutable.
    // pub fn stop_watching_guests(&self) {
    //     self.stop_watching.store(true, Ordering::Relaxed);
    // }

    // We clone the User to store it instead of a reference.
    pub fn ban_guest(&mut self, uid: String) {
        // Scope the first mutable borrow
        {
            let usermap = self.guest_list();
            let username = usermap.get(uid.as_str()).unwrap().name().to_string(); // Clone the username to use it later

            // This scope ends here, releasing the first mutable borrow
        }

        // Now we can perform the second mutable borrow
        let procs = self.get_user_processes(uid.clone()); // Clone the uid to avoid borrowing issues
        for proc in procs {
            proc.kill();
            println!("{} - {} has been killed", proc.name(), proc.pid());
        }

        // Perform system commands outside of the borrow scopes
        let username = self.guest_list().get(uid.as_str()).unwrap().name().to_string(); // Re-acquire username if needed
        let _ = Command::new("killall")
            .arg("-u")
            .arg(&username) // Use the cloned username
            .output();
        let _ = Command::new("deluser")
            .arg("--remove-all-files")
            .arg(&username) // Use the cloned username
            .output();

        // Finally, push the uid to the banned list and remove from flagged list
        self.banned_guests_list.push(uid.clone());
        self.flagged_guests_list.retain(|flagged_uid| flagged_uid.ne(&uid));
    }
}
// to pause process - kill with sigstop
// to resume process - kill with sigcont
// to kill process - kill with sigkill