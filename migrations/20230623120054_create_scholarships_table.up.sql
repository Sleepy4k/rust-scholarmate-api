-- Add up migration script here
CREATE TABLE IF NOT EXISTS scholarships(
    id SERIAL NOT NULL,
    name character varying NOT NULL,
    quantity integer NOT NULL,
    description character varying NOT NULL,
    requirement character varying NOT NULL,
    univ_id integer NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_schoolarships_universities FOREIGN key(univ_id) REFERENCES universities(id)
);