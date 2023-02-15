#[allow(dead_code)]
pub fn main() {
    // Informamos o tipo que ser[a f32 e terá 4 elementos]
    let notas: [f32; 4] = [10f32, 8f32,  8.5, 6.4];
    // Para repetirmos algum valor, utilizamos a declaração [valor ; quantas vezes irá repetir]
    // let notas: [f32; 4] = [6.5; 4]
    
    // Ele aloca o tamanho necessário para armazenar um ponteiro com base no sistema operacional
    // não usamos isize pois vetor nao tem indice negativo 
    let inteiro: usize = 0;
    
    println!("{}",notas[inteiro]);
    
    for indice in 0..notas.len() {
        println!("A nota{} é {}",indice+1,notas[indice]);
    }
    
    matriz();
}

fn matriz(){
    // É um vetor de vetores
    // declaração é [[f64; 3]; 2] para 2 linhas e 3 colunas
    let matriz = [
        [0.1; 3],
        [0.4; 3]
    ];
    
    for linha in matriz {
        for item in linha {
            println!("Item = {}", item)
        }
    }
}
