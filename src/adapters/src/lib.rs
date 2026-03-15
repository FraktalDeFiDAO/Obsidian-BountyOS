pub mod bugcrowd;
pub mod dework;
pub mod gitcoin;
pub mod github;
pub mod hackerone;
pub mod laborx;
pub mod trait_def;
pub mod web_scraper;

pub use bugcrowd::BugcrowdAdapter;
pub use dework::DeWorkAdapter;
pub use gitcoin::GitcoinAdapter;
pub use github::GitHubAdapter;
pub use hackerone::HackerOneAdapter;
pub use laborx::LaborXAdapter;
pub use web_scraper::{WebScraper, ScrapedContent};
pub use trait_def::*;
