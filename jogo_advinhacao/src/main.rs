use std::io;
use rand::Rng;

fn main(){
    let mut input = String::new();
    let numero = rand::thread_rng().gen_range(0..=100);
    let mut count = 0;
    let mut pontuacao = 100;
    let mut jogar_novamente:bool;
    

    loop{
        loop{
            println!("Advinhe o número, ele está entre 0 e 100");
            let guess: i32 = loop {
                input.clear();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().parse::<i32>() {
                    Ok(num) if (0..=100).contains(&num) => break num,
                    Ok(_) => println!("Número fora do intervalo!"),
                    Err(_) => println!("Número inválido!"),
                 }
            };

            count+=1;
        
            if guess == numero{
                println!("Parabéns, você acertou");
                println!("Você acertou em {} tentivas", count);
                println!("Sua pontuação foi {} pontos", pontuacao);
                println!("Deseja jogar novamente? S/N");
                jogar_novamente = loop{
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Erro inesperado");
                    match input.trim() {
                        "s" | "S" => break true,
                        "n" | "N" => break false,
                        _ => println!("Digite 's' para sim ou 'n' para não"),
                    }
                };
                break;
            }else if guess < numero{
                println!("O número secreto é maior!");
                pontuacao -= 10;
            }else if guess > numero{
                println!("O número secreto é menor!");
                pontuacao -= 10;
            }
        }

        if jogar_novamente == false{
            break;
        }
    }
}
