
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

Migrate database (make sure you already setting up .env file)

~~~bash  
  sqlx migrate run
~~~

Start the server

~~~bash  
  cargo run
~~~

Start the server (for production)

~~~bash  
  cargo run --release --bin scholarmate_api
~~~

## Environment Variables  

To run this project, you will need to add the following environment variables to your .env file  

`DATABASE_URL`  

`JWT_TOKEN_TITLE` 

`JWT_TOKEN_SECRET` 

## Acknowledgements  

- [Rust Lang](https://doc.rust-lang.org/book/)
- [Sqlx CLI](https://lib.rs/crates/sqlx-cli)

## Feedback  

If you have any feedback, please reach out to us at sarahpalastrin@gmail.com

## License  

[MIT](https://github.com/Sleepy4k/rust-scholarmate-api/blob/main/LICENSE)
