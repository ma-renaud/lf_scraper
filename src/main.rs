use anyhow::Result;

use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

mod config;
mod db;
mod models;
mod scaper;

use crate::db::{get_id_from_thing, DbRemote};
use models::*;
use scaper::scrape_prices;

#[tokio::main]
async fn main() -> Result<()> {
    let scraper_config = config::load_config().unwrap();
    println!("{:?}", scraper_config);

    let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    let db = DbRemote { client };
    db.connect("root", "root").await?;

    db.clear_logs().await; //TODO: Remove after tests

    let known_species: Vec<Specie> = db.get_species().await?;
    let mut data: Vec<ScrapedLog> = Vec::new();
    scrape_prices(&mut data);
    //println!("{} wood species found.", data.len()); //TODO: Remove after tests

    for specie in data {
        //println!("{:?}", specie); //TODO: Remove after tests

        if known_species
            .iter()
            .find(|s| get_id_from_thing(&s.id).unwrap() == specie.id)
            .is_none()
        {
            db.add_specie(specie.id, &specie.name).await?;
            //println!("{} added", specie.name); //TODO: Remove after tests
        }

        db.add_log(specie.id, specie.price).await?;
    }

    let logs: Vec<Log> = db.get_logs().await?;
    for log in logs.iter() {
        println!("{:?}", log);
    }

    Ok(())
}
