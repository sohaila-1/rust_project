## – Steps à exécuter

---

### 🟢 STEP 1 — Lancer le serveur principal

👉 Terminal 1 :

```bash
cd ~/pizza-project/archive-2026-03-01T12:59:55/aarch64-apple-darwin

./pizza_factory start \
--recipes-file ~/pizza-project/pizza_factory/recipes/examples.recipes \
--capabilities MakeDough AddBase AddCheese AddBasil Bake AddOliveOil
```

---

### 🟣 STEP 2 — Lancer notre agent

👉 Terminal 2 :

```bash
cd ~/pizza-project/pizza_agent
cargo run --bin pizza_agent
```

👉 Dire :

"Je lance notre agent personnalisé, qui écoute sur le port 9000 et traite les requêtes."

---

### 🔵 STEP 3 — Tester avec le client officiel

👉 Terminal 3 :

```bash
cd ~/pizza-project/archive-2026-03-01T12:59:55/aarch64-apple-darwin

./pizza_factory client --peer 127.0.0.1:8000 order Margherita
```

👉 Dire :

"J’envoie une commande Margherita via le client officiel."

---

### 🎯 RÉSULTAT À MONTRER

👉 Montrer :

```text
Dough
Base
Cheese
Basil
Bake
OliveOil
```

👉 Dire :

"On voit ici toutes les étapes de production de la pizza."

---

### ⚠️ STEP 4 — TEST D’ERREUR (TRÈS IMPORTANT)

👉 Stop serveur (CTRL + C)

👉 Relancer SANS AddBasil :

```bash
./pizza_factory start \
--recipes-file ~/pizza-project/pizza_factory/recipes/examples.recipes \
--capabilities MakeDough AddBase AddCheese Bake
```

---

👉 Relancer client :

```bash
./pizza_factory client --peer 127.0.0.1:8000 order Margherita
```

👉 Montrer :

```text
ERROR: AddBasil not available
```

👉 Dire :

"Si une capacité manque, la production échoue, ce qui montre la dépendance entre les agents."

---

### 🔄 STEP 5 — TEST FORWARD (OPTION BONUS)

👉 Modifier ton client (recipe inconnue) :

```rust
recipe_name: "Pepperoni".to_string()
```

👉 Lancer :

```bash
cargo run --bin client
```

👉 Montrer :

```text
Forwarded to another agent
```
👉 Dire :
"Notre agent simule un forwarding vers un autre agent."

---

### ✅ STEP 6 — CONCLUSION

👉 Dire :

"Nous avons validé un système distribué avec plusieurs nœuds,
une communication réseau en TCP, et une production de pizza complète."

---
