-- Add up migration script here

CREATE TABLE IF NOT EXISTS "todos"
(
    "id" SERIAL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "done" BOOLEAN NOT NULL DEFAULT FALSE
);
