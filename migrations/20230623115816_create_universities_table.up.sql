-- Add up migration script here
CREATE TABLE IF NOT EXISTS universities(
    id SERIAL NOT NULL,
    name character varying(255) NOT NULL,
    description character varying(255) NOT NULL,
    major character varying(255) NOT NULL,
    quantity integer NOT NULL,
    image character varying(255) NOT NULL DEFAULT './src/assets/logo.png'::character varying,
    link character varying(255) NOT NULL DEFAULT 'https://artkana30.github.io'::character varying,
    alias character varying(255) NOT NULL,
    PRIMARY KEY(id)
);