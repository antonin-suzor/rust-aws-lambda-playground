CREATE TYPE permission_level_enum AS ENUM ('user', 'admin');

CREATE TABLE IF NOT EXISTS "accounts"
(
    "id" SERIAL PRIMARY KEY,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "deleted_at" TIMESTAMPTZ,
	"email" TEXT NOT NULL,
	"permission_level" permission_level_enum NOT NULL DEFAULT 'user'
);
