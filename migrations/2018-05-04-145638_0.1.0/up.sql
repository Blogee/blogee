-- Your SQL goes here

CREATE TABLE users (
	email TEXT UNIQUE PRIMARY KEY NOT NULL,
	username TEXT NOT NULL,
	password TEXT NOT NULL,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	bio TEXT NOT NULL,
	avatar TEXT NOT NULL,
	website TEXT NOT NULL,
	gpg TEXT
);

CREATE TABLE articles (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	title TEXT NOT NULL,
	body TEXT NOT NULL,
	format TEXT NOT NULL,
	user_email TEXT NOT NULL,
	created_at TIMESTAMP DEFAULT (datetime('now','localtime')) NOT NULL,
	last_modified TIMESTAMP DEFAULT (datetime('now','localtime')) NOT NULL,
	FOREIGN KEY (user_email) REFERENCES users(email) ON DELETE CASCADE
);

CREATE TABLE links (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	href TEXT NOT NULL,
	title TEXT,
	description TEXT,
	clicked_at TIMESTAMP,
	clicks INTEGER DEFAULT '0' NOT NULL,
	seen INTEGER DEFAULT '0' NOT NULL,
	article_id INTEGER NOT NULL,
	created_at TIMESTAMP DEFAULT (datetime('now','localtime')) NOT NULL,
	last_modified TIMESTAMP DEFAULT (datetime('now','localtime')) NOT NULL,
	FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
);

CREATE TABLE comments (
	article_id INTEGER NOT NULL,
	comment_no INTEGER,
	body TEXT NOT NULL,
	show_email BOOLEAN DEFAULT 'T',
	commentor_name TEXT NOT NULL,
	commentor_email TEXT NOT NULL,
	reply_on INTEGER DEFAULT '0',
	user INTEGER,
	created_at TIMESTAMP DEFAULT (datetime('now','localtime')) NOT NULL,
	last_modified TIMESTAMP DEFAULT (datetime('now','localtime')) NOT NULL,
	PRIMARY KEY (article_id, comment_no),
	FOREIGN KEY (reply_on) REFERENCES comments(id) ON DELETE CASCADE,
	FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
);

CREATE TABLE users_socials (
	user_email TEXT NOT NULL,
	social TEXT NOT NULL,
	PRIMARY KEY (user_email, social),
	FOREIGN KEY (user_email) REFERENCES users(email) ON DELETE CASCADE
);

CREATE TABLE articles_tags (
	article_id INTEGER NOT NULL,
	tag TEXT NOT NULL,
	PRIMARY KEY (article_id, tag),
	FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
);

INSERT INTO users VALUES (
"admin@blogee.rs",
"admin",
"admin",
"blogee",
"admin",
"admin for the blog",
"NONE",
"blogee.rs",
"SDD324324DSFS"
);

INSERT INTO articles(id, title, body, format, user_email) VALUES (
'1',
"Hello Blogee",
"this is amazing you have installed Blogee",
"markdown",
"admin@blogee.rs"
);
