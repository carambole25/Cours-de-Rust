/*
Pourquoi coder en Rust ?
- Il est tout aussi performant que C et C++ (ça depend surtout de ce que l'on code).

Il a une gestion plus "sécurisé" de la mémoire. C'est à dire que là ou C/C++ vont vous laissez faire des erreurs, Rust va éviter cela.
Comment ? Grâce à l'ownership :
    - Chaque variable est propriétaire d'une valeur.
    - Il ne peut y avoir qu'un seul propriétaire
    - Lorsque le propriétaire sort de la portée, la valeur est libérée


Pourquoi ne pas coder en Rust ?
Jusqu'a maitenant j'en ai fait que quelques heures et je trouve la syntaxe attroce.
*/

fn main(){
    println!("Hello World !");

    // Par defaut les variables sont des constantes
    let variable_constante = 3;

    // Pour pouvoir les modifier il faut utiliser le mot clé mut à la déclaration
    let mut variable_mutable = 5;
    variable_mutable = 2;

    // On peu (et il le faut pour les fonctions) préciser le type d'une var
    let entier: i32 = 5; // int de 32 bits

    // On ne peu pas modifier la taille d'un tableau on peu simplement changer son contenu
    let tableau = [0, 2, 8];

    // Les vecteurs sont des tableaux modifiables
    let mut v = Vec::new();
    v.push(2);
    v.push(9);
    v.push(42);
    println!("{:?}", v);

    // Les slices sont des morceaux de mémoire ainsi que le nb d'element qu'elle contient
    let slice = &[0, 2, 5, 15];
    // Crée un slice qui commence au 2ème element de slice
    // On peu aussi faire un slice d'un tableau et d'un vecteur
    let slice_2 = &slice[1..];
    println!("{:?}", slice_2);

    // Le if et else c'est pareil qu'en C/C++
    // On peu utiliser () pour les sous conditions
    let heure = 15;

    if heure >= 18 {
        println!("bonsoir");
    }else {
        println!("bonjour");
    }

    // Le matching
    let couleur  = "red";
    match couleur {
        "rouge" => println!("Le mot est en Français"),
        "red" => println!("Le mot est en Anglais"),
        "rojo" => println!("Le mot est en Espagnole"),

        // Option par defaut
        _ => println!("Cas non pris en compte"),
    }

    let i = 45;

    match i {
        20..=60 => println!("La variable est comprise entre 20 et 60"),
        _ => println!("Pas comprise entre 10 et 100 :("),
    }

    println!("{}", addition(5, 8));

    afficher_hello(3);

    /*
    Plutôt que defaire while True on peu faire 
    loop {
        break; ou return; pour l'arreter
    }
        ça sert à rien...
    */

    // La syntaxe du for est avec deux points
    for i in 0..5{
        println!("{}", i);
    }

    let mon_super_vecteur = vec![5, 2, 6, 8, 5, 9];

    for v in mon_super_vecteur {
        println!("{}", v);
    }
}

fn addition(nb1: i32, nb2: i32) -> i32 {
    return nb1 + nb2;
}

fn afficher_hello(mut compt: i32){
    while compt > 0 {
        println!("Hello world");
        compt -= 1;
    }
}
