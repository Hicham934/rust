use hyper::{Body, Response, Server, service::{make_service_fn, service_fn}};
use std::net::SocketAddr;


/// Fonction pour gérer les requêtes HTTP reçues par le serveur.
async fn handle_request(_req: hyper::Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("Réponse du serveur 1")))
}


/// Point d'entrée du serveur 1.
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let service_maker = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(service_maker);
    println!("Serveur 1 sur http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Erreur serveur: {}", e);
    }
}
