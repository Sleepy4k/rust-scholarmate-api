# Scholarmate

an API service made for the needs of scholarship registration for students to ease the burden of registering for lectures

## Screenshots  

![App Screenshot](https://cdn.discordapp.com/attachments/881508824576565259/1121786577354489937/WhatsApp_Image_2023-06-23_at_19.58.46.jpg)

## Tech Stack  

**Server:** Rust, Postgresql, Actix Web

## Features  

- Authentication system
- Middleware
- Database model
- Database schema
- Database migration
- Unit test

## Lessons Learned  

When i make this service, i feel like this project has good performance than the other like laravel and express.
After some update i learn something that i think i don't get it before, like i get better when search problem error,
implement new system and server security.

## Run Locally  

Clone the project  

~~~bash  
  git clone https://github.com/Sleepy4k/rust-scholarmate-api
~~~

Go to the project directory  

~~~bash  
  cd rust-scholarmate-api
~~~

Install dependencies  (if you already install sqlx-cli than skip it)

~~~bash  
  cargo install sqlx-cli
~~~

Copy .env file

~~~bash  
  copy .env.example .env
~~~

Create database (if you already create one and make sure you already setting up .env file)

~~~bash  
  sqlx database drop
~~~

Create database (make sure you already setting up .env file)

~~~bash  
  sqlx database create
~~~

Migrate database (make sure you already setting up .env file)

~~~bash  
  sqlx migrate run
~~~

Build the server

~~~bash  
  cargo build
~~~

Start the server

~~~bash  
  cargo run
~~~

Start the server (for production)

~~~bash  
  cargo run --release
~~~

Enabling auth service

~~~bash  
  cargo run --bin auth_service
~~~

Enabling export data

~~~bash  
  cargo run --bin export_data
~~~

Enabling websocket service

~~~bash  
  cargo run --bin ws_service
~~~

Want to run all service and run for production?
build all service and run it in release mode (run it from target/release)

~~~bash  
  cargo build --release
  cargo build --release --bin export_data
  cargo build --release --bin auth_service
  cargo build --release --bin ws_service
~~~

## Environment Variables  

To run this project, you will need to add the following environment variables to your .env file  

`DATABASE_URL`  

`JWT_TOKEN_TITLE`

`JWT_TOKEN_SECRET`

## Acknowledgements  

- [Rust Lang](https://doc.rust-lang.org/book)
- [Actix Web](https://docs.rs/actix-web/4.2.1/actix_web)
- [Sqlx](https://docs.rs/sqlx/0.6.3/sqlx)
- [Sqlx CLI](https://lib.rs/crates/sqlx-cli)
- [Unit Test](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#tests)
- [Example](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples)
- [Validator](https://docs.rs/validator/latest/validator)
- [CSV Writer](https://docs.rs/csv/latest/csv/tutorial/index.html)
- [XLSX Writer](https://docs.rs/xlsxwriter/latest/xlsxwriter)
- [Argon2](https://docs.rs/rust-argon2/1.0.0/argon2)
- [Maud](https://maud.lambda.xyz/getting-started.html)
- [Tokenizer](https://docs.rs/tokenizers/0.13.3/tokenizers)
- [Lettre](https://docs.rs/lettre/0.10.4/lettre)

## Feedback  

If you have any feedback, please reach out to us at <sarahpalastrin@gmail.com>

## License  

[MIT](https://github.com/Sleepy4k/rust-scholarmate-api/blob/main/LICENSE)
