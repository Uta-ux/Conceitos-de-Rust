# Exercícios Iniciantes - Rust Básico

Baseados nos seus arquivos: `dados_primitivos.rs`, `dado_compostos.rs`, `main.rs`

---

## 🟢 Nível 1 - Tipos Primitivos (arquivo: `dados_primitivos.rs`)

### Exercício 1.1 - Declare e imprima
```rust
fn main() {
    // TODO: Declare uma variável 'idade' do tipo u8 com valor 18
    // TODO: Declare uma variável 'altura' do tipo f32 com valor 1.75
    // TODO: Declare uma variável 'inicial' do tipo char com valor 'A'
    // TODO: Declare uma variável 'estudante' do tipo bool com valor true
    
    // TODO: Imprima todas com println!
}
```

### Exercício 1.2 - Inteiros com sinal e sem sinal
```rust
fn main() {
    // TODO: Crie 'negativo: i8 = -10' e 'positivo: u8 = 10'
    // TODO: Imprima os dois
    // Pergunta: O que acontece se tentar 'let errado: u8 = -5'?
}
```

### Exercício 1.3 - Conversão simples
```rust
fn main() {
    let decimal: f64 = 10.75;
    // TODO: Converta 'decimal' para i32 usando 'as'
    // TODO: Imprima o resultado (deve ser 10)
}
```

### Exercício 1.4 - Type alias
```rust
// TODO: Crie: type Nota = u8;
// TODO: Declare 'let matematica: Nota = 9;'
// TODO: Imprima
```

### Exercício 1.5 - Função simples
```rust
// TODO: Complete a função que soma dois i32
fn soma(a: i32, b: i32) -> i32 {
    // dica: só escreva 'a + b' sem ponto e vírgula
}

fn main() {
    let resultado = soma(5, 3);
    println!("5 + 3 = {}", resultado);
}
```

---

## 🟢 Nível 2 - Tipos Compostos (arquivo: `dado_compostos.rs`)

### Exercício 2.1 - String fixa vs mutável
```rust
fn main() {
    // &str (tamanho fixo)
    let nome_fixo: &str = "Ana";
    
    // String (tamanho flexível) - precisa de 'mut'
    let mut nome_flexivel: String = String::from("Ana");
    
    // TODO: Adicione '!' ao final de nome_flexivel usando .push()
    // TODO: Imprima os dois
}
```

### Exercício 2.2 - Array fixo
```rust
fn main() {
    // TODO: Crie um array de 5 números: [10, 20, 30, 40, 50]
    // Dica: let numeros: [i32; 5] = [10, 20, 30, 40, 50];
    
    // TODO: Imprima o elemento do índice 2 (deve ser 30)
    // TODO: Imprima o tamanho com .len()
}
```

### Exercício 2.3 - Array preenchido com zeros
```rust
fn main() {
    // TODO: Crie array de 10 posições preenchido com 0
    // Dica: let zeros: [i32; 10] = [0; 10];
    
    // TODO: Imprima o último elemento (índice 9)
}
```

### Exercício 2.4 - Vetor (Vec)
```rust
fn main() {
    // TODO: Crie um vetor com: vec![1, 2, 3, 4, 5]
    // TODO: Adicione o número 6 com .push(6)
    // TODO: Remova o último com .pop()
    // TODO: Imprima o vetor final
}
```

### Exercício 2.5 - Tupla simples
```rust
fn main() {
    // TODO: Crie uma tupla: ("Rust", 2024, true)
    // TODO: Acesse o primeiro elemento com .0
    // TODO: Acesse o segundo com .1
    // TODO: Imprima os dois
}
```

### Exercício 2.6 - Desestruturar tupla
```rust
fn main() {
    let pessoa: (&str, u8) = ("Carlos", 25);
    
    // TODO: Desestruture: let (nome, idade) = pessoa;
    // TODO: Imprima nome e idade separados
}
```

---

## 🟢 Nível 3 - Variáveis, Mutabilidade e Escopo (arquivo: `main.rs`)

### Exercício 3.1 - Mutável vs Imutável
```rust
fn main() {
    // Imutável (padrão)
    let x = 10;
    // x = 20; // ERRO! Descomente para ver o erro
    
    // Mutável
    let mut y = 10;
    y = 20; // OK
    
    // TODO: Imprima x e y
}
```

### Exercício 3.2 - Shadowing (sombreamento)
```rust
fn main() {
    let numero = 5;
    println!("Primeiro: {}", numero);
    
    // TODO: Faça shadowing: let numero = numero + 1;
    // TODO: Imprima novamente
    
    // TODO: Faça shadowing mudando o tipo: let numero = "agora sou texto";
    // TODO: Imprima
}
```

### Exercício 3.3 - Escopo de bloco
```rust
fn main() {
    let x = 100;
    
    { // novo escopo
        let x = 50; // variável DIFERENTE, só existe aqui
        println!("Dentro do bloco: {}", x);
    }
    
    // TODO: Imprima x aqui (qual valor aparece?)
}
```

### Exercício 3.4 - Constante
```rust
// TODO: Declare uma constante MAX_PONTOS: u32 = 100;
// (constantes vão FORA do main, em MAIÚSCULO)

fn main() {
    // TODO: Imprima a constante
    // Tente: MAX_PONTOS = 200; // ERRO! Descomente para ver
}
```

### Exercício 3.5 - Char e impressão
```rust
fn main() {
    // TODO: Declare 'let letra: char = 'Z';'
    // TODO: Imprima: "A letra é: Z"
}
```

---

## 🟢 Nível 4 - Juntando tudo (usando seus módulos)

### Exercício 4.1 - Usar função do outro arquivo
```rust
// No main.rs, seus módulos já estão declarados:
// mod dados_primitivos;
// mod dado_compostos;

fn main() {
    // TODO: Chame a função soma do módulo dados_primitivos
    // Dica: dados_primitivos::matematica(10, 5)
    
    // TODO: Chame a função do módulo dado_compostos
    // Dica: dado_compostos::compostos_datatype()
}
```

### Exercício 4.2 - Conversão de temperatura
```rust
fn main() {
    let celsius: f64 = 30.0;
    
    // TODO: Calcule fahrenheit: celsius * 9.0 / 5.0 + 32.0
    // TODO: Calcule kelvin: celsius + 273.15
    // TODO: Imprima os três
}
```

### Exercício 4.3 - Tupla com temperaturas
```rust
fn main() {
    let celsius: f64 = 25.0;
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    let kelvin = celsius + 273.15;
    
    // TODO: Crie uma tupla com os três valores
    // TODO: Imprima com {:?}
}
```

---

## ✅ Checklist de Conclusão

- [ ] Todos exercícios do Nível 1 compilam e rodam
- [ ] Todos exercícios do Nível 2 compilam e rodam
- [ ] Todos exercícios do Nível 3 compilam e rodam
- [ ] Exercícios do Nível 4 funcionam com seus módulos
- [ ] Conseguiu explicar cada conceito para si mesmo

---

## 💡 Dicas

1. **Compile frequentemente**: `cargo check` ou `cargo run`
2. **Leia os erros** - o Rust explica o que está errado
3. **Use `// TODO:`** - marque o que precisa fazer
4. **Teste um por vez** - não escreva tudo junto
5. **Apague o `// TODO:`** quando completar

---

## 🚀 Como rodar

```bash
cd /c/Users/beicom/Documents/GitHub/Conceitos-de-Rust/Conceitos_Rust
cargo run
```

Bons estudos! 🦀