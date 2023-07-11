mod config;
mod db;
mod lf_scraper;
mod models;

use crate::db::{get_id_from_thing, DbRemote};
use anyhow::Result;
use models::*;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<()> {
    let scraper_config = config::load_config().unwrap();

    let db = DbRemote::new(
        &scraper_config.surreal_address,
        &scraper_config.surreal_username,
        &scraper_config.surreal_password,
        &scraper_config.surreal_namespace,
        &scraper_config.surreal_database,
    )
    .await?;

    let known_species: Vec<Specie> = db.get_species().await?;

    let data =
        spawn_blocking(move || lf_scraper::scrape_prices(&scraper_config.crawlio_key)).await?;

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

    Ok(())
}
