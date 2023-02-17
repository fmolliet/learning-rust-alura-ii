// metaprogramação
#[allow(dead_code)]
enum DiaDeSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana( dia_da_semana: DiaDeSemana ) -> bool {
    match dia_da_semana {
        DiaDeSemana::Domingo | DiaDeSemana::Sabado => true,
        _ => false
    }
}
// para remover warning de codigo morto
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    // Usamos chaves para receber com nome ou Struct
    CmykColor {cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn cores(){
    // Usamos chaves para receber com nome
    let cor = Color::CmykColor{cyan: 100, magenta: 50, yellow: 70, black: 255};
    
    println!("Cor = {}", match cor {
        Color::Blue => "azul",
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::RgbColor(0 ,0 , 0 ) 
            | Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => "preto",
        //Da para tratar o que veio por exemplo
        // Color::RgbColor(red,green ,blue ) => "RGB {}, {}, {}"
        Color::RgbColor(_ ,_ , _ ) => "desconhecido",
        // Podemos validar tipos complexos de enum
        Color::CmykColor{cyan: _, magenta: _, yellow: _, black: _}=> "cymk desconhecido"
    });
}
#[allow(dead_code)]
pub fn main() {
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDeSemana::Domingo));
    cores();
}