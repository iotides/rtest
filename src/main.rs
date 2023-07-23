
use warp::Filter;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / "world").map(|| {
        
        warp::reply::html("<h1><b><i>Hello, World!</i></b></h1>")
    });

    let client = connect().await.unwrap();

    if let Err(e) = fetch_users(&client).await {
        eprintln!("error fetching users: {}", e);
    }

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
async fn connect() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=12345 dbname=shif",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

async fn fetch_users(client: &tokio_postgres::Client) -> Result<(), Error> {
    let rows = client
        .query("SELECT * FROM mark", &[])
        .await?;

    for row in rows {
        let name: &str = row.get("name");
        println!("User: {}", name);
    }

    Ok(())
}




