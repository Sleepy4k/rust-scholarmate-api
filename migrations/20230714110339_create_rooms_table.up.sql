-- Add up migration script here
CREATE TABLE IF NOT EXISTS rooms(
    id SERIAL NOT NULL,
    name character varying(255) NOT NULL,
    members text NOT NULL,
    updated_at date NOT NULL DEFAULT CURRENT_DATE,
    created_at date NOT NULL DEFAULT CURRENT_DATE,
    PRIMARY KEY(id)
);