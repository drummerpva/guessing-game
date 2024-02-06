use std::io::stdin;

fn main() {
    loop {
        println!("Bem vindo ao jogo da advinhação!");
        println!("i - Iniciar o jogo");
        println!("q - Fechar o jogo");

        let mut escolha_str = String::new();
        let _ = stdin().read_line(&mut escolha_str);
        match escolha_str.trim().to_lowercase().as_str() {
            "i" => {
                println!("Iniciando o jogo!");
            }
            "q" => {
                println!("Obrigado por jogar!");
                break;
            }
            _ => {
                println!("Opção inválida! Tente novamente.");
                continue;
            }
        };
    }
}
