-- Your SQL goes here

-- Users
CREATE TABLE "users"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"password" VARCHAR NOT NULL,
	"user_type" INT4 NOT NULL,
	"email" VARCHAR NOT NULL
);

CREATE TABLE "users_metadata"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"discriminator" INT2 NOT NULL,
	"last_check_in" TIMESTAMPTZ NOT NULL,
	"picture" TEXT NOT NULL,
	"account_creation" TIMESTAMPTZ NOT NULL,
	"description" TEXT NOT NULL,
    FOREIGN KEY ("id") REFERENCES "users" ("id")
);

-- Guilds
CREATE TABLE "guilds"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"owner_id" INT4 NOT NULL,
    FOREIGN KEY ("owner_id") REFERENCES "users" ("id")
);

CREATE TABLE "roles"(
	"id" INT4 NOT NULL,
	"guild_id" INT4 NOT NULL,
    "name" VARCHAR NOT NULL,
	PRIMARY KEY("id", "guild_id"),
    FOREIGN KEY ("guild_id") REFERENCES "guilds" ("id")
);

-- Channels
CREATE TABLE "channel_kinds" (
    "kind" VARCHAR NOT NULL PRIMARY KEY
);

CREATE TABLE "channels"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"guild_id" INT4 NOT NULL,
	"name" VARCHAR NOT NULL,
	"kind" VARCHAR NOT NULL,
	FOREIGN KEY ("guild_id") REFERENCES "guilds" ("id"),
    FOREIGN KEY ("kind") REFERENCES "channel_kinds" ("kind")
);

-- Messages
CREATE TABLE "messages"(
	"id" INT4 NOT NULL,
	"channel_id" INT4 NOT NULL,
	"author_id" INT4 NOT NULL,
	"content" VARCHAR NOT NULL,
	"creation_date" TIMESTAMPTZ NOT NULL,
	PRIMARY KEY("id", "channel_id"),
    FOREIGN KEY ("channel_id") REFERENCES "channels" ("id"),
    FOREIGN KEY ("author_id") REFERENCES "users" ("id")
);

