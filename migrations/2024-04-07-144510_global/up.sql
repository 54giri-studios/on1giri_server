-- Your SQL goes here

-- Users

CREATE TABLE "access_levels" (
    "level" VARCHAR NOT NULL PRIMARY KEY
);

CREATE TABLE "users"(
	"id" SERIAL PRIMARY KEY,
	"password" VARCHAR NOT NULL,
	"access_level" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
    FOREIGN KEY ("access_level") REFERENCES "access_levels" ("level")
);

CREATE TABLE "users_metadata"(
	"id" SERIAL PRIMARY KEY,
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
	"id" SERIAL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"owner_id" INT4 NOT NULL,
	"description" TEXT NOT NULL,
	"creation_date" TIMESTAMPTZ NOT NULL,
    FOREIGN KEY ("owner_id") REFERENCES "users" ("id")
);

CREATE TABLE "roles"(
	"id" SERIAL NOT NULL,
	"guild_id" INT4 NOT NULL,
    "name" VARCHAR NOT NULL,
	PRIMARY KEY("id", "guild_id"),
    FOREIGN KEY ("guild_id") REFERENCES "guilds" ("id")
);

CREATE TABLE "members" (
	"user_id" INT4 NOT NULL,
	"guild_id" INT4 NOT NULL,
	PRIMARY KEY ("user_id", "guild_id"),
	FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
	FOREIGN KEY ("guild_id") REFERENCES "guilds" ("id")
);

-- Channels
CREATE TABLE "channel_kinds" (
    "kind" VARCHAR NOT NULL PRIMARY KEY
);

CREATE TABLE "channels"(
	"id" SERIAL PRIMARY KEY,
	"guild_id" INT4 NOT NULL,
	"name" VARCHAR NOT NULL,
	"kind" VARCHAR NOT NULL,
	FOREIGN KEY ("guild_id") REFERENCES "guilds" ("id"),
	FOREIGN KEY ("kind") REFERENCES "channel_kinds" ("kind")
);

-- Messages
CREATE TABLE "messages"(
	"id" SERIAL NOT NULL,
	"channel_id" INT4 NOT NULL,
	"author_id" INT4 NOT NULL,
	"content" VARCHAR NOT NULL,
	"creation_date" TIMESTAMPTZ NOT NULL,
	PRIMARY KEY("id", "channel_id"),
    FOREIGN KEY ("channel_id") REFERENCES "channels" ("id"),
	-- Can't reference a member as someone can join then leave
    FOREIGN KEY ("author_id") REFERENCES "users" ("id")
);

