# Atributos em Rust

## Intro

Em Rust, os atributos, ou "attributes", são metadados que fornecem informações adicionais sobre vários elementos do código, como funções, structs, enums, traits e até mesmo módulos inteiros. Os atributos são expressos usando a sintaxe `#[atributo]` e podem ser colocados acima do item que estão afetando.

Existem diferentes tipos de atributos em Rust, incluindo:

1. **Atributos Derivados:** Eles são usados para gerar automaticamente implementações de certos traços para tipos de dados personalizados. Um exemplo comum é o atributo `#[derive(Debug)]`, que automaticamente implementa a trait `Debug`, permitindo que o tipo seja formatado para depuração.

   ```rust
   #[derive(Debug)]
   struct Pessoa {
       nome: String,
       idade: u32,
   }
   ```

2. **Atributos do compilador:** São usados para controlar o comportamento do compilador. Por exemplo, o atributo `#[warn(dead_code)]` instrui o compilador a emitir avisos quando detecta código não utilizado.

   ```rust
   #[warn(dead_code)]
   fn funcao_nao_utilizada() {
       // código não utilizado
   }
   ```

3. **Atributos para controle de visibilidade:** São usados para controlar a visibilidade de itens em um módulo. O atributo `#[doc(hidden)]`, por exemplo, esconde um item da documentação gerada.

   ```rust
   #[doc(hidden)]
   fn funcao_privada() {
       // Esta função será ocultada na documentação
   }
   ```

4. **Atributos de Metadados:** Eles podem ser usados para associar metadados a um item de código. Por exemplo, o atributo `#[cfg(target_os = "linux")]` instrui o compilador a incluir ou excluir o código dependendo do sistema operacional de destino.

   ```rust
   #[cfg(target_os = "linux")]
   fn apenas_para_linux() {
       // Este código será compilado apenas para sistemas operacionais Linux
   }
   ```

5. **Atributos Customizados:** Você também pode criar seus próprios atributos personalizados para adicionar metadados específicos ao seu código. Isso é feito usando macros.

Os atributos em Rust são uma ferramenta poderosa para fornecer informações adicionais ao compilador e aos desenvolvedores sobre o código, ajudando na organização, na documentação e no controle de compilação condicional.
