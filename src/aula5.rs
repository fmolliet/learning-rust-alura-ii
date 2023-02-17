pub fn main() {
    conta_corrente();
}
// Parece uma classe do C++ e é armazenada na Stack
struct Conta {
    titular: Titular,
    saldo: f64
}

// Adicionando metodos ao struct
impl Conta {
    // A função deve receber o self para apontar  para a referencia de quem chamou ela
    // é necessario falar que vai passar uma referencia mutavel &mut
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String, 
    sobrenome:String
}

fn conta_corrente(){
    // let titular_conta :String = String::from("Fabio");
    // let saldo_conta: f64 = 100.0;
    let titular: Titular = Titular{nome: String::from("Fabio"), sobrenome: String::from("Molliet")};
    let mut conta: Conta = Conta{ 
        titular, // Quanddo tem o mesmo nome que o valor da Struct é possivel omitir o 
        // em vez de titular: titular, ficar somente titular, igual JS
        saldo : 100.0
    };
    
    conta.sacar(50f64);
    
    println!(
        "Dados da conta: Titular = {} {}, Saldo = {}",
        conta.titular.nome,
        conta.titular.sobrenome,
        conta.saldo
    );
    
}