# Progression GitHub Actions et CI/CD

Ce document est le journal pédagogique du projet `bicorde-lab`.

## Objectif personnel

Apprendre à construire, comprendre, diagnostiquer et sécuriser une CI/CD
pour un projet combinant Rust et Python.

## Tableau de progression

| Module | Statut | Notions maîtrisées | Notions à revoir | Exercice effectué | Commit ou PR |
|---|---|---|---|---|---|
| 0 — Diagnostic et préparation | En cours | Différence générale entre Git, GitHub et GitHub Actions ; diagnostic des outils ; création et clonage du dépôt | Identité Git, structure du dépôt, hygiène des fichiers, conventions de commit | Vérification de l’environnement et diagnostic d’un `cd` incorrect | À venir |
| 1 à 16 | Non commencé | — | — | — | — |

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

## Questions en suspens

- Comment structurer proprement les composants Rust et Python ?
- Quels fichiers locaux faut-il exclure de Git ?
- Quelle convention de commits utiliser ?
