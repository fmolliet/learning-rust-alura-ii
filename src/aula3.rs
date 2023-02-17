fn conteudo_opcional(){
    let conteudo_arquivo = ler_arquivo(String::from(""));
    // É necessário tratar se irá retornar ou None ou Some
    // Por estar no Heap ele precisa fazer borrow, usando o &
    println!("{}", match &conteudo_arquivo {
        None => "Arquivo não existe",
        Some(valor) => valor
    });
    // Essa sintaxe faz a validação e exibe arrays, enum 
    // Isso é a exibição de debug no Rust.
    println!("{:?}", conteudo_arquivo );
    
    // Em enum precisamos passar os tipos, com options não precisamos passar o tipo
    
    // caso de errado ele nao executa 
    // é um match com um unico valor
    // Tambem tem como usar um While let
    if let Some(valor) = conteudo_arquivo {
        // Só vai executar quando entrar na condicional
        println!("Agora, tem certeza de que ha o valor {}", valor);
    }
}

// Generics server para criar 
// template para o compilador utilizar para o tipos declarados
#[allow(dead_code)]
enum GenericOption<T> {
    Some( T ),
    None
}

#[allow(dead_code)]
enum GenericResult <S, E> {
    Ok(S),
    Err(E)
}

fn ler_arquivo(caminho: String) -> Option<String> {
    Some(String::from(caminho))
    // Ou podemos retornar None
}

#[allow(dead_code)]
pub fn main() {
    conteudo_opcional();
}