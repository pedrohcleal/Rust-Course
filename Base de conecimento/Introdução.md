# Introdução

## Declaração de variáveis

Em Rust, as declarações de variáveis são utilizadas para introduzir novas identificadores (nomes) associados a um valor. A linguagem Rust é estaticamente tipada, o que significa que o tipo de uma variável deve ser conhecido em tempo de compilação. Aqui estão as principais formas de declarar variáveis em Rust:

1. **Declaração Simples:**
   ```rust
   let nome_da_variavel = valor;
   ```

   Neste caso, o compilador Rust inferirá automaticamente o tipo da variável com base no tipo do valor atribuído.

   Exemplo:
   ```rust
   let numero = 42;  // Rust infere que 'numero' é do tipo i32
   ```

2. **Declaração com Tipo Explicito:**
   ```rust
   let nome_da_variavel: Tipo = valor;
   ```

   É possível especificar explicitamente o tipo da variável, o que pode ser útil em situações onde a inferência de tipo não é suficiente ou para melhorar a clareza do código.

   Exemplo:
   ```rust
   let idade: u32 = 25;  // 'idade' é explicitamente do tipo u32
   ```

3. **Mutabilidade:**
   Em Rust, as variáveis são, por padrão, imutáveis. Se você precisar de uma variável mutável, use a palavra-chave `mut`.

   Exemplo:
   ```rust
   let mut contador = 0;  // 'contador' é mutável
   contador += 1;
   ```

4. **Constantes:**
   As constantes são declaradas usando a palavra-chave `const` e sempre têm que ter um tipo explícito. Além disso, elas devem ser avaliadas em tempo de compilação.

   Exemplo:
   ```rust
   const PI: f64 = 3.14159;  // Constante PI do tipo f64
   ```

5. **Declaração de Múltiplas Variáveis:**
   É possível declarar várias variáveis ao mesmo tempo.

   Exemplo:
   ```rust
   let (x, y) = (10, 20);  // 'x' e 'y' são variáveis com valores 10 e 20
   ```

6. **Inicialização Condicional:**
   Rust permite inicialização condicional usando a palavra-chave `if`.

   Exemplo:
   ```rust
   let numero = if condição { 10 } else { 20 };
   ```

Essas são as principais formas de declarar variáveis em Rust. A linguagem enfatiza a segurança de memória e a prevenção de erros, utilizando o conceito de propriedade de propriedade, que é verificada em tempo de compilação pelo borrow checker. Isso ajuda a evitar muitos tipos comuns de erros associados à manipulação de memória.

## Tipos de dados

1. **Integer Types (Tipos Inteiros):**
   - `i8`, `i16`, `i32`, `i64`, `i128`: Inteiros com sinal de diferentes tamanhos.
   - `u8`, `u16`, `u32`, `u64`, `u128`: Inteiros sem sinal de diferentes tamanhos.
   - `isize` e `usize`: Dependem da arquitetura do sistema e representam o tamanho de um ponteiro.

2. **Floating-Point Types (Tipos de Ponto Flutuante):**
   - `f32` e `f64`: Números de ponto flutuante de precisão simples e dupla, respectivamente.

3. **Boolean Type (Tipo Booleano):**
   - `bool`: Representa valores verdadeiros ou falsos.

4. **Character Type (Tipo Caractere):**
   - `char`: Representa um caractere Unicode, delimitado por aspas simples.

5. **Compound Types (Tipos Compostos):**
   - **Arrays:** Coleções fixas de elementos do mesmo tipo com tamanho fixo.
     ```rust
     let array: [i32; 5] = [1, 2, 3, 4, 5];
     ```

   - **Tuples:** Coleções heterogêneas de elementos, onde cada elemento pode ter um tipo diferente.
     ```rust
     let tuple: (i32, f64, char) = (42, 3.14, 'a');
     ```

6. **Slice Types (Tipos de Fatia):**
   - `&[T]`: Representa uma visualização de uma parte de um array ou outra coleção.

7. **String Type (Tipo String):**
   - `String`: Uma coleção de caracteres, alocada dinamicamente e mutável.

8. **Option Type (Tipo Opção):**
   - `Option<T>`: Representa um valor opcional, podendo ser `Some(T)` ou `None`.

9. **Result Type (Tipo Resultado):**
   - `Result<T, E>`: Representa uma operação que pode falhar, sendo `Ok(T)` em caso de sucesso e `Err(E)` em caso de erro.

10. **Reference Types (Tipos de Referência):**
    - `&T` e `&mut T`: Referências imutáveis e mutáveis, respectivamente.

Estes são alguns dos principais tipos de dados em Rust. A linguagem é projetada para ser explícita e segura em relação aos tipos, promovendo a prevenção de erros de segurança e de execução em tempo de compilação.
