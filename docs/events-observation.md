# Observation des événements GitHub Actions

Ce document consigne les déclenchements observés pendant le module 2.

| Opération | Événement attendu | Résultat observé |
|---|---|---|
| Push de la branche source | `push` | Succès — run `36307961302` |
| Ouverture de la pull request vers `main` | `pull_request` | Succès — run `36308207301` |
| Mise à jour après le filtre de branches | `pull_request` uniquement | Succès — run `36309492180` |
| Modification uniquement de `docs/` sur `main` | Aucun run attendu | À vérifier |
