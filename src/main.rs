use tabled::{Table, Tabled};
mod fetch;
pub use crate::fetch::fetch_seasonal_anime;
mod types;
pub use crate::types::*;

#[derive(Tabled)]
pub struct AnimeTableLayout {
    name: String,
    broadcast: String,
    airing: String,
}

impl AnimeTableLayout {
    fn new(name: &str, broadcast: &str, airing: &str) -> Self {
        Self {
            name: name.to_string(),
            broadcast: broadcast.to_string(),
            airing: airing.to_string(),
        }
    }
}

fn main() -> Result<(), reqwest::Error> {
    let result = fetch_seasonal_anime().unwrap();

    let table = Table::new(result).to_string();
    println!("{}", table);

    Ok(())
}
