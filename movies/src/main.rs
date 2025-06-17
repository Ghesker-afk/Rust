mod filmes;

use filmes::{Filme, Catalogo, Genero};

fn main() {
    let mut catalogo = Catalogo::new();

    // Adicionando filmes
    catalogo.adicionar_filme(Filme::novo(1, "Interestelar", "Cristopher Nolan", "2014", Genero::FiccaoCientifica));
    catalogo.adicionar_filme(Filme::novo(2, "O Poderoso Chefão", "Francis Ford Coppola", "1972", Genero::Drama));
    catalogo.adicionar_filme(Filme::novo(3, "Toy Story", "John Lasseter", "1995", Genero::Animacao));
}
