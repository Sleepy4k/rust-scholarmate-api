-- Add up migration script here
CREATE TABLE IF NOT EXISTS chats(
    id SERIAL NOT NULL,
    room_id integer NOT NULL,
    user_id integer NOT NULL,
    message text NOT NULL,
    created_at date NOT NULL DEFAULT CURRENT_DATE,
    PRIMARY KEY(id),
    CONSTRAINT fk_chats_room_id FOREIGN key(room_id) REFERENCES rooms(id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT fk_chats_user_id FOREIGN key(user_id) REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE
);