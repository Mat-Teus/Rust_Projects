use std::io;
use rand::Rng;

fn main(){
    let mut resultado:i32 = 0;
    let mut input = String::new();
    let mut rng = rand::thread_rng();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_lowercase();

    let partes = parse_partes(&input);

    let mut bonus_total = 0;

    for parte in partes {
        let sinal = if parte.starts_with('-') { -1 } else { 1 };
        let parte = parte.trim_start_matches(|c| c == '+' || c == '-');

        let mut rolagem: Vec<i32> = parte.chars().map(|c| if c.is_numeric() { c } else { ' ' }).collect::<String>().split_whitespace().filter_map(|x| x.parse().ok()).collect();

        if rolagem.is_empty() {
            continue;
        }

        if parte.starts_with('d'){
            rolagem.insert(0, 1);
        }

        if rolagem.len() == 1 {
            bonus_total += sinal * rolagem[0];
        } else{
            let qtd = rolagem[0];
            let faces = rolagem[1];

            if faces <= 0 || qtd <= 0 {
                continue;
            }

            let bonus = if rolagem.len() > 2 { rolagem[2] } else { 0 };

            for _ in 0..qtd {
                resultado += sinal * rng.gen_range(1..=faces);
            }

            bonus_total += sinal * bonus;
        }
    }

    resultado += bonus_total;
    if resultado < 0{
        println!("0");
    }else{
        println!("{}", resultado);
    }
}

fn parse_partes(input: &str) -> Vec<String> {
    let mut partes = Vec::new();
    let mut atual = String::new();

    for (i, c) in input.chars().enumerate() {
        if (c == '+' || c == '-') && i != 0 {
            partes.push(atual);
            atual = String::new();
        }
        atual.push(c);
    }

    if !atual.is_empty() {
        partes.push(atual);
    }

    partes
}