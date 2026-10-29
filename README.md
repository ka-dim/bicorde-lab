# bicorde-lab

`bicorde-lab` est un projet public exclusivement pédagogique consacré à
l’apprentissage de GitHub, GitHub Actions et de l’automatisation CI/CD.

## Objectifs

Ce projet permet de construire progressivement une chaîne CI/CD capable de :

- vérifier du code Rust et Python ;
- exécuter des tests unitaires et des tests d’intégration ;
- construire et tester le projet sur Linux et Windows ;
- produire un exécutable Windows et des artifacts ;
- publier des versions avec Git tags et GitHub Releases ;
- appliquer de bonnes pratiques de sécurité aux workflows.

## Composants

### Serveur Rust

Un petit serveur HTTP développé avec Axum et Tokio. Il exposera progressivement
les routes `/`, `/health` et `/version`.

### Client Python

Un outil en ligne de commande chargé d’interroger le serveur, de vérifier ses
réponses et de retourner un code de sortie non nul en cas d’échec.

## Structure prévue

```text
bicorde-lab/
├── .github/
│   └── workflows/       # Workflows GitHub Actions
├── client/              # Client de diagnostic Python
├── docs/                # Documentation et suivi pédagogique
├── server/              # Serveur HTTP Rust
├── .gitignore
└── README.md
```

## Progression

Les modules 0 à 9 sont terminés et documentés.
Le prochain module à commencer est le module 10 : révision générale et compléments.

Le suivi détaillé est disponible dans [`docs/progression.md`](docs/progression.md).
Le programme de référence, révisé le 16 octobre 2026, est disponible dans
[`docs/programme.md`](docs/programme.md). Il comprend 18 modules numérotés de 0 à 17.
