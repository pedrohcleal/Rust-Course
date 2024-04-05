# Rocket

## Iniciar

Para iniciar o uso da biblioteca Rocket em um projeto Rust, você precisará adicionar a dependência correta ao seu arquivo `Cargo.toml`, que é o arquivo de manifesto do seu projeto Rust. Aqui estão os passos básicos:

1. Abra o seu projeto Rust no qual você deseja usar o Rocket, ou crie um novo projeto.

2. Abra o arquivo `Cargo.toml` do seu projeto.

3. Adicione a dependência Rocket sob a seção `[dependencies]`. Você pode especificar a versão desejada do Rocket. Por exemplo:

```toml
[dependencies]
rocket = "0.5.0"
```

4. Salve o arquivo `Cargo.toml`.

5. Em seguida, você precisa adicionar as macros `#[macro_use]` e o import `use rocket::routes;` no arquivo `main.rs` ou `lib.rs` do seu projeto. Aqui está um exemplo básico:

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::routes;

// Defina suas rotas aqui
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// O ponto de entrada da sua aplicação
fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
```

6. Agora, você pode começar a definir suas rotas dentro do arquivo `main.rs` (ou em módulos separados, se preferir) usando os atributos como `#[get]`, `#[post]`, etc.

7. Finalmente, compile e execute seu projeto Rust usando `cargo run`.

Esses são os passos básicos para iniciar o uso da biblioteca Rocket em um projeto Rust. A partir daqui, você pode explorar mais sobre as funcionalidades oferecidas pela Rocket e começar a construir sua aplicação web.

## Métodos Rocket

Rocket é um framework web para a linguagem de programação Rust, projetado para tornar o desenvolvimento web mais simples, seguro e produtivo. Ele oferece uma sintaxe intuitiva e expressiva, além de uma forte tipagem estática, aproveitando as características de segurança e desempenho que Rust proporciona. Abaixo estão alguns dos principais métodos e funcionalidades fornecidos pela biblioteca Rocket:

1. **#[get], #[post], #[put], #[delete]**:
   - Esses são os atributos (macros) utilizados para definir rotas HTTP para manipular requisições GET, POST, PUT e DELETE, respectivamente. Por exemplo:
     ```rust
     #[get("/hello")]
     fn hello() -> &'static str {
         "Hello, world!"
     }
     ```

2. **#[route]**:
   - Permite definir rotas HTTP personalizadas, combinando vários métodos HTTP sob a mesma rota. Por exemplo:
     ```rust
     #[route(GET, POST, "/custom")]
     fn custom_route() -> &'static str {
         "Custom route for both GET and POST requests"
     }
     ```

3. **#[param]**:
   - Utilizado para extrair parâmetros de URL em uma rota. Por exemplo:
     ```rust
     #[get("/user/<id>")]
     fn user(id: usize) -> String {
         format!("User ID: {}", id)
     }
     ```

4. **#[query]**:
   - Permite extrair parâmetros de consulta (query parameters) de uma requisição HTTP. Por exemplo:
     ```rust
     #[get("/search?<query>")]
     fn search(query: String) -> String {
         format!("Search query: {}", query)
     }
     ```

5. **#[form]**:
   - Similar ao #[query], mas usado para extrair dados de formulários HTTP. Por exemplo:
     ```rust
     #[post("/submit", data = "<form_data>")]
     fn submit(form_data: Form<Data>) -> String {
         format!("Submitted form: {:?}", form_data)
     }
     ```

6. **#[catch]**:
   - Permite definir funções de captura de erros que são chamadas quando ocorre um erro durante o processamento de uma rota. Por exemplo:
     ```rust
     #[catch(404)]
     fn not_found(req: &Request) -> String {
         format!("Error 404: Page '{}' not found", req.uri())
     }
     ```

Esses são apenas alguns dos principais métodos fornecidos pela biblioteca Rocket em Rust. Eles fornecem uma maneira elegante e eficiente de criar APIs web seguras e escaláveis em Rust.
