/// Import des crates et modules nécessaires. Hyper est utilisé pour créer le serveur HTTP,
/// et tokio pour les opérations asynchrones.
use hyper::{Body, Request, Response, Server, Client, Uri, service::{make_service_fn, service_fn}};
use std::convert::Infallible;
use std::net::SocketAddr;

/// Import du module `backends` défini par l'utilisateur pour la sélection et la vérification de la santé des backends.
mod backends;
use backends::{select_backend, maintain_backends_health};


/// Fonction asynchrone pour router les requêtes entrantes vers le backend approprié.
async fn route_request(mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    /// Crée un nouveau client HTTP.
    let client = Client::new();
    /// Sélectionne le backend à utiliser pour cette requête.
    if let Some(endpoint) = select_backend().await {
        /// Construit la nouvelle URI en se basant sur le backend sélectionné et le chemin de la requête originale. 
       let new_uri = format!("{}{}", endpoint, req.uri().path_and_query().map_or_else(|| "".into(), |x| x.as_str())).parse::<Uri>().unwrap();
        *req.uri_mut() = new_uri;
        /// Fait suivre la requête au backend sélectionné et retourne sa réponse.
        client.request(req).await
    } else {
        /// Si aucun backend n'est disponible, retourne une réponse indiquant que le service est indisponible.
        Ok(Response::new(Body::from("Service Unavailable"))) 
    }
    
}

/// Point d'entrée principal de l'application.
#[tokio::main]
async fn main() {
    /// Lance la tâche asynchrone pour maintenir la santé des backends en arrière-plan.
     tokio::spawn(maintain_backends_health());
   
    /// Définit l'adresse d'écoute du load balancer.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    /// Construit le service HTTP qui va gérer les requêtes entrantes.
    let service_builder = make_service_fn(|_conn| async {
      /// Utilise `service_fn` pour router chaque requête à travers la fonction `route_request`. 
       Ok::<_, Infallible>(service_fn(route_request))
    });

    /// Crée et lance le serveur HTTP.
    let server = Server::bind(&addr).serve(service_builder);
    println!("Load Balancer running on http://{}", addr);

    /// Attends que le serveur termine son exécution, gérant les erreurs potentielles.
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
