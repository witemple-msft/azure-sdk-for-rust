use azure_core::prelude::*;
use azure_cosmos::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("Please provide the database name as first parameter");

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    let http_client = azure_core::new_http_client();
    let client = CosmosClient::new(http_client, account.clone(), authorization_token);

    let database_client = client.into_database_client(database_name.clone());

    let response = database_client
        .get_database(Context::new(), GetDatabaseOptions::new())
        .await?;
    println!("response == {:?}", response);

    Ok(())
}
