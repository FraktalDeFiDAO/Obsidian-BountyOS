pub mod trait_def;
pub mod github;
pub mod gitcoin;
pub mod hackerone;
pub mod bugcrowd;
pub mod laborx;
pub mod dework;

pub use trait_def::*;
pub use github::GitHubAdapter;
pub use gitcoin::GitcoinAdapter;
pub use hackerone::HackerOneAdapter;
pub use bugcrowd::BugcrowdAdapter;
pub use laborx::LaborXAdapter;
pub use dework::DeWorkAdapter;
