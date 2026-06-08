# mini-grep

![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Crates.io](https://img.shields.io/crates/v/mini-grep)

Un outil en ligne de commande écrit en Rust, inspiré de `grep`.

Il permet de rechercher un motif dans un fichier texte et d’afficher les lignes correspondantes rapidement et simplement.

---

## 🚀 Installation

### Depuis crates.io (recommandé)

cargo install mini-grep

### Depuis le code source

git clone https://github.com/gosse123/mini-grep
cd mini-grep
cargo build --release

---

## ▶️ Utilisation

mini-grep <motif> <fichier>

### Exemple :

mini-grep rust test.txt

---

## 📦 Exemple

### Fichier `test.txt`

rust est rapide  
java est lent  
rust est sûr  

### Commande

mini-grep rust test.txt

### Résultat

rust est rapide  
rust est sûr  

---

## 🧠 Fonctionnalités

- Recherche de texte dans un fichier
- Affichage des lignes correspondantes
- CLI simple et rapide avec clap
- Gestion propre des erreurs Rust

---

## 🛠️ Développement

Lancer en mode dev :

cargo run -- rust test.txt

Lancer les tests :

cargo test

Build optimisé :

cargo build --release

---

## 📦 Dépendances

- clap (parsing des arguments CLI)

---

## 📚 Liens

GitHub : https://github.com/gosse123/mini-grep  
Crates.io : https://crates.io/crates/mini-grep

---

## 📜 Licence

MIT