# Coding

## .iter()

Claro! Existem muitos métodos úteis que podem ser combinados com `iter()` e `filter()` para resolver desafios de lógica em Rust. Aqui estão alguns exemplos:

1. **map()**: Transforma cada elemento do iterador em outro valor, com base em uma função especificada. Útil para aplicar uma operação a cada elemento do iterador.

```rust
let numeros = vec![1, 2, 3, 4, 5];
let quadrados: Vec<i32> = numeros.iter().map(|&x| x * x).collect();
```

2. **any()**: Verifica se algum dos elementos do iterador satisfaz uma determinada condição. Retorna `true` se pelo menos um elemento satisfizer a condição.

```rust
let numeros = vec![1, 2, 3, 4, 5];
let tem_numero_par = numeros.iter().any(|&x| x % 2 == 0);
```

3. **all()**: Verifica se todos os elementos do iterador satisfazem uma determinada condição. Retorna `true` se todos os elementos satisfizerem a condição.

```rust
let numeros = vec![1, 2, 3, 4, 5];
let todos_impares = numeros.iter().all(|&x| x % 2 != 0);
```

4. **find()**: Retorna o primeiro elemento do iterador que satisfaz uma determinada condição. Útil para encontrar um elemento específico em uma coleção.

```rust
let numeros = vec![1, 2, 3, 4, 5];
let primeiro_par = numeros.iter().find(|&x| x % 2 == 0);
```

5. **enumerate()**: Produz tuplas de índice e valor para cada elemento do iterador. Útil para acessar índices juntamente com os valores.

```rust
let numeros = vec![10, 20, 30, 40, 50];
for (indice, valor) in numeros.iter().enumerate() {
    println!("O elemento na posição {} é {}", indice, valor);
}
```

6. **take()**: Retorna um iterador que produz os primeiros `n` elementos do iterador original.

```rust
let numeros = vec![1, 2, 3, 4, 5];
let primeiros_tres = numeros.iter().take(3);
```

### Outros métodos:

Claro! Além dos métodos `sum()` e `count()`, existem vários outros métodos úteis que podem ser combinados com `iter()` e `filter()` para resolver desafios de lógica em Rust. Aqui estão mais alguns exemplos:

1. **max() e min()**: Retorna o maior e o menor elemento do iterador, respectivamente.

```rust
let numeros = vec![1, 5, 2, 8, 3];
let maximo = numeros.iter().max();
let minimo = numeros.iter().min();
```

2. **sum()**: Calcula a soma de todos os elementos do iterador.

```rust
let numeros = vec![1, 2, 3, 4, 5];
let soma: i32 = numeros.iter().sum();
```

3. **count()**: Conta o número total de elementos no iterador.

```rust
let numeros = vec![1, 2, 3, 4, 5];
let total_elementos = numeros.iter().count();
```

4. **fold()**: Aplica uma função a cada elemento do iterador para acumular um valor.

```rust
let numeros = vec![1, 2, 3, 4, 5];
let soma = numeros.iter().fold(0, |acc, &x| acc + x);
```

5. **zip()**: Combina dois iteradores em um único iterador que produz tuplas de elementos correspondentes.

```rust
let numeros = vec![1, 2, 3];
let letras = vec!['a', 'b', 'c'];
let combinados = numeros.iter().zip(letras.iter());
```

6. **filter_map()**: Aplica uma função a cada elemento do iterador e filtra os resultados que são `Some`.

```rust
let opcoes = vec![Some(5), None, Some(10)];
let valores: Vec<i32> = opcoes.iter().filter_map(|&x| x).collect();
```

## .fold()

Em Rust, a função `fold()` é uma operação de alto nível que permite reduzir uma coleção a um único valor, aplicando uma função de acumulação a cada elemento da coleção. Esta função é especialmente útil quando você precisa iterar sobre uma coleção e acumular resultados.

A assinatura da função `fold()` em Rust é a seguinte:

```rust
fn fold<B, F>(self, init: B, f: F) -> B
where
    F: FnMut(B, Self::Item) -> B,
```

Aqui está o que cada um dos parâmetros significa:

- `self`: A coleção sobre a qual estamos iterando.
- `init`: O valor inicial do acumulador.
- `f`: A função de acumulação que será aplicada a cada elemento da coleção. Esta função recebe dois argumentos: o acumulador atual e o próximo elemento da coleção, e retorna o novo valor do acumulador.

Por exemplo, imagine que temos um vetor de números e queremos calcular a soma de todos esses números usando `fold()`:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("A soma dos números é: {}", sum);
}
```

Neste exemplo, `fold()` inicia o acumulador com `0` e, para cada elemento do vetor, adiciona o elemento ao acumulador. No final, `sum` conterá a soma de todos os números no vetor.

A função `fold()` é uma ferramenta poderosa para trabalhar com coleções em Rust, permitindo realizar operações de redução de forma concisa e eficiente.

## Outros exemplos:

Claro! Aqui estão alguns outros exemplos de como a função `fold()` pode ser usada em Rust:

1. **Multiplicação de todos os elementos de um vetor**:
   ```rust
   let numbers = vec![2, 3, 4, 5];
   let product = numbers.iter().fold(1, |acc, &x| acc * x);
   println!("O produto dos números é: {}", product);
   ```

2. **Concatenação de uma lista de strings**:
   ```rust
   let words = vec!["Olá", "mundo", "Rust"];
   let concatenated = words.iter().fold(String::new(), |acc, &x| acc + x);
   println!("Concatenado: {}", concatenated);
   ```

3. **Calcular a média de uma lista de números**:
   ```rust
   let numbers = vec![10, 20, 30, 40, 50];
   let (sum, count) = numbers.iter().fold((0, 0), |(acc_sum, acc_count), &x| (acc_sum + x, acc_count + 1));
   let average = sum as f64 / count as f64;
   println!("A média dos números é: {}", average);
   ```

4. **Encontrar o maior número em uma lista**:
   ```rust
   let numbers = vec![10, 5, 25, 30, 15];
   let max_number = numbers.iter().fold(i32::MIN, |max, &x| if x > max { x } else { max });
   println!("O maior número é: {}", max_number);
   ```

5. **Contagem de ocorrências de cada elemento em uma lista**:
   ```rust
   let numbers = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
   let counts = numbers.iter().fold(std::collections::HashMap::new(), |mut acc, &x| {
       *acc.entry(x).or_insert(0) += 1;
       acc
   });
   println!("Contagem de ocorrências: {:?}", counts);
   ```

Esses são apenas alguns exemplos de como a função `fold()` pode ser utilizada para diversas operações de redução em Rust. Ela oferece uma maneira concisa e expressiva de processar coleções e calcular resultados agregados.

## .map()

Em Rust, o método `.map()` é uma função de ordem superior que está disponível para tipos que implementam o trait `Iterator`. Ele permite transformar os elementos de um iterador de uma forma flexível e concisa. Quando você chama `.map()`, você fornece uma função de transformação que será aplicada a cada elemento do iterador, produzindo um novo iterador com os resultados dessas transformações.

A assinatura básica do método `.map()` é a seguinte:

```rust
fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> B,
```

- `B` é o tipo de dado que será produzido após a transformação.
- `F` é o tipo da função de transformação que será aplicada a cada elemento do iterador.
- `Self::Item` é o tipo de elemento contido no iterador original.

Aqui está um exemplo simples de como você pode usar `.map()` em Rust:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squared_numbers: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("{:?}", squared_numbers); // Output: [1, 4, 9, 16, 25]
}
```

Neste exemplo, `numbers.iter()` cria um iterador sobre os elementos de `numbers`. O `.map(|&x| x * x)` aplica a função de transformação que multiplica cada elemento por ele mesmo. Finalmente, `.collect()` coleta os resultados transformados em um novo `Vec<i32>` chamado `squared_numbers`.

É importante notar que `.map()` não modifica o iterador original; ele retorna um novo iterador que produzirá os resultados transformados quando iterado sobre. Isso ajuda a manter a segurança e imutabilidade em Rust, já que não há efeitos colaterais na coleção original.

### Exemplos:

Claro! Aqui estão alguns exemplos adicionais de como você pode usar o método `.map()` em Rust:

1. **Conversão de tipos**:
   
   Você pode usar `.map()` para converter os elementos de um tipo para outro. Por exemplo, converter uma lista de strings em uma lista de seus comprimentos:

   ```rust
   let strings = vec!["hello", "world", "rust"];
   let lengths: Vec<usize> = strings.iter().map(|s| s.len()).collect();
   println!("{:?}", lengths); // Output: [5, 5, 4]
   ```

2. **Aplicação de funções customizadas**:

   Você pode definir suas próprias funções e aplicá-las com `.map()`. Por exemplo, aplicar uma função que adiciona um prefixo a cada string em uma lista:

   ```rust
   fn add_prefix(s: &str) -> String {
       format!("Prefix: {}", s)
   }

   let strings = vec!["hello", "world", "rust"];
   let prefixed: Vec<String> = strings.iter().map(|&s| add_prefix(s)).collect();
   println!("{:?}", prefixed); // Output: ["Prefix: hello", "Prefix: world", "Prefix: rust"]
   ```

3. **Transformação de estruturas**:

   Você pode usar `.map()` para transformar os elementos de uma estrutura de dados complexa. Por exemplo, extrair apenas um campo de uma lista de structs:

   ```rust
   #[derive(Debug)]
   struct Person {
       name: String,
       age: u32,
   }

   let people = vec![
       Person { name: "Alice".to_string(), age: 30 },
       Person { name: "Bob".to_string(), age: 25 },
   ];

   let names: Vec<String> = people.iter().map(|p| p.name.clone()).collect();
   println!("{:?}", names); // Output: ["Alice", "Bob"]
   ```

4. **Composição de funções**:

   Você pode encadear várias chamadas `.map()` para executar transformações mais complexas. Por exemplo, converter uma lista de números em uma lista de seus quadrados e, em seguida, filtrar os números pares:

   ```rust
   let numbers = vec![1, 2, 3, 4, 5];
   let even_squared: Vec<i32> = numbers.iter()
                                      .map(|&x| x * x)
                                      .filter(|&x| x % 2 == 0)
                                      .collect();
   println!("{:?}", even_squared); // Output: [4, 16]
   ```

Esses são apenas alguns exemplos para ilustrar a versatilidade do método `.map()` em Rust. Ele é uma ferramenta poderosa para transformar e manipular coleções de forma elegante e eficiente.
