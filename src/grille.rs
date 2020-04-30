use std::collections::HashSet;
use std::fs::File;
// Les conteneurs utilisé pour modéliser la formule : littéral, conteneur interne, médian et
// externe
type Litt = i32;
type CInt = Vec<Litt>;
type CMed = Vec<CInt>;
type Formule = Vec<CMed>;

/// Grille représente une grille de kurotto (ie. une taille et des contraintes)
/// # Exemple
/// ```
/// let g = Grille::new(3);
/// ```
pub struct Grille {
    /// Taille de la grille
    taille: u8,
    /// Les contraintes sont de la forme (i, j, p) avec i la colonne, j la ligne et p la valeur de
    /// la contrainte
    contraintes: Vec<(u8, u8, i8)>,
}

impl Grille {
    /// Construit une grille dont la taille est en argument sans aucune contrainte
    pub fn new(taille: u8) -> Grille {
        Grille {
            taille,
            contraintes: Vec::<(u8, u8, i8)>::new(),
        }
    }

    /// Ajoute la contrainte (i,j, p) à la grille
    pub fn add_contrainte(&mut self, i: u8, j: u8, p: i8) {
        self.contraintes.push((i, j, p));
    }

    ///Renvoie une formule représentant la modélisation
    fn to_formule(&self) -> Formule {
        let mut formule = Formule::new();
        for (i, j, p) in self.contraintes.iter() {
            if *p == -1 {
                formule.push(vec![
                    vec![-self.to_litt(*i, *j)],
                    vec![-self.contrainte_to_litt(*i, *j, *p)],
                ]);
            } else if *p == 0 {
                let sous_sous_formule: CInt = self
                    .voisins(*i, *j)
                    .iter()
                    .map(|(a, b)| self.to_litt(*a, *b))
                    .collect();
                formule.push(vec![
                    sous_sous_formule,
                    vec![-self.contrainte_to_litt(*i, *j, *p)],
                ]);
            } else {
                let mut sous_formule = CMed::new();
                sous_formule.push(vec![-self.contrainte_to_litt(*i, *j, *p)]);
                for forme in self.formes(*i, *j, *p).iter() {
                    let mut sous_sous_formule = CInt::new();
                    for (a,b) in forme.iter() {
                        if *a != *i && *b != *j{
                            sous_sous_formule.push(self.to_litt(*i,*j));
                        }
                    }
                    sous_sous_formule.push(-self.to_litt(*i,*j));
                    for (a,b) in self.bordure(forme){
                        sous_sous_formule.push(-self.to_litt(a, b));
                    }
                    sous_formule.push(sous_sous_formule);
                }
                formule.push(sous_formule);
            }
        }
        formule
    }

    /// Transforme la donnée d'une case en le littéral représentant le fait qu'elle est noire
    fn to_litt(&self, i: u8, j: u8) -> Litt {
        (self.taille * j + i + 1) as Litt
    }

    /// Transforme la donnée d'une contrainte en le littéral représentant le fait qu'elle existe dans une
    /// grille
    fn contrainte_to_litt(&self, i: u8, j: u8, p: i8) -> Litt {
        (self.taille * j + i + 1 + self.taille * self.taille * ((p + 2) as u8)) as Litt
    }

    /// Donne les voisins d'une case de la grille
    fn voisins(&self, i: u8, j: u8) -> Vec<(u8, u8)> {
        let mut v = Vec::<(u8, u8)>::new();
        if i != 0 {
            v.push((i - 1, j));
        }
        if j != 0 {
            v.push((i, j - 1));
        }
        if i != self.taille - 1 {
            v.push((i + 1, j));
        }
        if j != self.taille - 1 {
            v.push((i, j + 1));
        }
        v
    }

    /// Donne l'ensemble des formes (ensemble de coordonnéees) qui sont autour de (i,j) et de
    /// taille p
    fn formes(&self, i: u8, j: u8, p: i8) -> Vec<HashSet<(u8, u8)>> {
        let mut centre = HashSet::<(u8, u8)>::new();
        centre.insert((i, j));
        self.formes_rec(vec![centre], p)
    }

    /// Fonction intermédiaire pour la réccurence
    fn formes_rec(&self, centre: Vec<HashSet<(u8, u8)>>, p: i8) -> Vec<HashSet<(u8, u8)>> {
        if p == 0 {
            return centre;
        } else {
            let mut retour = Vec::<HashSet<(u8, u8)>>::new();
            let ensemble_rec = self.formes_rec(centre, p - 1);
            for forme in ensemble_rec.iter() {
                for (i, j) in forme.iter() {
                    for (iv, jv) in self.voisins(*i, *j).iter() {
                        let mut forme_temp = forme.clone();
                        if forme_temp.insert((*iv, *jv)) {
                            retour.push(forme_temp);
                        }
                    }
                }
            }
            retour
        }
    }

    ///Renvoie les cases n'étant pas dans la centre mais étant en contact direct
    fn bordure(&self, centre: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
        let mut retour = HashSet::<(u8, u8)>::new();
        for (i, j) in centre.iter(){
            for v in self.voisins(*i,*j){
                if !centre.contains(&v){
                    retour.insert(v);
                }
            }
        }
        retour
    }
}
