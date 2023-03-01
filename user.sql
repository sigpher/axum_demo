CREATE TABLE IF NOT EXISTS users(
    `id` integer  primary key AUTOINCREMENT,
    `username` text NOT NULL UNIQUE,
    `email` text NOT NULL UNIQUE,
    `dept` text NOT NULL,
    `password` text NOT NULL
);