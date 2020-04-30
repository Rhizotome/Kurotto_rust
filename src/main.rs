mod grille;
use grille::Grille;

fn main() {
    let taille = 2;
    let mut grille = Grille::new(taille);
    for i in 0..taille {
        for j in 0..taille {
            for p in -1..(taille as i8) {
                grille.add_contrainte(i,j,p);
            }
        }
    }
}
