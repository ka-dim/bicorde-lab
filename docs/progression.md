# Progression GitHub Actions et CI/CD

Ce document est le journal pédagogique du projet `bicorde-lab`.

## Objectif personnel

Apprendre à construire, comprendre, diagnostiquer et sécuriser une CI/CD
pour un projet combinant Rust et Python.

## Tableau de progression

| Module | Statut | Notions maîtrisées | Notions à revoir | Exercice effectué | Commit ou PR |
|---|---|---|---|---|---|
| 0 — Diagnostic et préparation | Terminé | Git, GitHub et GitHub Actions ; environnement ; dépôt public ; identité Git ; working tree, index et commit ; `.gitignore` ; premier push | Approfondir la sécurité des secrets dans les modules suivants | Diagnostic de l’environnement, erreurs de `cd` et de heredoc, préparation du premier commit | `f715f31` |
| 1 — Fondamentaux de GitHub Actions | Terminé | YAML ; workflow, job et step ; runner hébergé ; `name`, `on`, `jobs`, `runs-on`, `steps`, `run` et `uses` ; consultation des runs et logs | Approfondir les événements et expressions au module 2 | Workflow minimal, checkout, erreur YAML volontaire et réparation | `388bdeb`, `57cb448`, `fb3c2ae`, `c4e00cd` |
| 2 — Déclencheurs et expressions | Terminé | `push`, `pull_request`, `workflow_dispatch` ; filtres `branches` et `paths` ; expressions ; contexts `github`, `runner`, `env`, `steps` et `job` ; conditions ; codes de sortie | Revoir l’interaction entre filtres et checks obligatoires au module 5 | PR nº 1, déclenchements manuels, filtres, conditions et échecs volontaires | PR nº 1 ; `5cdf45c` ; `36b5717` |
| 3 — CI Rust | Terminé | Package Rust ; `Cargo.lock` ; formatage ; tests ; Clippy ; build ; MSRV ; cache Cargo ; diagnostic des échecs | Approfondir les artifacts et les matrices dans les modules suivants | CI Rust complète, cache miss/hit, échecs volontaires de formatage, test et Clippy | `dd6e752`, `330f8d0`, `723ac3a`, `377c846` |
| 4 — CI Python | Terminé | Environnement virtuel ; dépendances figées ; pytest ; Ruff ; matrice Python 3.11/3.12 ; cache pip ; diagnostic d’installation | Approfondir le verrouillage avec empreintes et la construction du client HTTP | Package Python, quatre tests, matrice CI, cache miss/hit, test et dépendance en échec | `e24ec53`, `96a7926`, `a9f5ac3` |
| 5 — Pull requests et protections | Terminé | Branches de fonctionnalité ; pull requests ; checks obligatoires ; squash merge ; protection de `main` ; résolution locale de conflits | Approfondir les reviews avec plusieurs contributeurs | PR réelles, protection de branche, conflit volontaire et résolution sans perte | PR nº 2 à 5 ; `7f8c107`, `f9a4f6f`, `a451e31`, `429137c` |
| 6 — Artifacts et dépendances entre jobs | Terminé | Isolation des runners ; `needs` ; upload et download d’artifacts ; intégrité SHA-256 ; builds debug et release ; compatibilité des binaires ; rétention | Approfondir les artifacts multiplateformes et les GitHub Releases | Binaire Rust transmis entre deux jobs, téléchargé localement et vérifié sur un runner indépendant | PR nº 7 ; `3b79afa` |
| 7 — Matrice Rust multiplateforme | Terminé | Matrices avec `include` ; `fail-fast` ; runners Linux et Windows ; binaires ELF et PE ; artifacts spécifiques aux plateformes ; steps conditionnels ; dépendances entre matrices | Approfondir les chaînes de jobs indépendantes par plateforme | Builds et vérifications Linux x86-64 et Windows x86-64, avec échec Windows volontaire | PR nº 9 ; `21e3b77` |
| 8 à 16 | Non commencé | — | — | — | — |

## Journal des séances

### 2026-09-26 — Démarrage du module 0

- Nom du projet choisi : `bicorde-lab`.
- Dépôt public créé et cloné.
- Branche initiale : `main`.
- Dépôt distant : `origin`.
- Git, Rust, Cargo, Python et GitHub CLI sont installés.
- GitHub CLI est authentifié.
- L’adresse d’auteur Git actuelle est volontairement conservée.
- Erreur diagnostiquée : tentative d’entrer deux fois dans `bicorde-lab`.

## Points à retenir

- Git gère localement l’historique du projet.
- GitHub héberge les dépôts Git et fournit des outils de collaboration.
- GitHub Actions automatise des tâches en réaction à des événements.
- Une erreur de `cd` ne change pas le dossier courant.
- `user.name` et `user.email` identifient l’auteur d’un commit ; ils ne constituent pas une signature cryptographique.

### Validation du module 0

- Le dépôt public est créé, cloné et synchronisé.
- Le README et la structure prévue sont documentés.
- Les principaux fichiers locaux, générés et sensibles sont ignorés.
- Le premier commit a été poussé sur `main`.
- Le fonctionnement général du working tree, de l’index et des commits est compris.
- En cas de secret exposé, la priorité est sa révocation ou sa rotation.

### 2026-09-27 — Validation du module 1

- Premier workflow déclenché par un `push`.
- Runner Ubuntu hébergé observé dans les logs.
- Différence comprise entre `run` et `uses`.
- Action officielle `actions/checkout@v6` exécutée.
- Erreur YAML volontaire détectée avant la création du runner.
- Run invalide : `36280283993` ; run réparé : `36280711701`.

### 2026-09-27 — Validation du module 2

- Déclencheurs `push`, `pull_request` et `workflow_dispatch` pratiqués.
- PR nº 1 créée, mise à jour automatiquement et fusionnée.
- Filtres de branches et de chemins vérifiés par des runs réels.
- Aucun run créé pour le commit documentaire `3e99840`.
- Expressions et contexts GitHub Actions utilisés.
- Step conditionnel observé en `skipped` puis en `success`.
- Codes `exit 0` et `exit 1` observés.
- Runs en échec : `36327319794` et `36329567933`.
- Workflow restauré : run réussi `36329780200`.

### 2026-09-28 — Validation du module 3

- Package Rust créé avec une bibliothèque, un binaire et deux tests unitaires.
- Vérifications locales pratiquées avec `cargo fmt`, `cargo test`, `cargo clippy`, `cargo check` et `cargo build`.
- Workflow `Rust CI` créé avec des jobs indépendants pour les contrôles courants et la MSRV 1.85.
- Cache Cargo observé en cache miss, sauvegardé, puis restauré avec un cache hit.
- Échec de formatage diagnostiqué dans le run `36366553095`.
- Échec de test unitaire diagnostiqué dans le run `36367412689`.
- Échec Clippy diagnostiqué dans le run `36371090005`.
- Workflow restauré avec succès dans le run `36371449778`.

### 2026-09-28 — Validation du module 4

- Environnement virtuel local créé et correctement ignoré par Git.
- Dépendances de développement enregistrées avec des versions exactes.
- Package `bicorde_client` créé avec quatre tests pytest.
- Ruff utilisé pour le lint et la vérification du formatage.
- Workflow `Python CI` exécuté avec Python 3.11 et Python 3.12.
- Cache pip observé en sauvegarde puis en restauration sur les deux versions.
- Test volontairement cassé dans le run `36494442533`.
- Dépendance inexistante diagnostiquée dans le run `36495966515`.
- Workflow restauré avec succès dans le run `36496214880`.

### 2026-10-02 — Validation du module 5

- PR nº 2 créée depuis une branche de fonctionnalité et fusionnée par squash.
- Squash merge choisi comme seule méthode de fusion du dépôt.
- Suppression automatique des branches distantes fusionnées activée.
- Branche `main` protégée et règles appliquées aux administrateurs.
- Passage par une pull request imposé avec zéro approbation pour le travail solo.
- Checks Python 3.11, Python 3.12, Rust et MSRV rendus obligatoires.
- Force push et suppression de `main` interdits.
- Historique linéaire et résolution des conversations imposés.
- Filtres de chemins retirés des événements `pull_request` pour garantir la création des checks obligatoires.
- Deux branches concurrentes créées depuis le même commit.
- Conflit réel observé dans la PR nº 5 puis résolu localement sans perdre les deux intentions.
- PR nº 3, nº 4 et nº 5 fusionnées par squash.

### 2026-10-04 — Validation du module 6

- Isolation des systèmes de fichiers entre deux jobs observée.
- Binaire Rust construit puis publié avec `actions/upload-artifact`.
- Job indépendant relié avec `needs` et utilisant `actions/download-artifact`.
- Intégrité du transfert confirmée par une empreinte SHA-256 identique.
- Artifact téléchargé localement et identifié comme un exécutable ELF Linux x86-64.
- Incompatibilité entre le binaire Linux x86-64 et macOS ARM64 comprise.
- Différences entre les profils Cargo `debug` et `release` observées.
- Artifact optimisé `server-linux-x86_64-release` produit.
- Artifacts de plusieurs runs observés avec des identifiants distincts.
- Durée de conservation limitée à 14 jours.
- Job `Verify Rust artifact` ajouté aux checks obligatoires de `main`.
- PR nº 7 fusionnée par squash dans le commit `3b79afa`.

### 2026-10-08 — Validation du module 7

- Matrice Rust créée avec deux associations explicites grâce à `include`.
- Binaire Linux `server` construit sur `ubuntu-latest`.
- Binaire Windows `server.exe` construit sur `windows-latest`.
- Artifacts Linux et Windows publiés avec des noms distincts.
- Matrice de vérification exécutant chaque binaire sur son système correspondant.
- Steps Linux et Windows sélectionnés par des conditions.
- Différence entre `include` et un produit cartésien comprise.
- Comportement de `fail-fast: false` observé avec un échec Windows volontaire.
- Dépendance globale envers un job matriciel observée avant expansion de la matrice suivante.
- Checks de packaging et de vérification rendus obligatoires pour les deux plateformes.
- `actions/cache` mise à jour vers la version utilisant Node.js 24.
- PR nº 9 fusionnée par squash dans le commit `21e3b77`.

## Questions en suspens

- Aucune question en suspens pour le module 7.
