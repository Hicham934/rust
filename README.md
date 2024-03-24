# Load Balancer en Rust ![Uploading FerrisRustGIF.gif…]()

Ce projet est un exemple simple de load balancer écrit en Rust, utilisant hyper pour la création du serveur HTTP et tokio pour l'asynchronisme. Il route de manière cyclique les requêtes entrantes vers deux serveurs backends, en vérifiant périodiquement leur santé.

# Fonctionnalités
Rotation cyclique entre deux serveurs backends.
Vérification de la santé des serveurs backends toutes les 30 secondes.
Routage des requêtes HTTP vers les backends sélectionnés.

# Prérequis
Rust (Edition 2021)
Cargo (intégré avec Rust)

# Installez le compilateur C (gcc) et les outils de compilation nécessaires en utilisant apt :
```
sudo apt update
sudo apt install build-essential
```


# Installation
Clonez le dépôt du projet :
```
git clone git@github.com:Hicham934/rust.git
```
Naviguez dans le dossier du projet :
```
cd rust
```

Compilez le projet avec Cargo :
```
cargo build --release
```

# Utilisation
Lancez le load balancer :
```
cargo run --bin loadbalancer
```

envoyer une requête à un serveur local fonctionnant sur le port 8080 et d'afficher la réponse dans votre terminal : 
```
curl http://127.0.0.1:8080/
```
Les serveurs backends peuvent également être lancés pour tester le routage :

Serveur 1 :
```
cargo run --bin serveur1
```

Serveur 2 :
```
cargo run --bin serveur2 (modifié)
```



