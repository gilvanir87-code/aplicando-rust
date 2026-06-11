use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Digite o primeiro número:");
    io::stdin()
        .read_line(&mut numero1)
        .expect("Erro ao ler o número");

    println!("Digite o segundo número:");
    io::stdin()
        .read_line(&mut numero2)
        .expect("Erro ao ler o número");

    let n1: f64 = numero1.trim().parse().expect("Número inválido");
    let n2: f64 = numero2.trim().parse().expect("Número inválido");

    let resultado = n1 + n2;

    println!("Resultado: {}", resultado);
}