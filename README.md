# 🍕 Projet : Chaîne de Production de Pizzas Distribuée

## 📌 Description

Ce projet consiste à développer un **agent distribué en Rust** capable de s'intégrer dans une chaîne de production de pizzas.

Le système est basé sur une architecture **décentralisée**, où plusieurs agents collaborent pour exécuter différentes étapes d'une recette (préparation, garniture, cuisson, etc.).

Chaque agent :

* reçoit des requêtes via TCP
* traite une partie de la recette
* ou transmet la tâche à un autre agent si nécessaire

---

## 🧩 Architecture du projet

```
pizza-project/
│
├── pizza_agent/        # Notre agent Rust
│   ├── main.rs         # Serveur TCP + gestion réseau
│   ├── client.rs       # Client pour tester les requêtes
│   ├── handler.rs      # Logique métier (traitement des recettes)
│   ├── protocol.rs     # Définition des messages (Request / Response)
│
├── pizza_factory/      # Binaire fourni (simulateur du réseau)
│   ├── recipes/
│   │   └── examples.recipes
│
└── README.md
```

---

## ⚙️ Fonctionnement

1. Le **client** envoie une commande (`Order Margherita`)
2. La **pizza_factory** distribue les tâches
3. Notre **agent** :

   * reçoit une requête
   * traite les étapes qu'il connaît
   * renvoie une réponse

---

## 🍕 Gestion des recettes (BONUS)

Au lieu de coder les recettes en dur, nous avons implémenté un système dynamique :

* Lecture du fichier :

```
pizza_factory/recipes/examples.recipes
```

* Parsing automatique des recettes
* Extraction des noms de pizzas

👉 Exemple :

```
Margherita =
    MakeDough
    -> AddBase(base_type=tomato)
    -> [AddCheese(amount=2), AddBasil(leaves=3)]
    -> Bake(duration=5)
```

✔ Notre agent reconnaît toutes les recettes sans modification du code.

---

## 🚀 Lancer le projet (TEST LOCAL)

### 🟢 1. Lancer pizza_factory

```bash
cd archive-2026-03-01T12:59:55/aarch64-apple-darwin

chmod +x pizza_factory

./pizza_factory start \
--recipes-file ~/pizza-project/pizza_factory/recipes/examples.recipes \
--capabilities MakeDough AddBase AddCheese AddBasil Bake AddOliveOil
```

---

### 🔵 2. Lancer notre agent

```bash
cd ~/pizza-project/pizza_agent
cargo run --bin pizza_agent
```

---

### 🟣 3. Tester avec le client

```bash
./pizza_factory client --peer 127.0.0.1:8000 order Margherita
```

---

## ✅ Résultat attendu

```
Order completed successfully
```

avec les étapes :

```
Dough prepared
Base added
Cheese added
Basil added
Baked
```

---

## 🧪 Tests réalisés

* ✔ Connexion TCP client / agent
* ✔ Envoi et réception de requêtes
* ✔ Parsing dynamique des recettes
* ✔ Traitement de plusieurs pizzas (Margherita, Funghi, etc.)
* ✔ Gestion des erreurs réseau

---

## 🛠 Technologies utilisées

* Rust (édition 2021)
* TCP (std::net)
* Sérialisation CBOR (`ciborium`)
* Multi-threading (`std::thread`)
* Parsing DSL

---

## 🔍 Reverse Engineering

Nous avons utilisé :

* Wireshark / tcpdump
* Analyse des échanges TCP
* Identification du format des messages

---

## 🎯 Améliorations possibles

* Implémentation complète du DSL (exécution réelle des étapes)
* Système de forwarding vers d'autres agents
* Optimisation du routage
* Interface graphique (dashboard)

---

## 👨‍💻 Auteur

* Sohaila

---

## 🏁 Conclusion

Ce projet nous a permis de :

* comprendre les systèmes distribués
* manipuler les communications réseau bas niveau
* appliquer les concepts fondamentaux de Rust
* implémenter une logique dynamique basée sur un DSL

---
