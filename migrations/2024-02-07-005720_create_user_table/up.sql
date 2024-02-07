-- Your SQL goes here
CREATE TABLE "user"(
	"id" UUID NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
	"password_hash" VARCHAR NOT NULL
);

