## Le projet

Ce projet est un site qui permet de faire des sondages 
lors de rassemblements militants, de manière anonyme et sécurisée.

Codé suite à la mobilisation lycéenne de novembre 2026,
le but est de proposer une manière plus démocratique de prendre les décisions lors de rassemblements.

## Anonymat

L'anonymat est la première des préoccupations ; le projet en est techniquement structuré autour. 

- Aucune donnée n'est persistée sur le serveur. Il n'y a pas de base de données, pas de logs
- Aucune donnée ne transite sans être chiffrée
- Aucun nom/pseudo n'est demandé, aucune donnée personnelle n'est envoyée sur le serveur, seulement les réponses aux votes
- Le serveur ([ag.moxyom.com](https://ag.moxyom.com)) est une machine appartenant à un militant située chez lui
- Aucun autre service ou serveur n'est contacté, même pas pour une police d'écriture, une image ou une icône
- Le projet est open source, tout le monde peut regarder comment il fonctionne. Chacun est libre d'héberger une instance du site sur son propre serveur

**Point sur la persistance sur le navigateur :** 

Lorsqu'un participant rejoint une séance, le site stocke sur son navigateur un **token**, indispensable pour :
- lui permettre de rester dans la séance même après interruption de sa connexion internet (souvent instable en AG)
- permettre de fermer les inscriptions à une séance pour ne pas laisser des perturbateurs fausser les résultats
- s'assurer que quelqu'un ne vote pas plusieurs fois une fois inscrit

Ce token est **supprimé** à partir du moment où la **séance se termine**, ou le participant **quitte la séance**, ou **8h après automatiquement**.
Si ces moyens de suppression ne conviennent pas, en utilisant la **navigation privée**, vous aurez 
la certitude qu'il sera supprimé au moment où vous fermerez votre navigateur.
 
## Utilisation

**Créer une séance**
- Aller sur [ag.moxyom.com/admin](https://ag.moxyom.com/amin), cliquer sur « Créer une séance ».
- Partagez aux participants le code en haut à gauche de l'écran
- Cliquez sur le bouton en haut à droite pour clôturer les inscriptions.

