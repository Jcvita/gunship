// use sysinfo::{ProcessExt, System, SystemExt, Process, Uid, User, UserExt};
// // use users::{get_user_by_uid, UsersCache, User, all_users};
// use std::{thread, time, vec};
//
//
// pub struct Party {
//     party: System,
// }
//
//  impl Party {
//     pub fn new () -> Self {
//         let mut system = System::new_all();
//         system.refresh_all();
//
//         Self {
//             party: system,
//         }
//     }
//
//     pub fn update(&mut self) -> &mut Self {
//         self.party.refresh_all();
//         self
//     }
//
//     pub fn whos_here(&mut self) -> &[User] {
//         self.party.refresh_users_list();
//         self.party.users()
//     }
//
//     pub fn whats_happening(&mut self) -> Vec<&Process> {
//         self.party.refresh_processes();
//         self.party.processes().values().collect()
//     }
//
//     pub fn whats_happening_dude(&mut self, user: &Uid) -> Vec<&Process> {
//         let mut procs: Vec<&Process> = self.whats_happening();
//         procs.retain(|proc| proc.user_id().unwrap() == user);
//         procs
//     }
//
//     pub fn whos_involved(&mut self, process: Process) -> Option<&User> {
//         let uid: Option<&Uid> = process.user_id();
//         match uid {
//             Some(uid) => {
//                 self.party.get_user_by_id(uid)
//             },
//             None => None,
//         }
//     }
// }