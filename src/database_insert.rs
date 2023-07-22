use tokio::net::TcpStream;
use tokio_postgres::{Client, Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

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

    Ok(())
}
