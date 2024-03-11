# Intro part2

## Funções

Em Rust, as funções são blocos de código que realizam uma tarefa específica. Aqui estão alguns aspectos importantes sobre as funções em Rust:

1. **Declaração de Função:**
   - Para declarar uma função em Rust, use a palavra-chave `fn`, seguida pelo nome da função e uma lista de parâmetros entre parênteses. O tipo de retorno é indicado após uma seta (`->`).
   ```rust
   fn saudacao(nome: &str) -> String {
       format!("Olá, {}!", nome)
   }
   ```

2. **Chamada de Função:**
   - Para chamar uma função em Rust, use o nome da função seguido por argumentos entre parênteses.
   ```rust
   fn main() {
       let mensagem = saudacao("Rust");
       println!("{}", mensagem);
   }
   ```

3. **Parâmetros e Tipos:**
   - Os parâmetros são definidos na lista entre parênteses, seguidos pelos tipos. Rust é fortemente tipado, então os tipos dos parâmetros e do retorno são especificados.
   ```rust
   fn adicionar(a: i32, b: i32) -> i32 {
       a + b
   }
   ```

4. **Retorno Explícito:**
   - O tipo de retorno pode ser explicitamente indicado com `->`, ou Rust pode inferi-lo automaticamente com base na última expressão.
   ```rust
   fn duplicar(numero: i32) -> i32 {
       numero * 2
   }
   ```

5. **Bloco de Código:**
   - O corpo da função é um bloco de código delimitado por chaves `{}`. Este bloco contém as instruções que a função executará.
   ```rust
   fn exibir_mensagem() {
       println!("Esta é uma mensagem!");
   }
   ```

6. **Expressão vs. Declaração:**
   - Em Rust, as funções geralmente consistem em uma série de expressões. A última expressão de um bloco é implicitamente o valor retornado pela função.
   ```rust
   fn somar_e_exibir(a: i32, b: i32) -> i32 {
       let resultado = a + b;
       println!("A soma é: {}", resultado);
       resultado // Esta é a última expressão e será retornada
   }
   ```

7. **Funções como Expressões:**
   - As funções em Rust podem ser tratadas como expressões, o que significa que podem ser usadas em expressões maiores ou atribuídas a variáveis.
   ```rust
   fn multiplicar_por_cinco(numero: i32) -> i32 {
       numero * 5
   }

   fn main() {
       let resultado = multiplicar_por_cinco(3);
       println!("Resultado: {}", resultado);
   }
   ```

8. **Funções Aninhadas:**
   - É possível definir funções dentro de outras funções, conhecidas como funções aninhadas ou locais.
   ```rust
   fn funcao_principal() {
       fn funcao_aninhada() {
           println!("Esta é uma função aninhada.");
       }
       funcao_aninhada();
   }
   ```

Esses são alguns aspectos fundamentais das funções em Rust. A linguagem suporta programação funcional e fornece ferramentas para escrever código modular e expressivo.
