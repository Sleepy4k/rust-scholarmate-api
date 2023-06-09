# Schoolar Mates

## ID

### Pengertian

    Sebuah server menggunakan framework express dari javascript untuk handle data dari database ke applikasi lain nya

### Kebutuhan System

- Node JS >= 18.xx.x
- XAMPP >= 8.x.x

### Instalasi

Pastikan kalian sudah menginstall node js dengan versi >= 18 ke atas

    npm install

Buatlah satu database pada phpmyadmin atau semacam nya dan setting di .env lalu jalankan migration

    npx sequelize-cli db:migrate

jika sudah melakukan migration lalu kita masukan data awal

    npx sequelize-cli db:seed:all

Jalankan file server.js

    npm start

## EN

### Understanding

    A server uses the express framework of javascript to handle data from the database to other applications

### Dependencies

- Node JS >= 18.xx.x
- XAMPP >= 8.x.x

### Instalation

Make sure you have installed node js with version >= 18 and above

    npm install

Create a database on phpmyadmin or something like that and the settings on .env then run migration

    npx sequelize-cli db:migrate

if you have done the migration then we enter the initial data

    npx sequelize-cli db:seed:all

Run the main.py file

    npm start
