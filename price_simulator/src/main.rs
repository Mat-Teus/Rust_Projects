use rand::Rng;

fn main() {
    let mut preco:f64 = 100.00;
    let mut rng = rand::thread_rng();
    let mut menor = preco;
    let mut maior = preco;

    for i in 1..=10{
        if preco <= 0.0{
            println!("Você abriu falência");
            break;
        }else if rng.gen_range(0..=100) > 2{
            let variacao = rng.gen_range(-2.0..=2.0);
            aplicar_variacao(&mut preco, &mut menor, &mut maior, variacao, i);
        }else{
            let variacao = rng.gen_range(-50.0..=50.0);
            aplicar_variacao(&mut preco, &mut menor, &mut maior, variacao, i);
        }
    }

    println!("O menor preço foi de {:.2} e o maior foi de {:.2}", menor, maior);
}

fn aplicar_variacao(preco: &mut f64,menor: &mut f64,maior: &mut f64,variacao: f64,passo: i32,) {
    *preco += *preco * variacao / 100.0;

    *menor = menor.min(*preco);
    *maior = maior.max(*preco);

    println!("No momento {}, a variação foi de {:.2} totalizando o preço {:.2}",passo, variacao, *preco);
}