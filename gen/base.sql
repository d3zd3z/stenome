PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE probs (id INTEGER PRIMARY KEY,
                question TEXT UNIQUE,
                answer TEXT NOT NULL);
CREATE TABLE learning (probid INTEGER PRIMARY KEY REFERENCES probs (id),
                next REAL NOT NULL,
                interval REAL NOT NULL);
CREATE TABLE schema_version (version TEXT NOT NULL);
INSERT INTO "schema_version" VALUES('20170704A');
CREATE TABLE config (key TEXT PRIMARY KEY, value TEXT NOT NULL);
INSERT INTO "config" VALUES('kind','simple');
CREATE INDEX learning_next ON learning (next);
COMMIT;
