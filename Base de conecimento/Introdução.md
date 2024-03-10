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

##  Borrow Checker

O Borrow Checker (verificador de empréstimos) é uma parte fundamental da abordagem de segurança de memória em Rust. Ele faz parte do sistema de propriedade de empréstimos (ownership) e é responsável por garantir que as regras de empréstimo (borrowing) sejam seguidas corretamente durante o tempo de compilação.

O conceito principal por trás do Borrow Checker está relacionado à propriedade de empréstimo em Rust, que é uma maneira única e eficaz de gerenciar a segurança de memória sem a necessidade de coletor de lixo. Em Rust, uma variável possui a "propriedade" de seus dados, o que significa que é responsável por liberar esses dados quando não precisar mais deles. No entanto, Rust também permite empréstimos temporários de variáveis para outras partes do código, sem transferir a propriedade. Esses empréstimos podem ser mutáveis ou imutáveis.

O Borrow Checker verifica as seguintes regras:

1. **Regras de Mutabilidade:**
   - Uma variável só pode ter um empréstimo mutável em um determinado escopo.
   - Pode haver vários empréstimos imutáveis de uma variável em um mesmo escopo.

2. **Escopo do Empréstimo:**
   - Um empréstimo não pode durar mais do que a vida útil da variável original.
   - O sistema de lifetimes (tempo de vida) em Rust é usado para determinar a extensão dos empréstimos.

3. **Não há Referências Nulas:**
   - O sistema de empréstimos em Rust não permite referências nulas ou dangling references, eliminando assim erros comuns de acesso a memória inválida.

O Borrow Checker trabalha em conjunto com o sistema de lifetimes para analisar o código-fonte e garantir que todas as referências (empréstimos) sejam válidas e não causem problemas de segurança de memória. Isso é feito em tempo de compilação, o que significa que muitos erros relacionados a empréstimos e propriedade são capturados antes mesmo de o código ser executado.

Embora o Borrow Checker possa, em alguns casos, parecer rigoroso demais para programadores que estão acostumados com outras linguagens, ele desempenha um papel crucial na prevenção de erros de acesso à memória e no fornecimento de garantias de segurança robustas, sem a necessidade de um coletor de lixo em tempo de execução. Isso é particularmente valioso em sistemas de baixo nível, onde o controle direto sobre a memória é essencial.
