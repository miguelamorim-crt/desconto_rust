## 🦀 Calculadora de Desconto (Rust)

Um programinha simples em Rust que calcula o valor final de um produto após aplicar um desconto.

## 📌 Sobre o projeto

Este é meu primeiro código publicado usando Rust!
O objetivo é treinar:

Variáveis (let)

Tipos numéricos (f64)

Operações matemáticas

println! com interpolação

Estrutura básica de um programa Rust


## 🧮 Como funciona

O programa define:

preço original

valor do desconto

validaçao

preço final

Depois imprime o resultado formatado no console.

## 🔢 Exemplo de código

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

## ▶️ Como rodar

Instale o Rust (via rustup)

Compile:

rustc main.rs


Execute:

./main

## 📚 O que aprendi

Como criar um arquivo .rs

Como compilar com rustc

Como declarar variáveis

Como imprimir valores no termina

Validaçao simples

Return

## ♻️ versoes

versao  2:

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
