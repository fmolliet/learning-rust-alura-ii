
// Vetor com tamanho fixo são os Arrays ou [] 
// E tem o Vectors que são Arrays dinamicos
pub fn main() {
vectors();
}

fn vectors(){
    // Cria um vetor na HEAP
    // Vec tbm precisa de um tipo somente em um dos lados, antes da criação da variavel por isso Vec<u8>
    let mut notas:Vec<f32> = Vec::new();
    // Alocar memória na heap é um processo custoso, 
    // então evitar esse processo é necessário. 
    // PS.: Além de alocar a memória, esse processo também demanda a cópia dos dados
    // de um endereço para outro (o que não é tão custoso assim).
    let mut notas_com_capacity: Vec<f32> = Vec::with_capacity(4);
    
    // existe uma macro para criar ele já preenchido
    let mut notas_preenchidas = vec![10.0, 8.8, 6.5];
    println!("Capacidade = {}", notas_com_capacity.capacity());
    
    notas_com_capacity.push(10f32);
    notas_com_capacity.push(8.8);
    notas_com_capacity.push(6.5);
    
    println!("Capacidade = {}", notas_com_capacity.capacity());
    
    println!("{:?}", notas );
    println!("{:?}", notas_preenchidas );
    // Com Vec vc pode apontar para o indice que pode estar fora 
    // Ele compila mas pode ter erro ao tentar acessar um indice
    // Acessando os indices de forma dinamica utilizando um Option
    println!("Nota1 = {}", match notas.get(7) {
        Some(n) => *n, // Esse get ele irá retornar uma Referencia de um Some caso ele encontre
        // então podemos ou usar o *n para buscar na referencia o valor
        // Outra possibilidade é desreferencia-lo usando & dentro do Some: Some(&n)
        None => 0.0
    } );
    
    // o pop removerá o ultimo item e retonará um option
    // Enquanto o notas pop nao retornar none ele executará
    // while let Some(nota) =  notas.pop(){
    //     println!("Valor removido = {}",nota);
    //     println!("{:?}", notas );
    // }
    
    // Quando é usando um for, ele faz borrow do valor, logo precisamos adicionar o &
    // Para que o Vec seja percorrido, uma função chamada into_iter é chamada recebendo o Vec por parâmetro, 
    // logo, o borrowing precisa ser levado em consideração.
    for nota in &notas {
        println!("Nota = {}", nota)
    }
    println!("{:?}", notas );
    
    // Devemos evitar realocação de memória pois é custoso
    
}