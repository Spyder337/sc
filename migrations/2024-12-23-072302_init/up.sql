-- Your SQL goes here
CREATE TABLE IF NOT EXISTS searches (
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    query TEXT NOT NULL,
    website TEXT,
    allintext TEXT,
    time_stamp DATE NOT NULL
);
CREATE TABLE IF NOT EXISTS quotes (
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    quote TEXT NOT NULL,
    author TEXT NOT NULL
);