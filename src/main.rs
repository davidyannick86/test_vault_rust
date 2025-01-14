use dotenv::dotenv;
use std::env;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charge le fichier .env
    dotenv().ok();

    // Récupère les variables d'environnement
    let token = env::var("VAULT_TOKEN").expect("VAULT_TOKEN must be set in .env file");
    let addr = env::var("VAULT_ADDR").expect("VAULT_ADDR must be set in .env file");

    // Création du client Vault avec les variables d'environnement
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(addr)
            .token(token)
            .build()?,
    )?;

    // Lecture du secret
    let secret: std::collections::HashMap<String, String> =
        kv2::read(&client, "secret", "test/secret").await?;

    // Pour accéder à la valeur spécifique "tagada"
    if let Some(value) = secret.get("tagada") {
        println!("Contenu du secret 'tagada': {:?}", value);
    }

    Ok(())
}
