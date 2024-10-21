# Cours-de-Rust
Note from a Rust course

<img src="[https://tenor.com/view/rust-lang-ferris-gif-19191459](https://media1.tenor.com/m/djaLJiqAxzIAAAAC/rust-lang-ferris.gif)">

## Pourquoi coder en Rust ?
- Il est tout aussi performant que C et C++ (ça depend surtout de ce que l'on code).

- Il a une gestion plus "sécurisé" de la mémoire. C'est à dire que là ou C/C++ vont vous laissez faire des erreurs, Rust va éviter cela.
Comment ? Grâce à l'ownership :
    - Chaque variable est propriétaire d'une valeur.
    - Il ne peut y avoir qu'un seul propriétaire
    - Lorsque le propriétaire sort de la portée, la valeur est libérée
 
Cela rajoute également certainne contrainte :
```rs
fn main() {
    let chaine = String::from("hello");
    let len = calculate_length(chaine); // ici, la fonction prend possession de `chaine`
    println!("La longueur de '{}' est {}.", chaine, len);    ERREUR car `message` a été libéré auparavant
}

fn calculate_length(s: String) -&gt; usize {
    s.len()
}  
```


## Pourquoi ne pas coder en Rust ?
Jusqu'a maitenant j'en ai fait que quelques heures et je trouve la syntaxe atroce.
