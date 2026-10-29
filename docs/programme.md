# Programme pédagogique de référence

## Objectif et décision de cadrage

Former progressivement l'utilisateur à concevoir, écrire, expliquer, diagnostiquer
et sécuriser une CI/CD GitHub Actions pour un serveur Rust et un client Python.
Le dépôt est public, exclusivement pédagogique, sans dépendance à un projet existant.
Le travail se fait principalement sur macOS ; les livrables seront aussi testés
sur un véritable PC Windows 11 x64 sans installation de Rust.

Ce programme reprend le prompt initial fourni par l'utilisateur et la décision
explicite du 2026-10-16. Les modules 0 à 9 réalisés sont conservés, y compris Docker.
Le module 10 rassemble une révision cumulative et les pratiques manquantes.
Les anciens modules 10 à 16 sont décalés d'un numéro, vers 11 à 17, en conservant
leurs exigences. Le total est de 18 modules, numérotés de 0 à 17.
Aucun module supplémentaire n'est ajouté à ce stade.

Le statut et les preuves sont suivis dans [progression.md](progression.md).
La clôture des modules passés ne signifie pas que les compléments du module 10
ont déjà été pratiqués. La documentation de clôture du module 9 doit être fusionnée
avant de commencer les exercices du module 10.

## Modules 0 à 9 réalisés

| Module | Contenu effectivement travaillé | Preuves principales |
|---|---|---|
| 0 — Diagnostic et préparation | Outils, authentification, dépôt public, structure, hygiène Git, convention de commits | `f715f31` |
| 1 — Fondamentaux de GitHub Actions | YAML, workflows, jobs, steps, runners, logs et erreur YAML volontaire | `388bdeb`, `57cb448`, `fb3c2ae`, `c4e00cd` |
| 2 — Déclencheurs et expressions | Événements, filtres, contexts, conditions et codes de sortie | PR nº 1 ; `5cdf45c`, `36b5717` |
| 3 — CI Rust | Formatage, tests, Clippy, build, MSRV, cache et trois échecs volontaires | `dd6e752`, `330f8d0`, `723ac3a`, `377c846` |
| 4 — CI Python | Environnement virtuel, dépendances figées, pytest, Ruff, versions Python, cache et échecs volontaires | `e24ec53`, `96a7926`, `a9f5ac3` |
| 5 — Pull requests et protections | PR, méthodes de fusion, protections, checks obligatoires et résolution locale de conflit | PR nº 2 à 5 |
| 6 — Artifacts et dépendances entre jobs | Isolation des runners, `needs`, transfert d'artifact, SHA-256, profils, compatibilité et rétention | PR nº 7 ; `3b79afa` |
| 7 — Matrice Rust multiplateforme | `include`, `fail-fast`, Linux/Windows, shells, chemins, packaging et vérification d'artifacts | PR nº 9 ; `21e3b77` |
| 8 — Serveur HTTP et intégration Rust-Python | Axum/Tokio, routes, tests du routeur, client HTTP, intégration Python sur Linux, vérifications HTTP Windows et nettoyage des processus | PR nº 11 ; `6b93bc3` |
| 9 — Docker et configuration réseau | Adresse configurable, multi-stage, contexte, utilisateur, ports, interfaces et intégration Docker en CI | PR nº 13 ; `0eff02d` ; run `38820662160` |

Docker est conservé comme module 9 du parcours effectivement suivi. Le module
« Artifacts » du prompt initial a été partiellement couvert dans les modules 6
et 7 ; ses livrables Windows manquants sont explicitement repris au module 10.

## Module 10 — Révision générale et compléments

Travailler les blocs suivants progressivement, avec un diagnostic initial sans
donner immédiatement les réponses. Réutiliser les acquis plutôt que refaire
intégralement les modules précédents.

### A. Révision cumulative

- Git, branches, PR, squash, conflits et protections ; différence entre code exécuté et check obligatoire.
- Événements, filtres, contexts, expressions, conditions et codes de sortie.
- Rust/Python, MSRV, reproductibilité des dépendances et risques des caches trop larges ou mal identifiés.
- Matrices : produit cartésien, `include`, pratique de `exclude`, nombre de jobs, coût et utilité.
- Différences Bash/PowerShell, chemins, réseau local et Docker, image/conteneur, artifacts/cache.

### B. Jobs, outputs et rapport final

- Faire circuler une donnée avec un output de step puis un output de job.
- Comparer variables d'environnement et outputs ; pratiquer `needs` et le parallélisme.
- Pratiquer `continue-on-error`, son effet sur les résultats et ses dangers.
- Pratiquer `timeout-minutes`, conditions de synthèse et stratégie d'échec.
- Construire un petit rapport final à partir des résultats des jobs Rust et Python.

### C. Intégration réseau robuste sur Linux et Windows

- Exécuter le véritable client Python contre le serveur Rust sur les deux systèmes.
- Attendre activement la disponibilité, vérifier `/health` et `/version` et les codes HTTP.
- Mettre en place et vérifier un arrêt gracieux ; distinguer nettoyage forcé et arrêt gracieux.
- Conserver et rendre récupérables les logs en cas d'échec.
- Provoquer et diagnostiquer en CI : mauvais port, serveur qui ne démarre pas, URL incorrecte et timeout.
- Tester les comportements du client HTTP utiles à ces scénarios avec pytest.
- Vérifier le contrat JSON des routes, y compris `/`, et aligner le code et la documentation avec le prompt initial.
- Garder l'amélioration `RemoteDisconnected` dans les reports tant que l'utilisateur n'a pas choisi de la reprendre.

### D. Livrable Windows complet

- Compiler sur `windows-latest` un exécutable Windows x64.
- Préparer une archive ZIP contenant le livrable et un fichier de version ; examiner son contenu.
- Produire et publier une empreinte SHA-256 du ZIP ; introduire un manifeste si utile.
- Nommer les artifacts, fixer leur rétention et distinguer les empreintes du fichier et de l'archive d'artifact.
- Télécharger sur le PC Windows physique, vérifier l'empreinte en PowerShell et exécuter sans Rust.
- Vérifier `/health` et `/version` et observer l'arrêt du serveur sur ce PC.

### Sortie et clôture

- L'utilisateur explique les notions essentielles et diagnostique les situations d'échec.
- Outputs, rapport, exclusions, intégration Linux/Windows, logs et paquet Windows ont des preuves de pratique.
- Le livrable a été testé sur le PC Windows physique sans Rust ; ne pas remplacer cette preuve par le seul runner Windows.
- Une PR finale de consolidation regroupe les compléments cohérents et leur documentation ; conserver des commits et étapes de revue lisibles.
- Mettre à jour progression, décisions et aide-mémoire avant validation et passage au module 11.

## Module 11 — Tags et versionnement sémantique

Ancien module 10. Commencer manuellement avant d'automatiser :

- Branche, commit et tag ; tags légers et annotés ; création locale et push ciblé.
- Suppression et correction prudentes, avec confirmation avant suppression ou réécriture.
- `MAJOR.MINOR.PATCH`, choix correctif/fonctionnalité/rupture et préversions comme `v0.2.0-beta.1`.
- Cohérence entre tag, version Cargo et version retournée par `/version`.

Sortie : l'utilisateur choisit, crée et publie une version cohérente et explique ses choix.

## Module 12 — GitHub Releases

Ancien module 11 :

- Workflow sur tag `v*`, vérifications du projet et compilation Windows.
- Archive, empreinte, contrôle tag/version et refus d'une publication incohérente.
- Création de Release, pièces jointes et notes de version.
- Artifact de workflow contre asset de Release ; `GITHUB_TOKEN` et `contents: write` limité au besoin.
- Version immuable, prérelease, draft et réparation d'une publication ratée sans réécrire inconsidérément l'historique.
- Télécharger la Release sur le PC Windows, vérifier l'empreinte et exécuter le serveur sans Rust.

Sortie : publication cohérente et test physique du livrable, avec preuves.

## Module 13 — Secrets, variables et environnements

Ancien module 12 ; utiliser uniquement des données fictives :

- Variables de dépôt, secrets de dépôt et d'environnement, environment GitHub.
- Protection d'environnement et validation manuelle d'un déploiement fictif.
- Masquage, limites du masquage, rotation et indisponibilité de secrets pour certains workflows de forks.

Sortie : pratiquer une approbation et expliquer la portée de chaque donnée et ses limites.

## Module 14 — Sécurité des workflows

Ancien module 13 :

- Moindre privilège, permissions par défaut, `contents: read` et élévation limitée au job de Release.
- `pull_request_target`, code non fiable et injections via contexts, titres ou noms de branches.
- Actions tierces : provenance, éditeur, versions, fixation par SHA et Dependabot pour les actions.
- Secrets exposés, scripts téléchargés puis exécutés, provenance et empreintes des outils.
- Identifier et corriger au moins un exemple vulnérable sans vrai secret.

Sortie : revue argumentée et correction pratique d'un workflow vulnérable.

## Module 15 — Réutilisation et maintenance

Ancien module 14 :

- Workflows réutilisables et `workflow_call` ; actions composites si justifiées.
- Factorisation raisonnable, noms, commentaires utiles et journal de décisions.
- `concurrency`, annulation des anciennes CI d'une branche et consommation des ressources.
- Mise à jour des actions et diagnostic d'un workflow devenu lent ; éviter la sur-ingénierie.

Sortie : factorisation testée, concurrence observée et diagnostic d'une lenteur.

## Module 16 — Introduction au service Windows

Ancien module 15 :

- Passer du programme console à un mode service Windows.
- Service Control Manager, compte de service, permissions et configuration.
- Distinguer répertoires d'exécutables, données et journaux.
- Installer/désinstaller avec PowerShell ; démarrer, arrêter et journaliser.
- Vérifier `/health`, l'arrêt gracieux et le redémarrage après reboot ou simulation raisonnable.
- Produire un paquet Windows testable ; ne pas introduire PostgreSQL avant maîtrise du service Rust seul.

Sortie : service testé sur Windows et procédures documentées.

## Module 17 — Projet final autonome

Ancien module 16. Donner un cahier des charges sans solution complète :

- CI Rust/Python sur PR, matrice pertinente et intégration réseau.
- Exécutable Windows, manifeste, empreinte et artifacts.
- Release sur tag et permissions maîtrisées.
- Duplication limitée et documentation permettant la reproduction par un tiers.
- Test du livrable sur un PC Windows sans Rust.

Sortie : réalisation autonome puis revue professionnelle complète de la solution.

## Méthode et livrables transversaux

- Un module à la fois ; explication, pratique par l'utilisateur, résultats, diagnostic, questions et validation.
- En cas d'erreur : lecture, hypothèse de l'utilisateur, indice, puis correction si nécessaire.
- En fin de module : récapitulatif, 3 à 5 questions, exercice, erreur à diagnostiquer, checklist et proposition de commit.
- Révision cumulative environ tous les quatre modules ; vérifier les sources officielles pour les informations évolutives.
- Préciser terminal et effets des commandes ; confirmation avant opérations destructrices ou réécriture d'historique.
- Garder le dépôt public et l'offre gratuite ; vérifier les règles actuelles avant de consommer des ressources significatives.
- Livrables finaux : README, documentation CI/CD, schéma des workflows, fiche de diagnostic, fiche tags/Releases, fiche sécurité, checklists de publication et de test Windows, journal des décisions et aide-mémoire des commandes.
