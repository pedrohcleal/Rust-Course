# Intro part2

## Manipução de strings

Em Rust, a manipulação de strings é realizada principalmente usando o tipo de dados `String`, que é uma string de texto dinâmica, e métodos associados a ela. Aqui estão alguns dos métodos comuns associados ao tipo `String` em Rust:

1. **new()**:
   - O método `new()` cria uma nova instância vazia de uma string.

   ```rust
   let mut my_string = String::new();
   ```

2. **from()**:
   - O método `from()` converte um tipo implementando o trait `ToString` em uma `String`.

   ```rust
   let my_number = 42;
   let my_string = String::from(my_number.to_string());
   ```

3. **push_str()**:
   - O método `push_str()` adiciona uma string ao final de outra.

   ```rust
   let mut greeting = String::from("Hello, ");
   greeting.push_str("world!");
   ```

4. **push()**:
   - O método `push()` adiciona um caractere ao final da string.

   ```rust
   let mut my_string = String::from("abc");
   my_string.push('d');
   ```

5. **pop()**:
   - O método `pop()` remove e retorna o último caractere da string.

   ```rust
   let mut my_string = String::from("abc");
   let popped_char = my_string.pop();
   ```

6. **replace()**:
   - O método `replace()` substitui todas as ocorrências de uma substring por outra.

   ```rust
   let original = String::from("Hello, world!");
   let modified = original.replace("world", "Rust");
   ```

7. **trim()**:
   - O método `trim()` remove espaços em branco no início e no final da string.

   ```rust
   let padded_string = "   Rust   ";
   let trimmed_string = padded_string.trim();
   ```

8. **len()** e **is_empty()**:
   - `len()` retorna o número de bytes na string, enquanto `is_empty()` verifica se a string está vazia.

   ```rust
   let my_string = String::from("Hello");
   let length = my_string.len();
   let is_empty = my_string.is_empty();
   ```

9. **split()** e **join()**:
   - `split()` divide a string em substrings com base em um delimitador, enquanto `join()` concatena uma coleção de substrings em uma única string.

   ```rust
   let my_string = String::from("apple,orange,banana");
   let fruits: Vec<&str> = my_string.split(',').collect();
   let joined_string = fruits.join("-");
   ```

Estes são apenas alguns dos muitos métodos associados ao tipo `String` em Rust. A documentação oficial de Rust fornece informações mais detalhadas sobre esses métodos e outros recursos relacionados às strings: [std::string::String](https://doc.rust-lang.org/std/string/struct.String.html).

## `parse` e `unwrap`

Em Rust, os termos "parse" e "unwrap" estão frequentemente relacionados ao trabalho com tipos de dados que podem ser convertidos de uma representação de string para um tipo específico ou para lidar com opções que podem ou não conter um valor. Vamos abordar cada termo separadamente:

1. **Parse:**
   - O método `parse` em Rust é frequentemente associado à conversão de strings em tipos de dados específicos. Por exemplo, ao ler uma entrada de usuário como uma string e tentar convertê-la para um número.
   - Exemplo de uso:
     ```rust
     let numero_str = "42";
     let numero: i32 = numero_str.parse().expect("Falha ao converter para número");
     ```

2. **Unwrap:**
   - O método `unwrap` é usado para "desembrulhar" ou extrair o valor de uma `Option` ou `Result`. No contexto de `Option`, `unwrap` retorna o valor se estiver presente, ou entra em pânico se for `None`. No contexto de `Result`, `unwrap` retorna o valor se for `Ok`, ou entra em pânico se for `Err`.
   - Exemplo de uso com `Option`:
     ```rust
     let opcional: Option<i32> = Some(42);
     let valor = opcional.unwrap();
     ```
   - Exemplo de uso com `Result`:
     ```rust
     let resultado: Result<i32, &str> = Ok(42);
     let valor = resultado.unwrap();
     ```

É importante notar que o uso excessivo de `unwrap` pode tornar o código menos robusto, pois panics (entrar em pânico) não são tratados de maneira elegante. Em situações reais, considera-se preferível usar métodos como `match`, `if let`, ou métodos combinadores como `map` e `and_then` para lidar com opções ou resultados de maneira mais segura e controlada.


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

## Estrutura de dados para coleção de elementos

Em Rust, existem vários tipos de estruturas de dados que podem ser usadas para representar coleções de elementos, cada uma com suas próprias características e finalidades. As principais estruturas para armazenar coleções são:

1. **Arrays Fixos:**
   - Em Rust, um array é uma coleção fixa de elementos do mesmo tipo com um tamanho específico definido em tempo de compilação.
   - Sintaxe: `let array: [T; N] = [elemento1, elemento2, ...];`
   - Exemplo:
     ```rust
     let array: [i32; 3] = [1, 2, 3];
     ```

2. **Slices:**
   - Um slice é uma visualização de uma parte de uma coleção, como um array ou vetor.
   - Sintaxe: `let slice: &[T] = &colecao[indice_inicial..indice_final];`
   - Exemplo:
     ```rust
     let vetor = vec![1, 2, 3, 4, 5];
     let slice: &[i32] = &vetor[1..4];
     ```

3. **Vectores (Vec):**
   - Um vector é uma coleção dinâmica e redimensionável de elementos do mesmo tipo.
   - Sintaxe: `let vetor: Vec<T> = vec![elemento1, elemento2, ...];`
   - Exemplo:
     ```rust
     let vetor: Vec<i32> = vec![1, 2, 3];
     ```

4. **Strings:**
   - Strings são coleções de caracteres Unicode. Em Rust, a string mais comum é o tipo `String`, que é uma coleção dinâmica e modificável de caracteres.
   - Sintaxe: `let string: String = String::from("Texto");`
   - Exemplo:
     ```rust
     let string: String = String::from("Hello, Rust!");
     ```

5. **Linked Lists (Listas Ligadas):**
   - Rust não possui uma implementação padrão de listas ligadas na biblioteca padrão, mas você pode criar suas próprias implementações usando structs e enums.

6. **Deque:**
   - A coleção `VecDeque` fornece uma deque (double-ended queue) implementada como um vetor dinâmico que permite inserção e remoção eficientes nas extremidades.
   - Sintaxe: `let deque: VecDeque<T> = VecDeque::new();`
   - Exemplo:
     ```rust
     use std::collections::VecDeque;

     let mut deque: VecDeque<i32> = VecDeque::new();
     deque.push_front(0);
     deque.push_back(1);
     ```

7. **HashMaps:**
   - O tipo `HashMap` é uma coleção associativa que mapeia chaves a valores, proporcionando acesso eficiente aos elementos por meio de uma função de hash.
   - Sintaxe: `let hashmap: HashMap<K, V> = HashMap::new();`
   - Exemplo:
     ```rust
     use std::collections::HashMap;

     let mut hashmap: HashMap<&str, i32> = HashMap::new();
     hashmap.insert("um", 1);
     hashmap.insert("dois", 2);
     ```

Estas são algumas das principais estruturas de dados em Rust. A escolha entre elas dependerá das necessidades específicas do seu programa, como tamanho fixo versus dinâmico, eficiência em termos de tempo de execução, e se você precisa de uma associação de chave-valor.
