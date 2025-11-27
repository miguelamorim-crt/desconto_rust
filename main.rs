fn main() {
        // vem com tudo Rust!
        let preco: f64 = 200.00;
        let desconto: f64 = 100.00;
        let valor_final: f64 = preco - desconto;
        println!("preco original: {} | desconto: {} | valor com desconto: {}", preco, desconto, valor_final);
}