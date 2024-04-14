# Intro part2

## Conversão de tipos

Em Rust, a conversão de tipos, também conhecida como "casting", ocorre quando você deseja converter um valor de um tipo para outro. Existem várias formas de conversão de tipos em Rust:

1. **Conversão Implícita**: Rust suporta muitas conversões implícitas entre tipos compatíveis. Por exemplo, você pode atribuir um valor de um tipo para uma variável de outro tipo compatível sem a necessidade de uma conversão explícita.

   ```rust
   let x: i32 = 5;
   let y: i64 = x; // Conversão implícita de i32 para i64
   ```

2. **Conversões Explícitas (Casting)**: Quando Rust não permite uma conversão implícita entre tipos, você pode usar os operadores de "casting" para converter explicitamente um tipo em outro. Existem duas formas principais de casting em Rust: "coercion" e "as".

   - **Coercion**: É uma conversão implícita que pode ocorrer em expressões de Rust. Por exemplo, você pode converter um tipo em outro compatível sem usar uma sintaxe explícita.

     ```rust
     let x: i32 = 5;
     let y: i64 = x; // Coercion de i32 para i64
     ```

   - **As**: É um operador que permite a conversão explícita entre tipos numéricos.

     ```rust
     let x: i32 = 5;
     let y: i64 = x as i64; // Conversão explícita de i32 para i64 usando "as"
     ```

3. **Conversão de String para Tipos Numéricos**: Rust fornece métodos para converter strings em tipos numéricos usando a função `parse()` ou `FromStr` trait.

   ```rust
   let s = "42";
   let x: i32 = s.parse().expect("Não é um número válido!"); // Usando parse()
   ```

4. **Conversão de Tipos Numéricos para String**: Você pode usar o método `to_string()` para converter tipos numéricos em strings.

   ```rust
   let x: i32 = 42;
   let s: String = x.to_string(); // Convertendo i32 para String
   ```

5. **Conversão de Tipos de Referência e Ponteiros**: Rust permite a conversão de tipos de referência e ponteiros usando a sintaxe `as`.

   ```rust
   let x: i32 = 42;
   let y: &i32 = &x;
   let z: *const i32 = y as *const i32; // Conversão de referência para ponteiro
   ```

Essas são as formas básicas de conversão de tipos em Rust, cobrindo as operações mais comuns necessárias em programas.


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

Em Rust, as strings são representadas como sequências de bytes UTF-8. Para trabalhar com caracteres individuais ou bytes dentro de uma string, Rust fornece métodos e funções úteis que facilitam a manipulação e processamento de strings. Aqui estão algumas das funções e métodos comumente usados:

### `.chars()`
O método `chars()` é usado para obter um iterador sobre os caracteres individuais de uma string. Ele retorna um iterador que produz caracteres Unicode. Aqui está um exemplo:

```rust
let minha_string = String::from("Olá");

for caractere in minha_string.chars() {
    println!("{}", caractere);
}
```

Este loop irá imprimir cada caractere da string `minha_string` em linhas separadas.

### `.as_bytes()`
O método `as_bytes()` é usado para obter um iterador sobre os bytes individuais que compõem a string. Ele retorna um iterador que produz bytes individuais na representação UTF-8 da string. Aqui está um exemplo:

```rust
let minha_string = String::from("Olá");

for byte in minha_string.as_bytes() {
    println!("{}", byte);
}
```

Este loop imprimirá cada byte da representação UTF-8 da string `minha_string`.

### `.len()`
O método `len()` retorna o número de bytes na representação UTF-8 da string. É importante observar que isso não é necessariamente o mesmo que o número de caracteres, pois alguns caracteres Unicode podem ocupar mais de um byte.

### `.is_empty()`
O método `is_empty()` retorna `true` se a string estiver vazia (sem nenhum caractere) e `false` caso contrário.

### `.to_uppercase()` e `.to_lowercase()`
Esses métodos retornam uma nova string com todos os caracteres convertidos para maiúsculas ou minúsculas, respectivamente.

Esses são apenas alguns exemplos das muitas funções e métodos disponíveis para trabalhar com strings em Rust. Eles são projetados para serem seguros e eficientes, ajudando os desenvolvedores a manipular strings de forma fácil e confiável.

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

## Intervalo

### Notação de Intervalo:

Rust oferece uma notação de intervalo usando o operador `..` para criar intervalos abertos no final (excluindo o último valor) e `..=` para intervalos fechados no final (incluindo o último valor).

#### Exemplo de Intervalo Aberto:

```rust
fn main() {
    for i in 1..5 {
        println!("{}", i);
    }
}
```

Este exemplo imprimirá os valores de 1 a 4, excluindo 5.

#### Exemplo de Intervalo Fechado:

```rust
fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }
}
```

Este exemplo imprimirá os valores de 1 a 5, incluindo 5.

### Funções Relacionadas a Intervalos:

Rust também possui métodos e funções relacionadas a intervalos. Por exemplo, a função `contains` pode ser usada para verificar se um valor está dentro de um determinado intervalo.

```rust
fn main() {
    let intervalo = 1..=5;
    let valor = 3;

    if intervalo.contains(&valor) {
        println!("O valor {} está dentro do intervalo.", valor);
    } else {
        println!("O valor {} está fora do intervalo.", valor);
    }
}
```

Este exemplo verifica se o valor 3 está dentro do intervalo de 1 a 5.

Em resumo, embora Rust não tenha um tipo de dados específico chamado "range", a notação de intervalo e as funções relacionadas permitem trabalhar eficientemente com intervalos de valores em várias situações.

