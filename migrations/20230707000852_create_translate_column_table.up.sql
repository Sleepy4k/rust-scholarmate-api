-- Add up migration script here
CREATE TABLE IF NOT EXISTS translate_columns(
    id SERIAL NOT NULL,
    table_name character varying(255) NOT NULL,
    column_name character varying(255) NOT NULL,
    language character varying(255) NOT NULL,
    PRIMARY KEY(id)
);