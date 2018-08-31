-- Your SQL goes here

CREATE TABLE user (
	email TEXT UNIQUE PRIMARY KEY NOT NULL,
	username TEXT NOT NULL,
	password TEXT NOT NULL,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	bio TEXT NOT NULL,
	avatar TEXT NOT NULL,
	website TEXT NOT NULL,
	gpg TEXT NOT NULL
);

CREATE TABLE article (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	title TEXT NOT NULL,
	body TEXT NOT NULL,
	format TEXT NOT NULL,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	last_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	user_email TEXT NOT NULL,
	FOREIGN KEY (user_email) REFERENCES user(email) ON DELETE CASCADE
);

CREATE TABLE link (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	href TEXT NOT NULL,
	title TEXT NOT NULL DEFAULT '',
	description TEXT DEFAULT '' NOT NULL,
	clicked_at TIMESTAMP,
	clicks INTEGER DEFAULT '0' NOT NULL,
	seen INTEGER DEFAULT '0' NOT NULL,
	article_id INTEGER NOT NULL,
	FOREIGN KEY (article_id) REFERENCES article(id) ON DELETE CASCADE
);

CREATE TABLE comment (
	article_id INTEGER NOT NULL,
	comment_no INTEGER,
	body TEXT NOT NULL,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	last_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	show_email BOOLEAN DEFAULT 'T',
	commentor_name TEXT NOT NULL,
	commentor_email TEXT NOT NULL,
	reply_on INTEGER DEFAULT '0',
	user INTEGER,
	PRIMARY KEY (article_id, comment_no),
	FOREIGN KEY (reply_on) REFERENCES comment(id) ON DELETE CASCADE,
	FOREIGN KEY (article_id) REFERENCES article(id) ON DELETE CASCADE
);

CREATE TABLE user_socials (
	email TEXT NOT NULL,
	social TEXT NOT NULL,
	PRIMARY KEY (email, social),
	FOREIGN KEY (email) REFERENCES user(email) ON DELETE CASCADE
);

CREATE TABLE article_tags (
	article_id INTEGER NOT NULL,
	tag TEXT NOT NULL,
	PRIMARY KEY (article_id, tag),
	FOREIGN KEY (article_id) REFERENCES article(id) ON DELETE CASCADE
);



INSERT INTO user VALUES (
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

INSERT INTO article VALUES (
'1',
"Hello Blogee",
"this is amazing you have installed Blogee",
"markdown",
NULL,
NULL,
"admin@blogee.rs"
);
