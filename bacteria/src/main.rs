#[derive(Debug)]
enum Gram {
    GramPositiva,
    GramNegativa,
}

#[derive(Debug)]
enum Formato {
    Cocos(Cocos),
    Bacilos(Bacilos),
    Espirilos,
}

#[derive(Debug)]
enum Cocos {
    Diplococos,
    Estreptococos,
    Estafilococos,
}

#[derive(Debug)]
enum Bacilos {
    Diplobacilos,
    Estreptobacilos,
}

#[derive(Debug)]
enum Recombinacao {
    Conjugacao,
    Transformacao,
    Transducao,
}

#[derive(Debug)]
enum TipoDeCelula {
    Procarionte,
    Eucarionte,
}

#[derive(Debug)]
enum Reproducao {
    Cissiparidade,
    Brotamento,
    Esporulacao,
}

#[derive(Debug)]
struct Bacteria {
    quantidade_de_celulas: u32,
    tipo_de_celula: TipoDeCelula,
    formato: Formato,
    coloracao_de_gram: Gram,
    reproducao: Reproducao,
    recombinacao_genetica: Recombinacao,
    doenca: String,
}

fn main() {
    let bacteria = Bacteria {
        quantidade_de_celulas: 1,
        tipo_de_celula: TipoDeCelula::Procarionte,
        coloracao_de_gram: Gram::GramPositiva,
        reproducao: Reproducao::Brotamento,
        recombinacao_genetica: Recombinacao::Conjugacao,
        doenca: String::from("Tuberculose"),
    };

    println!("The bacteria is: {:#?}", bacteria);
}