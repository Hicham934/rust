#[cfg(test)]
mod tests {
    use super::*;
    use hyper::body::HttpBody as _;
    use std::str;

    #[tokio::test]
    async fn test_load_balancer_response() {
        let _ = tokio::spawn(main()); // Lance le load balancer dans un thread séparé.

        // Attends que le serveur soit prêt.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Fait une requête au load balancer.
        let client = Client::new();
        let uri = "http://127.0.0.1:8080".parse::<Uri>().unwrap();
        let mut resp = client.get(uri).await.unwrap();

        // Vérifie que la réponse est celle attendue.
        let body = resp.body_mut().data().await.unwrap().unwrap();
        let body_str = str::from_utf8(&body).unwrap();
        assert!(body_str.contains("Réponse du serveur"));
    }
}
