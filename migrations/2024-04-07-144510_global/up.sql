-- Your SQL goes here

-- Users

CREATE TABLE "access_levels" (
    "level" VARCHAR NOT NULL PRIMARY KEY
);

CREATE TABLE "users"(
	"id" SERIAL PRIMARY KEY,
	"password" VARCHAR NOT NULL,
	"access_level" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL UNIQUE,
    FOREIGN KEY ("access_level") REFERENCES "access_levels" ("level")
);

CREATE TABLE "users_metadata"(
	"id" INT4 NOT NULL,
	"username" VARCHAR NOT NULL,
	"discriminator" SERIAL NOT NULL,
	"last_check_in" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
	"picture" TEXT NOT NULL,
	"account_creation" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
	"description" TEXT NOT NULL,
    PRIMARY KEY ("username", "discriminator"),
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

CREATE TABLE "members" (
	"user_id" INT4 NOT NULL,
	"guild_id" INT4 NOT NULL,
	PRIMARY KEY ("user_id", "guild_id"),
	FOREIGN KEY ("user_id") REFERENCES "users" ("id"),
	FOREIGN KEY ("guild_id") REFERENCES "guilds" ("id")
);

CREATE TABLE "roles_category" (
	"category" VARCHAR NOT NULL PRIMARY KEY
);

CREATE TABLE "roles" (
	"id" SERIAL NOT NULL,
	"guild_id" INT4 NOT NULL,
    "name" VARCHAR NOT NULL,
	"color" VARCHAR(7) NOT NULL,
	"category" VARCHAR NOT NULL,
	PRIMARY KEY ("id"),
    FOREIGN KEY ("guild_id") REFERENCES "guilds" ("id")
);

CREATE TABLE "members_roles" (
	"role_id" INT4 NOT NULL,
	"guild_id" INT4 NOT NULL,
	"member_id" INT4 NOT NULL,
	PRIMARY KEY ("role_id", "guild_id", "member_id"),
	FOREIGN KEY ("role_id") REFERENCES "roles" ("id"),
	FOREIGN KEY ("member_id", "guild_id") REFERENCES "members" ("user_id", "guild_id")
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

CREATE TABLE "channel_permissions" (
	"role_id" INT4 NOT NULL,
	"guild_id" INT4 NOT NULL,
	"channel_id" INT4 NOT NULL,
	"can_read" BOOLEAN NOT NULL,
	"can_write" BOOLEAN NOT NULL,
	-- We could shortcut this as ("role_id", "channel_id")
	-- Since role ids are unique **FOR NOW**
	PRIMARY KEY ("role_id", "guild_id", "channel_id"),
	FOREIGN KEY ("role_id") REFERENCES "roles" ("id"),
	FOREIGN KEY ("guild_id") REFERENCES "guilds" ("id"),
	FOREIGN KEY ("channel_id") REFERENCES "channels" ("id")
);

-- Messages
CREATE TABLE "messages"(
	"id" SERIAL NOT NULL,
	"channel_id" INT4 NOT NULL,
	"author_id" INT4 NOT NULL,
	"content" VARCHAR NOT NULL,
	"creation_date" TIMESTAMPTZ NOT NULL,
	PRIMARY KEY ("id", "channel_id"),
    FOREIGN KEY ("channel_id") REFERENCES "channels" ("id"),
	-- Can't reference a member as someone can join then leave
    FOREIGN KEY ("author_id") REFERENCES "users" ("id")
);

