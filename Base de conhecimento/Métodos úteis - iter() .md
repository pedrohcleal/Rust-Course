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

Esses são apenas alguns exemplos, mas existem muitos outros métodos disponíveis no Rust Standard Library que podem ser combinados de várias maneiras para resolver desafios de lógica de maneira eficiente e expressiva.
