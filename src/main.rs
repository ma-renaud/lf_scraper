mod config;
mod db;
mod lf_scraper;
mod models;

use anyhow::Result;
use crate::db::{get_id_from_thing, DbRemote};
use models::*;

#[tokio::main]
async fn main() -> Result<()> {
    let scraper_config = config::load_config().unwrap();
    println!("{:?}", scraper_config);

    let db = DbRemote::new(
        &scraper_config.surreal_address,
        &scraper_config.surreal_username,
        &scraper_config.surreal_password,
        &scraper_config.surreal_namespace,
        &scraper_config.surreal_database,
    )
    .await?;

    db.clear_logs().await; //TODO: Remove after tests

    let known_species: Vec<Specie> = db.get_species().await?;
    let mut data: Vec<ScrapedLog> = Vec::new();
    lf_scraper::scrape_prices(&mut data, &scraper_config.crawlio_key);

    for specie in data {
        if known_species
            .iter()
            .find(|s| get_id_from_thing(&s.id).unwrap() == specie.id)
            .is_none()
        {
            db.add_specie(specie.id, &specie.name).await?;
        }

        db.add_log(specie.id, specie.price).await?;
    }

    //TODO: Remove after tests
    let logs: Vec<Log> = db.get_logs().await?;
    for log in logs.iter() {
        println!("{:?}", log);
    }
    //TODO: Remove after tests

    Ok(())
}
