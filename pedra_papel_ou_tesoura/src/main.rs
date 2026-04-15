use std::io;
use rand::Rng;

#[derive(PartialEq)]
enum Escolha{
    Pedra,
    Papel,
    Tesoura,
}

fn main(){
    let mut vit_p:u32 = 0;
    let mut vit_c:u32 = 0;
    let mut empate:u32 = 0;
    let mut jogar_novamente:bool;

    loop{
        let mut input = String::new();

        println!("Escolha 1 - pedra, 2 - papel ou 3 - tesoura");
        let escolha = loop{
            input.clear();
            io::stdin().read_line(&mut input).expect("Erro inesperado");
            match input.trim(){
                "1" => break Escolha::Pedra,
                "2" => break Escolha::Papel,
                "3" => break Escolha::Tesoura,
                _=> println!("Entrada inválida"),
            }
        };

        let escolha_pc = loop{
            let x = rand::thread_rng().gen_range(1..=3);
            match x{
                1 => break Escolha::Pedra,
                2 => break Escolha::Papel,
                3 => break Escolha::Tesoura,
                _=> unreachable!(),
            }
        };

        resultado(&escolha, "Player");
        resultado(&escolha_pc, "PC");

        match (&escolha, &escolha_pc) {
            (a, b) if a == b => {println!("Empate");
                empate += 1; 
            },

            (Escolha::Pedra, Escolha::Tesoura) | (Escolha::Papel, Escolha::Pedra)| (Escolha::Tesoura, Escolha::Papel) => {println!("Vitória do player");
                vit_p += 1;
        }

            _ => {println!("Vitória do computador");
                  vit_c += 1;
                                                    },
        }

        println!("Deseja parar de jogar?");
        jogar_novamente = loop{
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Erro inesperado");
                    match input.trim() {
                        "s" | "S" => break true,
                        "n" | "N" => break false,
                        _ => println!("Digite 's' para sim ou 'n' para não"),
                    }
                };

        if jogar_novamente{
            break;
        }
    }

    println!("Estatísticas:");
    println!("Vitórias player:{}", vit_p);
    println!("Vitórias computador:{}", vit_c);
    println!("Empates:{}", empate);
}

fn resultado(escolha:&Escolha, nome:&str){
    match escolha{
        Escolha::Pedra => println!("O {} escolheu pedra", nome),
        Escolha::Papel => println!("O {} escolheu papel", nome),
        Escolha::Tesoura => println!("O {} escolheu tesoura", nome),
    }
}