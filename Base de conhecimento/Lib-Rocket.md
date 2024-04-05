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

## tokio-postgres + Rocket

Para utilizar o `tokio-postgres` com o Rocket em uma aplicação Rust, você precisará integrar as duas bibliotecas. O `tokio-postgres` é uma biblioteca de acesso ao banco de dados PostgreSQL que é projetada para ser usada com o runtime `tokio`, enquanto o Rocket é um framework web. Aqui está um exemplo básico de como você pode integrar essas duas bibliotecas em um projeto:

1. Adicione as dependências ao seu arquivo `Cargo.toml`:

```toml
[dependencies]
rocket = "0.5.0"
tokio = { version = "1.0", features = ["full"] }
tokio-postgres = "0.7.2"
```

2. Crie uma conexão com o banco de dados PostgreSQL usando `tokio-postgres`. Aqui está um exemplo básico de como você pode fazer isso:

```rust
use tokio_postgres::{NoTls, Error};

async fn connect() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres dbname=mydatabase",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    Ok(client)
}
```

3. Em seguida, você pode usar essa função para executar consultas SQL dentro das rotas Rocket. Aqui está um exemplo de como você pode fazer isso:

```rust
#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;
use tokio_postgres::Row;

#[get("/users")]
async fn get_users(client: &tokio_postgres::Client) -> Result<Json<Vec<User>>, Status> {
    let rows = client.query("SELECT id, name FROM users", &[]).await;
    match rows {
        Ok(rows) => {
            let users: Vec<User> = rows.iter().map(|row| {
                User {
                    id: row.get(0),
                    name: row.get(1),
                }
            }).collect();
            Ok(Json(users))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[rocket::main]
async fn main() {
    let client = connect().await.expect("Failed to connect to database");
    rocket::build()
        .manage(client)
        .mount("/", routes![get_users])
        .launch()
        .await
        .expect("Rocket failed to launch");
}
```

Neste exemplo, criamos uma rota `GET /users` que consulta os usuários no banco de dados PostgreSQL usando `tokio-postgres` e retorna os resultados em formato JSON. A conexão com o banco de dados é estabelecida uma vez na inicialização do aplicativo e passada como um estado gerenciado do Rocket.

Lembre-se de ajustar os detalhes da conexão com o banco de dados de acordo com as configurações do seu ambiente. Além disso, este é apenas um exemplo básico; você precisará ajustar conforme necessário para atender às suas necessidades específicas de aplicativo.
