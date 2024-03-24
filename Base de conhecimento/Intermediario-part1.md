# Rust intermediario

## usize

Em Rust, `usize` é um tipo de dado fundamental que representa o tamanho de um endereço de memória na arquitetura atual do sistema. Isso significa que `usize` é uma parte do sistema de tipos de Rust e é usado para representar o tamanho de objetos na memória, índices de arrays e vetores, e muitas outras operações que envolvem tamanhos de memória e índices.

A especificação exata de `usize` depende da arquitetura do sistema em que o código Rust está sendo executado. Em sistemas de 32 bits, `usize` geralmente é um inteiro não assinado de 32 bits, enquanto em sistemas de 64 bits, `usize` é geralmente um inteiro não assinado de 64 bits. Isso permite que o tipo `usize` seja capaz de representar o tamanho total da memória endereçável pelo sistema.

A principal razão para usar `usize` em vez de tipos de dados como `u32` ou `u64` é garantir portabilidade do código. Ao utilizar `usize`, você pode escrever código que é independente da arquitetura do sistema, tornando-o mais flexível e menos propenso a erros ao migrar entre diferentes plataformas.

Aqui estão alguns exemplos de uso de `usize` em Rust:

1. **Índices de Arrays e Slices**:
   ```rust
   let array: [i32; 5] = [1, 2, 3, 4, 5];
   let index: usize = 2;
   println!("Valor no índice {}: {}", index, array[index]);
   ```

2. **Tamanho de Estruturas de Dados**:
   ```rust
   struct Pessoa {
       nome: String,
       idade: usize,
   }
   ```

3. **Operações com Ponteiros**:
   ```rust
   let ponteiro: *mut i32 = 0x7fff00000000 as *mut i32;
   let offset: usize = 4;
   unsafe {
       *ponteiro.offset(offset as isize) = 42;
   }
   ```

4. **Tamanhos de Memória**:
   ```rust
   let tamanho: usize = std::mem::size_of::<i32>();
   println!("Tamanho de um i32: {} bytes", tamanho);
   ```

Em resumo, `usize` é um tipo de dado fundamental em Rust que representa o tamanho de endereços de memória na arquitetura do sistema em que o código está sendo executado. Ele é amplamente utilizado para operações que envolvem tamanhos de memória e índices de arrays e vetores.
