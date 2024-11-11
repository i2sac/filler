# Filler docker image

Ce projet consiste en un jeu de stratégie où deux robots s'affrontent en plaçant des pièces aléatoires sur une grille (Anfield). Le but est d'occuper le maximum de surface possible tout en prenant en compte les positions de l'adversaire.

## Instructions Docker
Pour construire l'image Docker, exécutez la commande suivante : ``` docker build -t filler . ```

- **Pour exécuter le conteneur, utilisez cette commande** : ``` docker run -v "$(pwd)/solution":/filler/solution -it filler ```. Cette commande ouvrira un terminal dans le conteneur, et le répertoire solution sera monté dans le conteneur.

- **Exemple de commande dans le conteneur pour exécuter le jeu** : ``` ./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator ```

Votre solution doit être située dans le répertoire `solution` afin qu'elle soit montée et compilée à l'intérieur du conteneur, ce qui permettra de l'exécuter dans le moteur du jeu.

## Notes
- `Terminator` est un robot très fort, donc il n'est pas obligatoire de le battre.
- Pour les Macs M1, utilisez `m1_robots` et `m1_game_engine`.


## Projet Filler - Informations supplémentaires
Le jeu se déroule sur une grille (Anfield) où chaque robot tente de maximiser sa surface en plaçant des pièces `"O"` tout en tentant de bloquer les mouvements de son adversaire. Chaque joueur peut déplacer sa pièce aléatoire dans une position stratégique, en utilisant des algorithmes de convolution pour vérifier les superpositions de pièces sur la grille.

## Fonctionnement du Code
Le projet implémente un algorithme de placement de pièces en utilisant des matrices pour effectuer des calculs de convolution. Les étapes principales sont :

1. **Lecture des Entrées** : Le jeu récupère les informations sur la grille, la taille des pièces, et les positions des joueurs.
2. **Traitement de la Carte et des Pièces** : La grille et la pièce sont transformées en matrices pour faciliter le calcul.
3. **Vérification des Mouvements** : Une convolution est effectuée pour vérifier les positions possibles pour la pièce, en s'assurant qu'elles ne chevauchent pas les adversaires et que la pièce est correctement placée.
4. **Choix de la Meilleure Position** : Le code calcule la position optimale en fonction de la proximité de l'adversaire, pour essayer de bloquer ses avancées.

## Fonctionnalités
- `turn` : Gère chaque tour du jeu, en analysant les données d'entrée, en mettant à jour la grille, et en calculant la meilleure position pour placer la pièce.
- `closer_to_enemy` : Fonction qui choisit la position la plus proche de l'adversaire, afin d'entrer en compétition pour la meilleure zone.

Le but du jeu est de remplir la grille avec le maximum de pièces tout en anticipant les mouvements de l’adversaire et en plaçant les pièces de manière stratégique.

## Auteur
@lodiouf