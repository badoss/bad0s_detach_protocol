use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
// use surrealdb::RecordId;
use surrealdb::Surreal;

pub async fn _connect_to_db() -> Surreal<Client> {
    // Connect to the SurrealDB instance
    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();

    // Sign in with root credentials
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();

    // Select the namespace and database
    db.use_ns("test").use_db("test").await.unwrap();

    db
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection() {
        // Connect to the SurrealDB instance
        let _db = _connect_to_db().await;

        // If we reach this point, the connection is successful
        assert!(true);
    }
}
