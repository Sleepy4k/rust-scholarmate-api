-- Add up migration script here
CREATE TABLE IF NOT EXISTS applications(
    id SERIAL NOT NULL,
    univ_id integer NOT NULL,
    student_id integer NOT NULL,
    status character varying(255) NOT NULL,
    major character varying(255) NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_applications_universities FOREIGN key(univ_id) REFERENCES universities(id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT fk_applications_students FOREIGN key(student_id) REFERENCES students(id) ON UPDATE CASCADE ON DELETE CASCADE
);