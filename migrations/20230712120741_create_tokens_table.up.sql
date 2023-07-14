-- Add up migration script here
CREATE TABLE IF NOT EXISTS tokens(
    id SERIAL NOT NULL,
    token character varying(255) NOT NULL,
    otp character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_tokens_email FOREIGN key(email) REFERENCES users(email) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE UNIQUE INDEX token_1689150343566_index ON "tokens" USING btree ("token");
CREATE UNIQUE INDEX email_1689150364591_index ON "tokens" USING btree ("email");
CREATE UNIQUE INDEX otp_1689150360389_index ON "tokens" USING btree ("otp");