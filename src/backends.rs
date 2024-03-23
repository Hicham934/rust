/// Importations nécessaires pour la gestion des backends et des tâches asynchrones.
use hyper::Client;
use std::sync::Mutex;
use lazy_static::lazy_static;
use tokio::time::{interval, Duration};

/// Utilisation de `lazy_static` pour garder une trace du dernier backend utilisé.
lazy_static! {
    static ref LAST_BACKEND: Mutex<u8> = Mutex::new(0);
}

/// Sélectionne de manière cyclique le backend suivant pour traiter les requêtes.
pub async fn select_backend() -> Option<String> {
    let mut last = LAST_BACKEND.lock().unwrap();
    let backend = if *last == 0 {
        *last = 1;
        "http://127.0.0.1:3000".to_string() // URL du premier serveur.
    } else {
        *last = 0;
        "http://127.0.0.1:3001".to_string() // URL du deuxième serveur.
    };
    Some(backend)
}

/// Vérifie périodiquement la santé des backends.
pub async fn maintain_backends_health() {
    let client = Client::new(); // Client HTTP pour les requêtes de vérification.
    let mut health_check_interval = interval(Duration::from_secs(30));
    loop {
        health_check_interval.tick().await; // Attendre le prochain tick de l'intervalle.
        let health1 = check_backend_health(&client, "http://127.0.0.1:3000").await; // Vérifier la santé du premier serveur.
        let health2 = check_backend_health(&client, "http://127.0.0.1:3001").await; // Vérifier la santé du deuxième serveur.
        println!("Backend 1 health: {}, Backend 2 health: {}", health1, health2);
    }
}

/// Vérifie la santé d'un backend spécifique.
async fn check_backend_health(client: &Client<hyper::client::HttpConnector>, url: &str) -> bool {
    let uri = url.parse::<hyper::Uri>().expect("Failed to parse URI");
    match client.get(uri).await {
        Ok(response) => response.status().is_success(), // Réussite si réponse HTTP est 200-299.
        Err(_) => false, // Échec de la requête.
    }
}
