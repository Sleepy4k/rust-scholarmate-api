-- Add up migration script here
CREATE TABLE IF NOT EXISTS students(
    id SERIAL NOT NULL,
    first_name character varying NOT NULL,
    last_name character varying NOT NULL,
    email character varying NOT NULL,
    phone character varying NOT NULL,
    date_of_birth date NOT NULL,
    region character varying NOT NULL,
    register_number character varying(255) NOT NULL,
    toefl_score integer NOT NULL,
    ielts_score integer NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_students_users FOREIGN key(email) REFERENCES users(email) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE UNIQUE INDEX email_1686980514290_index ON "students" USING btree ("email");
CREATE UNIQUE INDEX phone_1686980521796_index ON "students" USING btree ("phone");
CREATE UNIQUE INDEX register_number_1686980530332_index ON "students" USING btree ("register_number");