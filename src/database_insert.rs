use tokio::net::TcpStream;
use arguments;
use tokio_postgres::{Client, Error, NoTls};
use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<(), Error> {

    color_eyre::install().unwrap();

    let argumentspre = std::env::args();
    let argumentsofthisprogram = arguments::parse(argumentspre).unwrap();

    let postgres = argumentsofthisprogram.get::<String>("postgres").expect("No postgres arg specified!");

    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect(&postgres, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("Connected to database!");

    let makedb = "DROP TABLE IF EXISTS temp_kylers_map; 
    CREATE TABLE temp_kylers_map AS (select date_time, title, address, city, location, neighborhood, categories,event_type , lat, lng, locality, has_video, updates, citizen_link , updated_at, match_level, locality_id, locality_full, locality_district  from citizen_notifications where categories ilike '%Traffic Related%'); 
    BEGIN; ALTER TABLE temp_kylers_map RENAME TO kylers_accident_map; COMMIT;
    ";

    // Now we can execute a simple statement that just returns its parameter.
    let _ = client.execute(makedb, &[]).await;

    println!("Database re-cloned!");

    Ok(())
}