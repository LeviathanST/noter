CREATE TABLE IF NOT EXISTS todo (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name VARCHAR(2048) NOT NULL,
	description VARCHAR(2048) NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
) 
