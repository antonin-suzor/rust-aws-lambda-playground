-- Add up migration script here

CREATE TABLE IF NOT EXISTS "maintainer_log"
(
    "id" SERIAL PRIMARY KEY,
    "message" TEXT NOT NULL,
    "from" TEXT,
    "created_at" TIMESTAMP DEFAULT NOW()
);
