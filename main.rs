fn main() {
        let preco: f64 = 200.00;
        let desconto: f64 = 100.00;

        // validaçao
        if preco < 0.00 || desconto < 0.00 {
                println!("erro: valor menor que 0!");
                return;
        }

        let valor_final: f64 = preco - desconto;

        println!("preco original: {preco}");
        println!("desconto: {desconto}");
        println!("valor com desconto: {valor_final}");
}
