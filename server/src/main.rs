use axum::Router;
use axum::routing::get;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::types::{RecordId, SurrealValue};

#[derive(Debug, SurrealValue)]
struct Name {
    first: String,
    last: String,
}
#[derive(Debug, SurrealValue)]
struct Person {
    title: String,
    name: Name,
    marketing: bool,
}

#[derive(Debug, SurrealValue)]
struct Record {
    id: RecordId,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8080").await?;

    db.signin(Root {
        username: "root".to_string(),
        password: "root".to_string(),
    })
    .await?;

    db.use_ns("main").use_db("main").await?;

    // Create a new person
    let created: Option<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO".to_string(),
            name: Name {
                first: "Tobie".to_string(),
                last: "Morgan Hitchcock".to_string(),
            },
            marketing: true,
        })
        .await?;

    dbg!(created);

    let people: Vec<Record> = db.select("person").await?;
    dbg!(people);

    let app = Router::new().route(
        "/",
        get(|| async { "Hello, world!" }).post(|| async { "Received POST request." }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
