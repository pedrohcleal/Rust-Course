# Estrutura de dados para coleção de elementos

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

## Arrays em Rust

Em Rust, arrays servem como estruturas de dados cruciais para armazenar coleções de elementos do mesmo tipo. Eles oferecem acesso eficiente e direto aos seus elementos, tornando-os ferramentas valiosas para diversas tarefas de programação.

**Conceitos-chave:**

* **Tamanho fixo:** Ao contrário de linguagens como Python, os arrays em Rust possuem tamanho fixo definido na compilação. Isso garante segurança de memória e otimização de desempenho.
* **Tipagem estática:** Cada elemento do array possui um tipo específico, como `i32`, `String` ou structs personalizadas. Essa característica garante segurança de tipos e previne erros em tempo de execução.
* **Indexação:** Os elementos do array são acessados através de índices, que começam em zero. A indexação permite manipulação individual de cada elemento.
* **Tipos de arrays:**
    * **Arrays simples:** Armazenam elementos do mesmo tipo.
    * **Slices:** Subconjuntos de um array original, permitindo manipulação de partes específicas.
    * **Arrays multidimensionais:** Estruturam dados em múltiplas dimensões, como matrizes.

**Declaração e inicialização:**

```rust
// Array de 5 elementos i32
let mut numeros: [i32; 5] = [1, 2, 3, 4, 5];

// Array de 3 strings
let frutas: [&str; 3] = ["Maçã", "Banana", "Laranja"];

// Inicialização com valor padrão
let zeros: [i32; 10] = [0; 10];
```

**Acesso e manipulação:**

```rust
// Acessando o segundo elemento
let segundo_numero = numeros[1];

// Alterando o valor do terceiro elemento
numeros[2] = 10;

// Iteração sobre o array
for numero in &numeros {
    println!("{}", numero);
}
```

**Slices:**

```rust
// Slice dos 2 primeiros elementos
let slice_primeiros_dois = &numeros[0..2];

// Slice do terceiro ao último elemento
let slice_resto = &numeros[2..];

// Revertendo a ordem do array
let mut numeros_revertidos = numeros.to_vec();
numeros_revertidos.reverse();
```

**Arrays multidimensionais:**

```rust
// Matriz 3x3 de i32
let matriz: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

// Acessando um elemento específico da matriz
let elemento_matriz = matriz[1][2];
```

**Vantagens:**

* **Eficiência:** Arrays em Rust oferecem acesso rápido e direto aos seus elementos, resultando em alto desempenho.
* **Segurança:** A tipagem estática garante segurança de tipos e previne erros em tempo de execução.
* **Flexibilidade:** Suporte a diferentes tipos de dados e estruturas multidimensionais.

**Desvantagens:**

* **Tamanho fixo:** O tamanho do array precisa ser definido na compilação, limitando a flexibilidade em alguns casos.
* **Gerenciamento de memória:** O programador é responsável por gerenciar a memória do array, o que pode gerar erros se não for feito corretamente.

**Recursos adicionais:**

* Documentação oficial: Documentação Rust Arrays: [https://doc.rust-lang.org/std/primitive.array.html](https://doc.rust-lang.org/std/primitive.array.html)

## Métodos com Arrays

Em Rust, os arrays são coleções de elementos do mesmo tipo com um tamanho fixo conhecido em tempo de compilação. Eles são declarados usando a sintaxe `[tipo; tamanho]`. Os métodos disponíveis para trabalhar com arrays em Rust incluem:

1. **len()**: Retorna o comprimento do array, ou seja, o número de elementos que ele contém.

   ```rust
   let arr = [1, 2, 3, 4, 5];
   println!("Comprimento do array: {}", arr.len());
   ```

2. **iter()**: Retorna um iterador sobre os elementos do array.

   ```rust
   let arr = [1, 2, 3, 4, 5];
   for element in arr.iter() {
       println!("{}", element);
   }
   ```

3. **iter_mut()**: Retorna um iterador mutável sobre os elementos do array, permitindo a modificação dos elementos.

   ```rust
   let mut arr = [1, 2, 3, 4, 5];
   for element in arr.iter_mut() {
       *element *= 2;
   }
   ```

4. **as_slice()**: Converte o array em uma fatia (slice), que é uma visão não mutável dos elementos do array.

   ```rust
   let arr = [1, 2, 3, 4, 5];
   let slice = arr.as_slice();
   ```

5. **as_mut_slice()**: Converte o array em uma fatia mutável, permitindo a modificação dos elementos do array.

   ```rust
   let mut arr = [1, 2, 3, 4, 5];
   let mut_slice = arr.as_mut_slice();
   ```

6. **into_iter()**: Consuma o array e retorna um iterador possuindo a propriedade do array.

   ```rust
   let arr = [1, 2, 3, 4, 5];
   for element in arr.into_iter() {
       println!("{}", element);
   }
   ```

7. **iter().enumerate()**: Retorna um iterador que produz tuplas contendo o índice e o valor de cada elemento do array.

   ```rust
   let arr = [1, 2, 3, 4, 5];
   for (index, value) in arr.iter().enumerate() {
       println!("Index: {}, Value: {}", index, value);
   }
   ```

8. **chunks()**: Divide o array em pedaços de um determinado tamanho.

   ```rust
   let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
   for chunk in arr.chunks(3) {
       println!("{:?}", chunk);
   }
   ```

Esses métodos fornecem maneiras convenientes de iterar sobre, acessar e modificar os elementos de um array em Rust.

## Vetores

Em Rust, vetores são coleções de elementos do mesmo tipo, com tamanho fixo e alocados na pilha. Eles são representados pelo tipo `Vec<T>`, onde `T` é o tipo dos elementos que o vetor contém. Aqui estão alguns aspectos importantes sobre vetores em Rust:

### 1. Declaração e Inicialização:

Vetores podem ser declarados e inicializados de várias maneiras:

```rust
// Vetor vazio de inteiros
let vetor_vazio: Vec<i32> = Vec::new();

// Inicialização usando a macro vec!
let vetor_macro = vec![1, 2, 3, 4, 5];

// Inicialização com elementos repetidos
let vetor_repetido = vec![0; 5]; // [0, 0, 0, 0, 0]
```

### 2. Acesso a Elementos:

Os elementos de um vetor podem ser acessados usando a notação de índice, começando do índice 0:

```rust
let vetor = vec![10, 20, 30, 40, 50];

println!("Primeiro elemento: {}", vetor[0]); // Saída: 10
println!("Segundo elemento: {}", vetor[1]); // Saída: 20
```

### 3. Atualização de Elementos:

Os elementos de um vetor podem ser atualizados usando a notação de índice:

```rust
let mut vetor = vec![1, 2, 3];

vetor[1] = 5;

println!("{:?}", vetor); // Saída: [1, 5, 3]
```

### 4. Métodos e Funções Úteis:

Os vetores em Rust oferecem uma variedade de métodos e funções úteis, como `push`, `pop`, `len`, `contains`, `iter`, entre outros:

```rust
let mut vetor = vec![1, 2, 3];

// Adicionando um elemento ao final do vetor
vetor.push(4);

// Removendo o último elemento
let ultimo_elemento = vetor.pop();

// Obtendo o tamanho do vetor
let tamanho = vetor.len();

// Verificando se o vetor contém um determinado valor
let contem_tres = vetor.contains(&3);

// Iterando sobre os elementos do vetor
for elemento in &vetor {
    println!("{}", elemento);
}
```

### 5. Propriedades de Propriedade e Controle de Propriedade:

Os vetores em Rust são seguros em termos de propriedade. Eles têm propriedades de propriedade, o que significa que cada valor no vetor é de propriedade única, garantindo que não haja acesso simultâneo aos elementos.

```rust
let v1 = vec![1, 2, 3];
let v2 = v1; // Movendo a propriedade de v1 para v2
// println!("{:?}", v1); // Isso resultaria em um erro, pois v1 não é mais válido aqui
```
## Conversão Arrays <> Vetores:

Em Rust, a conversão entre arrays e vetores é bastante simples. Um array é uma coleção fixa de elementos do mesmo tipo, enquanto um vetor é uma coleção dinâmica de elementos do mesmo tipo que pode crescer ou encolher.

Para converter um array em um vetor, você pode simplesmente usar o método `to_vec()` disponível em arrays. Por exemplo:

```rust
let array = [1, 2, 3, 4, 5];
let vetor = array.to_vec();
```

Isso criará um novo vetor contendo os mesmos elementos do array.

Para converter um vetor em um array, é um pouco mais complicado, pois você precisa garantir que o vetor tenha o mesmo tamanho que o array. Isso geralmente é feito usando uma técnica conhecida como "slicing". Por exemplo:

```rust
let vetor = vec![1, 2, 3, 4, 5];
let array: [i32; 5] = vetor[..].try_into().unwrap();
```

Aqui, `vetor[..]` é um slice que cobre todos os elementos do vetor. Em seguida, `try_into()` tenta converter o slice em um array. Essa operação pode falhar se o vetor não tiver o mesmo tamanho que o array, então é necessário lidar com o resultado usando `unwrap()` ou tratando o erro adequadamente.

Essa é uma visão geral simplificada das conversões entre array e vetor em Rust.

## Strings

Em Rust, uma string é uma coleção de caracteres Unicode codificados em UTF-8. Rust fornece dois tipos principais de strings: `String` e `&str`. Ambos representam uma sequência de caracteres, mas diferem em sua natureza e uso.

1. **`String`**:
   - `String` é uma string de propriedade, o que significa que ela possui alocação de memória e pode ser modificada.
   - Ela é mutável e pode crescer ou encolher conforme necessário.
   - Para criar uma `String`, você pode usar o método `String::from()` ou o literal de string `String::from("texto")`.
   - Exemplo:
     ```rust
     let mut s = String::from("Olá, mundo!");
     s.push_str(" Bem-vindo!");
     println!("{}", s); // Saída: Olá, mundo! Bem-vindo!
     ```

2. **`&str`**:
   - `&str` (também conhecido como fatia de string) é uma visualização emprestada de uma sequência de caracteres.
   - Ela não possui propriedade sobre os dados subjacentes e é imutável por padrão.
   - Pode ser criada a partir de um literal de string (`"texto"`) ou a partir de uma `String` existente por meio de referências (`&`).
   - Exemplo:
     ```rust
     let s: &str = "Olá, mundo!";
     ```

As strings em Rust são seguras contra condições de corrida (race conditions) e erros de memória, graças ao sistema de propriedade e ao sistema de tipos forte. Isso significa que o Rust rastreia cuidadosamente a vida útil das referências para garantir que as strings sejam manipuladas de forma segura e eficiente.

Além disso, Rust fornece uma ampla gama de métodos e funcionalidades para trabalhar com strings, incluindo operações de manipulação, pesquisa, concatenação, divisão e muito mais. Isso torna o trabalho com strings em Rust bastante poderoso e expressivo.

## Métodos com String

Em Rust, o tipo `String` oferece uma variedade de métodos para manipular e interagir com strings. Esses métodos permitem realizar operações como concatenação, busca, manipulação de caracteres, conversão de caixa, divisão e muito mais. Aqui está uma visão geral de alguns dos métodos mais comuns disponíveis para o tipo `String`:

1. **Manipulação de Conteúdo**:
   - `push_str(&str)`: Adiciona uma fatia de string à `String`.
   - `push(char)`: Adiciona um caractere à `String`.
   - `insert(index, &str)`: Insere uma fatia de string em uma posição específica na `String`.
   - `remove(index)`: Remove e retorna o caractere na posição especificada.
   - `replace(old, new)`: Substitui todas as ocorrências de uma substring por outra.
   - `truncate(len)`: Trunca a `String` para o comprimento especificado.
   - `clear()`: Remove todo o conteúdo da `String`.

2. **Manipulação de Caracteres**:
   - `chars()`: Retorna um iterador sobre os caracteres da `String`.
   - `bytes()`: Retorna um iterador sobre os bytes da `String`.
   - `as_bytes()`: Retorna uma referência para a representação em bytes da `String`.

3. **Operações de Pesquisa**:
   - `contains(&str)`: Verifica se a `String` contém a substring fornecida.
   - `starts_with(&str)`: Verifica se a `String` começa com a substring fornecida.
   - `ends_with(&str)`: Verifica se a `String` termina com a substring fornecida.
   - `find(&str)`: Retorna a posição da primeira ocorrência da substring fornecida.

4. **Concatenação e Divisão**:
   - `+` ou `format!()`: Concatena duas `String` ou uma `String` com outros tipos.
   - `split(&str)`: Divide a `String` em partes com base em um delimitador.

5. **Transformação e Conversão**:
   - `to_lowercase()` / `to_uppercase()`: Converte a `String` para minúsculas / maiúsculas.
   - `trim()`, `trim_start()`, `trim_end()`: Remove espaços em branco no início e/ou no final da `String`.
   - `parse()`: Converte a `String` em um tipo específico.

6. **Outros Métodos Úteis**:
   - `len()`: Retorna o comprimento da `String`.
   - `is_empty()`: Verifica se a `String` está vazia.
   - `capacity()`: Retorna a capacidade atual da `String`.

Esses são apenas alguns exemplos dos métodos disponíveis para manipulação de strings em Rust. Eles são parte da versatilidade que o Rust oferece ao trabalhar com strings, permitindo operações eficientes e seguras em tempo de compilação.
