## le projet

Ce projet est un site qui permet de faire des sondages 
lors de rassemblements militant, de manière anonyme et sécurisé.

Codé suite à la mobilisation lycéenes de novembre 2026,
le but est de proposer une manière plus démocratique de prendre les décisions lors de rassemblements.

## anonymat

L'anonymat est la première des préoccupation, le projet en est techniquement structuré autour. 

- Aucune donnée n'est persisté sur le serveur. Il n'y a pas de base de donnée, pas de logs
- Aucune donnée ne transite sans être chiffrée
- Aucun nom / pseudo n'est demandé, aucunes donnée personnelles, n'est envoyée sur le serveur, seulement les réponses aux votes
- Le serveur ([ag.moxyom.com](https://ag.moxyom.com)) est une machine apartenant à un militant situé chez lui
- Aucun autre service / serveur n'est contacté, même pas pour une police d'écriture, un image ou un icon.
- Le projet est open source, tout le monde peut regarder comment il fonctionne. Chacun est libre d'héberger une instance du site sur son propre serveur

**Point sur la persistance sur le navigateur :** <br>

Lorsqu'un participant rejoins une séance, le site stoque sur son navigateur un **token**, indispensable pour :
- lui permettre de rester dans la séance même après interruption de sa connection internet (souvent instable en AG)
- permettre de fermer les inscriptions à une séance pour ne pas laisser des perturbateur fausser les résultats
- s'assurer que quelqu'un ne vote pas plusieurs fois un fois inscrit

Ce token est **supprimé** à partir du moment ou la **séance se termine**, ou le participant **quitte la séance**, ou **8h après automatiquement**.<br>
Si ces moyens de suppression ne conviennent pas, en se mettant en **navigation privée**, vous aurez 
la certitude qu'il sera supprimé au moment ou vous fermez votre navigateur.
 
## utilisation

**Créer une séance**<br>
- Aller sur [ag.moxyom.com/admin](https://ag.moxyom.com/amin), cliquer sur créer une séance.<br>
- Partagez aux participants le code en haut à gauche de l'écran
- clickez sur le boutton en haut à droite pour cloturer les inscriptions.

