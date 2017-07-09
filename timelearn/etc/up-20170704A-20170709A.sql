-- Upgrade from 20170704A to 20170709A.
BEGIN;
CREATE TABLE log (stamp REAL NOT NULL,
	score INTEGER NOT NULL,
	probid INTEGER REFERENCES probs (id) NOT NULL);
UPDATE schema_version SET version = '20170709A' WHERE version = '20170704A';
COMMIT;
