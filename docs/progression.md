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
| 4 à 16 | Non commencé | — | — | — | — |

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

## Questions en suspens

- Aucune question en suspens pour le module 3.
