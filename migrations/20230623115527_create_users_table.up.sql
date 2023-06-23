-- Add up migration script here
CREATE TABLE IF NOT EXISTS users(
    id SERIAL NOT NULL,
    email character varying(255) NOT NULL,
    role character varying(255) NOT NULL,
    password character varying(255) NOT NULL,
    PRIMARY KEY(id)
);
CREATE UNIQUE INDEX email_1686980212905_index ON "users" USING btree ("email");