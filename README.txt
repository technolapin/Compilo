* Projet de Chomicki Clément
** Prérequis
Le projet a été réalisé avec cargo v1.41.0.

** Le Parser
Le parser a été réalisé avec la librairie lalrpop.
Toutes les grammaires se trouvent dans le fichier src/parser.lalrpop.
Les expressions ont été organisées en niveau de précédences, en se
basant sur ce qui se fait en C
https://fr.cppreference.com/w/cpp/language/operator_precedence
Cela permet de ne pas trop se casser la tête pour savoir quel
opérateur doit avoir quel priorité sur quel autre, parce que on ne va
pas réinventer l'eau tiède. Aussi, commencer dès le début avec autant
de niveaux de précédences permet de greffer n'importe quel opérateur
que l'on voudrait supporter sans avoir à modifier la structure du
code.

** L'AST
On distingue deux structures majeures: les expressions et les
séquences d'expressions (afin de faciliter les choses).
Les expressions seront générées par une énum Expression et les
séquences par une structure Seq.
On utilise des pointeurs Box pour casser la déclaration réccursive
d'Expression.
Les opérateurs unaires, binaires, et les opérateurs sur les variables
seront listés sous forme d'enum simples.
Le If, let LetIn, le For, le While ont droit à leur variantes dédiées
de l'enum Expression car ils ne partagent pas de pattern de données
similaires.
On supporte 4 types: nil, int, string et bool. Les valeures sont
représentées par Terminal.
Les identifier (uniquement des variables ici), sont représentés par la
structure Identifier.
On désigne par “bloc” la séquence d'instruction contenue enre deux
parenthèses.

** Le Pretty printer
On implémente Display pour toutes les structures présentes dans
l'arborecence de l'AST, afin d'obtenir le pretty print avec un simple
appel de println!() ou format!().

** Le test du parseur
Tous les éléments de l'AST disposent d'une méthode associée "random",
afin de créer des AST aléatoires, certes non bindables et ne
respectant pas les types, mais parsable par notre parser et pretty
printable.
Un des tests de lib.rs est la construction d'un ast aléatoire, sa
conversion en str, le parsing de cette str, et la comparaison des deux
AST ainsi obtenus, ce itéré 1000 fois. On a alors une garantie forte
du fonctionnement de notre parseur.

** Le binder-type checker
Le binding et le type checking se font en même temps, par parcour
récursif de l'AST.
Un problème majeur ayant survenu est le fait que l'ordre de parcours
des HashMap ne soit pas garanti, or les déclarations des LetIn s étant
contenues dans des HashMap pour bénéficier de l'accès aléatoire sont
parfois dépendantes les unes des autres, et changer l'ordre de
parcours revient donc à ne pas pouvoir binder certaines variables.
Pour remédier à ce problème, la structure VarsRegister dispose, en plus
de son HashMap mettant en relation Identifier et Expression, d'un
Vecteur des Identifiers clés du HashMap. Le Vec étant ordoné, on a
donc maintennant moyen de garder l'ordre de déclaration des variables.
Le binding et le type checking ne nécessitent pas de garder d'autres
informations que les scopes et les types, ainsi notre structure
Binder, qui garde les informations des scopes le temps du type
checking/binding, ne contiendra que des Identifier et des Type.

** Plus d'opérateur
En plus des opérateurs demandés, on implément aussi l'opérateur unaire
"+", la négation binaire et bit à bit, le modulo, le "et" et le "ou"
bit à bit et binaire, le Xor (bit à bit), le décalage à droite, le
décalage à gauche, et les divers opérateurs arithmétiques d'assignement
(comme le += ou le >>=).

** Le désucrage
On a plusieurs éléments de l'ast que l'on veut remplacer:
notemment les fors par des whiles et les incrémentations et autres
opérateurs sur les identifier par des assignations.
On pourrait faire un match complet par désucrage, mais ça seraît
beaucoup de répétition de code pour pas grand chose (car on ne peut
pas juste copier les expressions qui ne sont pas à remplacer, car on
doit les parcourrir aussi).
Ainsi, on écrit une méthode “propagate” pour les expressions et les
seq, prennant en paramètre une closure qui sera appliquée sur tous les
descendants potentiel de l'expression courrante.
Ainsi, cela permet de remplacer les expressions souhaitées, et de
propager les modifications à travers les noeuds de l'AST à ne pas
toucher.
Théoriquement, les if sans corps elses se font désucrer en if then
else, mais dans l'état, le parseur ne gère pas les if sans else.

** L'exécution
Similairement à la structure Binder, on créé une structure Context,
qui permettera d'empiler de de dépiler des scopes de variables lors de
l'exécution.
Étant donnée que l'AST que l'on exécute est déjà passé par le
binder/type-checker et par le désucrage, on peut considérer les
variables et les types corrects. On fait donc juste un simple filtrage
pour réduire l'expression en Terminal. 

