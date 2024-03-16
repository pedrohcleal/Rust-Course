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

## Sobre `&`, o "e" comercial

Em Rust, o símbolo "&" tem vários significados, dependendo do contexto em que é utilizado. Aqui estão alguns dos usos mais comuns:

1. **Referências:**
   - Em Rust, o "&" é usado para criar referências a valores. Isso permite que você passe valores por referência para funções sem transferir a propriedade do valor. Por exemplo:
     ```rust
     fn main() {
         let x = 42;
         let referencia_para_x = &x;
         println!("Valor de x: {}", x);
         println!("Referência para x: {}", referencia_para_x);
     }
     ```

2. **Borrowing (Emprestar):**
   - O "&" é usado em operações de empréstimo, onde uma função ou trecho de código temporariamente "empresta" uma referência para um valor sem tomar posse completa dele.
     ```rust
     fn emprestar_referencia(valor: &i32) {
         // Faz algo com a referência, mas não possui o valor
     }

     fn main() {
         let numero = 100;
         emprestar_referencia(&numero);
     }
     ```

3. **Tipos de Referências:**
   - Rust possui dois tipos de referências: referências imutáveis (`&T`) e referências mutáveis (`&mut T`). As referências imutáveis permitem apenas leitura do valor referenciado, enquanto as referências mutáveis permitem alterações no valor referenciado.
     ```rust
     fn modificar_valor(valor: &mut i32) {
         *valor += 1; // Desreferenciação e modificação do valor
     }

     fn main() {
         let mut x = 5;
         modificar_valor(&mut x);
         println!("Novo valor de x: {}", x);
     }
     ```

4. **Desreferenciação:**
   - O operador "*" é usado para desreferenciar uma referência, convertendo-a de volta ao valor original.
     ```rust
     fn main() {
         let x = 42;
         let referencia_para_x = &x;
         let valor_original = *referencia_para_x;
         println!("Valor desreferenciado: {}", valor_original);
     }
     ```

5. **Slicing (Fatias):**
   - O "&" também é usado em fatias (`&[T]`), que são referências a partes de um array ou vetor.
     ```rust
     fn main() {
         let vetor = vec![1, 2, 3, 4, 5];
         let fatia = &vetor[1..4];
         println!("Fatia: {:?}", fatia);
     }
     ```

Esses são alguns dos principais contextos em que o "&" é utilizado em Rust. A ênfase em referências é uma parte fundamental da abordagem de Rust para garantir a segurança e a ausência de erros relacionados à propriedade de dados.

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

## Controle de fluxos

1. **Condicional - `if`, `else if`, `else`:**
   ```rust
   let numero = 42;

   if numero < 0 {
       println!("Negativo");
   } else if numero == 0 {
       println!("Zero");
   } else {
       println!("Positivo");
   }
   ```

2. **Loop Infinito - `loop`:**
   ```rust
   loop {
       // Código aqui
   }
   ```

3. **Loop com Condição - `while`:**
   ```rust
   let mut contador = 0;

   while contador < 5 {
       println!("Contagem: {}", contador);
       contador += 1;
   }
   ```

4. **Iteração com `for` e Ranges:**
   ```rust
   for numero in 1..=5 {
       println!("Número: {}", numero);
   }
   ```

5. **Iteração com `for` e Coleções:**
   ```rust
   let vetor = vec![1, 2, 3, 4, 5];

   for elemento in &vetor {
       println!("Elemento: {}", elemento);
   }
   ```

6. **Match - Padrões e Correspondência:**
   ```rust
   let valor = 42;

   match valor {
       0 => println!("Zero"),
       1 | 2 => println!("Um ou Dois"),
       3..=9 => println!("Três a Nove"),
       _ => println!("Outro valor"),
   }
   ```

7. **`if let` - Combinação de `if` e `match`:**
   ```rust
   let resultado: Result<i32, &str> = Ok(42);

   if let Ok(valor) = resultado {
       println!("Valor: {}", valor);
   } else {
       println!("Erro!");
   }
   ```

8. **`while let` - Iteração com `while` e `match`:**
   ```rust
   let mut opcao = Some(0);

   while let Some(valor) = opcao {
       println!("Valor: {}", valor);
       opcao = None; // Exemplo de interrupção do loop
   }
   ```

## Operadores lógicos

1. **`&&` (E lógico):**
   - Retorna verdadeiro (`true`) se ambas as expressões booleanas à esquerda e à direita forem verdadeiras.
   ```rust
   let x = true;
   let y = false;
   let resultado = x && y; // Resultado é false
   ```

2. **`||` (OU lógico):**
   - Retorna verdadeiro (`true`) se pelo menos uma das expressões booleanas à esquerda ou à direita for verdadeira.
   ```rust
   let x = true;
   let y = false;
   let resultado = x || y; // Resultado é true
   ```

3. **`!` (NÃO lógico):**
   - Inverte o valor booleano de uma expressão.
   ```rust
   let x = true;
   let resultado = !x; // Resultado é false
   ```

4. **`!=` (Diferente):**
   - Retorna verdadeiro (`true`) se as expressões à esquerda e à direita não forem iguais.
   ```rust
   let a = 5;
   let b = 7;
   let resultado = a != b; // Resultado é true
   ```

5. **`==` (Igual):**
   - Retorna verdadeiro (`true`) se as expressões à esquerda e à direita forem iguais.
   ```rust
   let a = 5;
   let b = 5;
   let resultado = a == b; // Resultado é true
   ```

6. **`<`, `>`, `<=`, `>=` (Relacionais):**
   - Realizam comparações relacionais entre valores.
   ```rust
   let a = 10;
   let b = 15;
   let resultado = a < b; // Resultado é true
   ```

7. **Operadores de Atribuição com Lógica:**
   - Rust permite combinar operadores de atribuição com operadores lógicos.
   ```rust
   let mut x = true;
   x &= false; // Equivalente a x = x && false;
   ```

## Manipulação de entrada e saída (I/O)

Em Rust, a manipulação de entrada e saída (I/O) é realizada principalmente por meio da biblioteca padrão, `std::io`. Aqui estão algumas das funcionalidades relacionadas à entrada e saída em Rust:

1. **Leitura de Linha (stdin):**
   - Para ler uma linha da entrada padrão (teclado), você pode usar o método `read_line` do tipo `std::io::Stdin`.
   ```rust
   use std::io;

   fn main() {
       let mut input = String::new();
       println!("Digite algo:");
       io::stdin().read_line(&mut input).expect("Erro ao ler linha");
       println!("Você digitou: {}", input.trim());
   }
   ```

2. **Leitura de Tipos Específicos:**
   - Você pode usar métodos como `parse` para converter a entrada para um tipo específico.
   ```rust
   use std::io;

   fn main() {
       println!("Digite um número:");
       let mut input = String::new();
       io::stdin().read_line(&mut input).expect("Erro ao ler número");
       let numero: i32 = input.trim().parse().expect("Erro ao converter para número");
       println!("Você digitou: {}", numero);
   }
   ```

3. **Saída para o Console (stdout):**
   - A macro `println!` é usada para imprimir texto no console, seguida por uma quebra de linha.
   ```rust
   fn main() {
       let nome = "Rust";
       println!("Olá, {}!", nome);
   }
   ```

4. **Formatação de Saída:**
   - Rust oferece formatação de saída avançada usando a macro `format!` e a função `println!`.
   ```rust
   fn main() {
       let idade = 5;
       let mensagem = format!("Eu tenho {} anos.", idade);
       println!("{}", mensagem);
   }
   ```

5. **Leitura de Arquivos:**
   - Para ler o conteúdo de um arquivo, você pode usar a função `std::fs::read_to_string` ou `std::fs::File` com `std::io::Read`.
   ```rust
   use std::fs::File;
   use std::io::Read;

   fn main() {
       let mut arquivo = File::open("arquivo.txt").expect("Erro ao abrir arquivo");
       let mut conteudo = String::new();
       arquivo.read_to_string(&mut conteudo).expect("Erro ao ler arquivo");
       println!("Conteúdo do arquivo:\n{}", conteudo);
   }
   ```

6. **Escrita em Arquivos:**
   - Para escrever em um arquivo, você pode usar a função `std::fs::File` com `std::io::Write`.
   ```rust
   use std::fs::File;
   use std::io::Write;

   fn main() {
       let mut arquivo = File::create("saida.txt").expect("Erro ao criar arquivo");
       arquivo.write_all(b"Olá, mundo!").expect("Erro ao escrever no arquivo");
   }
   ```

7. **Manipulação de Erros na I/O:**
   - O método `expect` ou o uso de resultados (`Result`) são comuns para lidar com possíveis erros durante operações de I/O.
   ```rust
   use std::io;

   fn main() {
       let mut input = String::new();
       io::stdin().read_line(&mut input).expect("Erro ao ler linha");
   }
   ```

Essas são algumas das funcionalidades básicas relacionadas à entrada e saída em Rust. A biblioteca padrão `std::io` fornece uma variedade de tipos e métodos para lidar com operações de entrada e saída de maneira eficiente e segura.

## Operações matemáticas

1. **Adição:**
   - `+` é usado para adicionar dois valores.
   - Exemplo: `let soma = 5 + 3;` resulta em `soma` contendo 8.

2. **Subtração:**
   - `-` é usado para subtrair um valor de outro.
   - Exemplo: `let diferenca = 7 - 2;` resulta em `diferenca` contendo 5.

3. **Multiplicação:**
   - `*` é usado para multiplicar dois valores.
   - Exemplo: `let produto = 4 * 6;` resulta em `produto` contendo 24.

4. **Divisão:**
   - `/` é usado para dividir um valor por outro.
   - Exemplo: `let quociente = 9 / 3;` resulta em `quociente` contendo 3.

5. **Módulo (Resto da Divisão):**
   - `%` retorna o resto da divisão entre dois números.
   - Exemplo: `let resto = 10 % 3;` resulta em `resto` contendo 1.

6. **Potenciação:**
   - `i32::pow()` é usado para elevar um número a uma potência.
   - Exemplo: `let resultado = i32::pow(2, 3);` resulta em `resultado` contendo 8.

7. **Operações com Números de Ponto Flutuante:**
   - Operações como adição, subtração, multiplicação e divisão também são aplicáveis a números de ponto flutuante.
   - Exemplo: `let resultado_adicao = numero1 + numero2;`.

8. **Incremento e Decremento:**
   - `+=` é usado para incrementar uma variável.
   - `-=` é usado para decrementar uma variável.
   - Exemplo:
     ```rust
     let mut contador = 0;
     contador += 1; // Incremento
     contador -= 1; // Decremento
     ```

Lembre-se de que, ao trabalhar com números de ponto flutuante, é importante considerar as peculiaridades associadas à representação desses números em computadores e os possíveis erros de arredondamento. Além disso, ao lidar com incremento e decremento, use `mut` para tornar a variável mutável.
