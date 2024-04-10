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

## Tuplas

Em Rust, uma tupla é uma estrutura de dados que pode conter um número fixo de elementos de diferentes tipos. Ao contrário de arrays, onde todos os elementos devem ter o mesmo tipo e um tamanho fixo, uma tupla pode agrupar valores heterogêneos.

A sintaxe para criar uma tupla em Rust envolve listar os elementos entre parênteses e separá-los por vírgulas. Por exemplo:

```rust
let minha_tupla = (42, "hello", 3.14);
```

Esta linha de código cria uma tupla contendo um inteiro, uma string e um número de ponto flutuante.

As tuplas podem ser desestruturadas para acessar seus elementos individuais. Isso é feito atribuindo a tupla a uma variável e usando padrões de correspondência para extrair os valores. Por exemplo:

```rust
let (x, y, z) = minha_tupla;
println!("x: {}, y: {}, z: {}", x, y, z);
```

Este código extrai os elementos da tupla `minha_tupla` e os atribui às variáveis `x`, `y` e `z`.

As tuplas em Rust são frequentemente usadas quando você quer retornar múltiplos valores de uma função ou quando precisa de uma estrutura simples para agrupar valores de tipos diferentes. No entanto, como as tuplas não têm nomes de campo, seu uso pode ser menos expressivo do que usar structs em certos casos.

## Enum

Em Rust, `enum` é uma construção poderosa que permite definir um tipo de dados que pode ter um de vários valores distintos. Isso é útil para representar um conjunto finito de opções ou estados.

Aqui está um exemplo simples de enum em Rust:

```rust
enum Cor {
    Vermelho,
    Verde,
    Azul,
}
```

Neste exemplo, `Cor` é um enum que pode ter três valores possíveis: `Vermelho`, `Verde` e `Azul`.

Enums podem ter valores associados, o que permite armazenar dados junto com cada variante. Por exemplo:

```rust
enum Coordenada {
    PontoCartesiano(f64, f64),
    PontoPolar { raio: f64, angulo: f64 },
}
```

Aqui, `Coordenada` é um enum com duas variantes: `PontoCartesiano`, que contém duas coordenadas cartesianas, e `PontoPolar`, que contém um raio e um ângulo.

Enums em Rust podem ser usados com correspondência de padrões para inspecionar e manipular os diferentes valores possíveis. Por exemplo:

```rust
let minha_cor = Cor::Verde;

match minha_cor {
    Cor::Vermelho => println!("A cor é vermelha"),
    Cor::Verde => println!("A cor é verde"),
    Cor::Azul => println!("A cor é azul"),
}
```

Além disso, enums podem ter métodos associados, permitindo a definição de comportamentos específicos para cada variante. Isso os torna úteis para implementar padrões de tipo de soma (sum types), onde um valor pode ser uma de várias coisas.

Em resumo, enums em Rust são uma ferramenta versátil para representar tipos de dados que podem ter uma variedade de valores possíveis, e eles desempenham um papel importante na expressividade e segurança do Rust.

## match + enum

Em Rust, você pode usar `match` com `enum` de várias maneiras. `Enum` é uma estrutura de dados poderosa que permite definir tipos que podem ter diferentes variantes. Aqui está um exemplo simples de como você pode usar `match` com `enum` em Rust:

```rust
// Definindo um enum chamado `DiaDaSemana`
enum DiaDaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

// Função que usa match com enum
fn dia_util(dia: DiaDaSemana) -> bool {
    // Usando o match para lidar com cada variante do enum
    match dia {
        DiaDaSemana::Segunda => true,
        DiaDaSemana::Terca => true,
        DiaDaSemana::Quarta => true,
        DiaDaSemana::Quinta => true,
        DiaDaSemana::Sexta => true,
        DiaDaSemana::Sabado => false,
        DiaDaSemana::Domingo => false,
    }
}

fn main() {
    let hoje = DiaDaSemana::Segunda;
    println!("Hoje é um dia útil? {}", dia_util(hoje));
}
```

Neste exemplo, temos um `enum` chamado `DiaDaSemana`, que define os dias da semana. Em seguida, temos uma função chamada `dia_util`, que recebe um `DiaDaSemana` como argumento e retorna verdadeiro se o dia for útil (de segunda a sexta-feira) ou falso se for um fim de semana (sábado ou domingo). Dentro da função `dia_util`, usamos `match` para lidar com cada variante do `enum`, retornando verdadeiro para os dias úteis e falso para os dias não úteis.

Você também pode usar `match` para extrair valores associados a variantes do `enum`. Por exemplo, se as variantes do `enum` tiverem dados associados, você pode usá-los desta forma:

```rust
// Definindo um enum com dados associados
enum Evento {
    Mensagem(String),
    Numero(i32),
}

fn processar_evento(evento: Evento) {
    match evento {
        Evento::Mensagem(mensagem) => {
            println!("Mensagem recebida: {}", mensagem);
        }
        Evento::Numero(numero) => {
            println!("Número recebido: {}", numero);
        }
    }
}

fn main() {
    let evento_1 = Evento::Mensagem(String::from("Olá, mundo!"));
    let evento_2 = Evento::Numero(42);

    processar_evento(evento_1);
    processar_evento(evento_2);
}
```

Neste exemplo, o `enum` `Evento` tem duas variantes, `Mensagem` e `Numero`, ambas com dados associados (`String` e `i32`, respectivamente). A função `processar_evento` usa `match` para lidar com cada variante e extrair os dados associados, imprimindo-os na tela.
