use std::io;
use rand::Rng;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;

fn main() {
    let caracteres = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*".to_vec();
    let mut input = String::new();
    println!("Selecione o tamanho da senha (Mínimo 7 caracteres):");
    io::stdin().read_line(&mut input).unwrap();
    let tam_senha: usize = {
        let valor: usize = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Erro: entrada inválida! Digite um número.");
                return;
            }
        };

        if valor >= 7 {
            valor
        } else {
            println!("Erro: a senha deve ter pelo menos 7 caracteres");
            return;
        }
    };

    let senha = {
        let mut s = String::with_capacity(tam_senha);
        let mut rng = OsRng;

        let minusculas = b"abcdefghijklmnopqrstuvwxyz";
        let maiusculas = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let numeros = b"0123456789";
        let simbolos = b"!@#$%^&*";

        s.push(minusculas[rng.gen_range(0..minusculas.len())] as char);
        s.push(maiusculas[rng.gen_range(0..maiusculas.len())] as char);
        s.push(numeros[rng.gen_range(0..numeros.len())] as char);
        s.push(simbolos[rng.gen_range(0..simbolos.len())] as char);

        for _ in 4..tam_senha {
            let c = rng.gen_range(0..caracteres.len());
            s.push(caracteres[c] as char);
        }

        let mut embaralha: Vec<char> = s.chars().collect();
        embaralha.shuffle(&mut rng);

        embaralha.into_iter().collect::<String>()
    };

    println!("{}", senha);
}
