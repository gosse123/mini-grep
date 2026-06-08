# mini-grep

Un mini outil en ligne de commande écrit en Rust, inspiré de `grep`.

Il permet de rechercher un motif dans un fichier texte et d’afficher les lignes correspondantes.

## 🚀 Installation

Clone le projet :
git clone https://github.com/gosse123/mini-grep
cd mini-grep

Compile le projet :
cargo build --release

## ▶️ Utilisation

cargo run -- <motif> <fichier>

Exemple :
cargo run -- rust test.txt

## 🧠 Fonctionnalités

- Recherche de texte dans un fichier
- Retour des lignes correspondantes
- Interface CLI simple avec clap

## 📦 Exemple

Contenu de `test.txt` :
rust est rapide  
java est lent  
rust est sûr  

Commande :
cargo run -- rust test.txt

Résultat :
rust est rapide  
rust est sûr  

## 🛠️ Dépendances

- clap (CLI parser)

## 📜 Licence

MIT