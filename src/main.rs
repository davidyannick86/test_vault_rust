use dotenv::dotenv;
use std::env;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

async fn create_client() -> Result<VaultClient, Box<dyn std::error::Error>> {
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

    Ok(client)
}

async fn get_key(
    client: &VaultClient,
    key_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Lecture du secret
    let secret: std::collections::HashMap<String, String> =
        kv2::read(client, "secret", "test/secret").await?;

    Ok(secret[key_name].clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client().await?;

    // Lecture du secret
    let secret = get_key(&client, "tagada").await;

    match secret {
        Ok(_) => println!("Secret tagada lu avec succès : {:?}", secret.unwrap()),
        Err(e) => println!("Erreur lors de la lecture du secret tagada: {:?}", e),
    }

    Ok(())
}
