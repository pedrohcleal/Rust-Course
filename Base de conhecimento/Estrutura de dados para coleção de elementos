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
