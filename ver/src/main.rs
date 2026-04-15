use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    
//Caso não passe nenhum argumento, retorna um erro
    if args.len() < 2{
        println!("Houve um problema, tente novamente");
    }else{
//Abre o arquivo
        let nome_arquivo = &args[1];
        let mut arquivo = File::open(nome_arquivo).unwrap();
        let mut conteudo = String::new();
//Le o conteúdo para dentro de uma variável
        arquivo.read_to_string(&mut conteudo).expect("Erro ao abrir o arquivo"); // Propaga o erro se falhar
//Acabando com o \n no final de conteudo
	conteudo.trim();
//Imprime na tela o conteúdo
        print!("{}", conteudo);
    }
}
