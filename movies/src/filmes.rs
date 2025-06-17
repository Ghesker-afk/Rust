use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Genero {
  Acao,
  Drama,
  Comedia,
  FiccaoCientifica,
  Terror,
  Animacao,
  Documentario,
}

#[derive(Debug, Clone)]
pub struct Filme {
  pub id: u32,
  pub titulo: String,
  pub diretor: String,
  pub ano: String,
  pub genero: Genero,
}

impl Filme {
  pub fn novo(id: u32, titulo: &str, diretor: &str, ano: u32, genero: Genero) -> Self {
    Filme {
      id,
      titulo: titulo.to_string(),
      diretor: diretor.to_string(),
      ano: ano.to_string(),
      genero,
    }
  }

  pub fn detalhes(&self) -> String {
    format!("{} ({}), dirigido por {}, gênero: {:?}", self.titulo, self.ano, self.diretor, self.genero)
  }
}

pub struct Catalogo {
  filmes: HashMap<u32, Filme>,
}

impl Catalogo {
  pub fn new() -> Self {
    Catalogo {
      filmes: HashMap::new(),
    }
  }

  pub fn adicionar_filme(&mut self, filme: Filme) {
    self.filmes.entry(filme.id).or_insert(filme);
  }

  pub fn buscar_por_id(&self, id: u32) -> Option<&Filme> {
    self.filmes.get(&id)
  }

  pub fn listar_filmes(&self) {
    for filme in self.filmes.values() {
      println!("{}", filme.detalhes());
    }
  }
}