# Intro part2

## Manipução de strings

Em Rust, a manipulação de strings envolve uma série de métodos e operações que podem ser realizadas para atender a diversas necessidades. Vamos explorar cada um dos itens da sua lista:

### Concatenação de strings:

Para concatenar strings em Rust, você pode usar o operador `+` ou o método `format!`. Aqui está um exemplo com o operador `+`:

```rust
let s1 = "Hello";
let s2 = " World!";
let concatenated = s1.to_string() + s2;
println!("{}", concatenated);
```

Alternativamente, usando o método `format!`:

```rust
let s1 = "Hello";
let s2 = " World!";
let concatenated = format!("{}{}", s1, s2);
println!("{}", concatenated);
```

### Tamanho e informações sobre strings:

Para obter o tamanho de uma string em Rust, você pode usar o método `len()`:

```rust
let my_string = "Hello, Rust!";
let length = my_string.len();
println!("Tamanho da string: {}", length);
```

### Acesso a caracteres e fatiamento de strings:

Para acessar caracteres individuais ou fatiar uma string, você pode usar a notação de índice e a função `slice`:

```rust
let my_string = "Rust Programming";
let first_char = my_string.chars().next(); // Obtém o primeiro caractere
let sliced_str = &my_string[5..11]; // Fatiamento de 5 a 10
```

### Busca e substituição:

Para buscar substrings ou substituir parte de uma string, você pode usar métodos como `contains`, `find`, `replace`, entre outros:

```rust
let my_string = "Rust Programming";
let contains_rust = my_string.contains("Rust");
let index_of_programming = my_string.find("Programming");

// Substituir "Programming" por "Language"
let replaced_string = my_string.replace("Programming", "Language");
```

### Separação e junção:

Para dividir uma string com base em um delimitador ou juntar várias strings, você pode usar os métodos `split` e `join`:

```rust
let my_string = "Rust,Programming,Language";
let parts: Vec<&str> = my_string.split(',').collect(); // Separação
let joined_string = parts.join("-"); // Junção com "-"
```

### Remoção de espaços em branco:

Para remover espaços em branco do início e do final de uma string, você pode usar os métodos `trim`, `trim_start`, e `trim_end`:

```rust
let my_string = "   Rust Programming   ";
let trimmed_string = my_string.trim();
let trimmed_start = my_string.trim_start();
let trimmed_end = my_string.trim_end();
```

### Verificação de tipo de caracteres:

Para verificar o tipo de caracteres em uma string, você pode usar métodos como `is_alphabetic`, `is_numeric`, `is_whitespace`, etc.:

```rust
let my_string = "Rust123";
let alphabetic = my_string.chars().all(char::is_alphabetic);
let numeric = my_string.chars().all(char::is_numeric);
```

Estes são apenas alguns exemplos dos métodos disponíveis para manipulação de strings em Rust. A linguagem fornece uma variedade de ferramentas para lidar eficientemente com operações relacionadas a strings.

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

## "casting" e "coercion"

### Casting em Rust:

Para realizar um casting em Rust, você pode usar os operadores `as` ou `transmute`. O `as` é usado para realizar conversões de tipo seguras, enquanto o `transmute` permite a conversão bruta entre tipos, mas deve ser usado com extrema cautela.

#### Usando `as` para Casting:

```rust
fn main() {
    let inteiro: i32 = 42;
    let ponto_flutuante: f64 = inteiro as f64;
    
    println!("Inteiro: {}", inteiro);
    println!("Ponto Flutuante: {}", ponto_flutuante);
}
```

#### Usando `transmute` para Casting:

```rust
fn main() {
    let inteiro: i32 = 42;
    let ponto_flutuante: f64;

    unsafe {
        ponto_flutuante = std::mem::transmute(inteiro);
    }

    println!("Inteiro: {}", inteiro);
    println!("Ponto Flutuante: {}", ponto_flutuante);
}
```

**Nota:** O uso de `transmute` pode ser perigoso e é geralmente desencorajado, pois pode levar a comportamentos indefinidos se não for usado com cuidado.

### Uncasting (Coercion) em Rust:

Em Rust, você pode usar coerções para converter automaticamente entre tipos relacionados quando a conversão é segura. Isso é feito implicitamente pela linguagem.

```rust
fn main() {
    let inteiro: i32 = 42;
    let ponto_flutuante: f64 = inteiro as f64;

    println!("Inteiro: {}", inteiro);
    println!("Ponto Flutuante: {}", ponto_flutuante);

    // Rust realiza a coerção automaticamente quando apropriado
    let resultado: f64 = inteiro.into();
    println!("Resultado: {}", resultado);
}
```

Neste exemplo, `into()` é usado para realizar uma coerção automática de `i32` para `f64`.

Lembre-se de que, em Rust, é sempre preferível usar coerções seguras de tipos sempre que possível para garantir a segurança do programa. O uso de `transmute` e operações de casting inseguras deve ser limitado a situações em que você está absolutamente certo do que está fazendo e pode garantir a segurança da operação.

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
