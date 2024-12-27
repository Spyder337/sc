-- Your SQL goes here
CREATE TABLE IF NOT EXISTS searches (
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    -- Search query parameter
    query TEXT NOT NULL,
    -- Site to limit the search to
    website TEXT,
    -- Required search term
    allintext TEXT,
    -- Time the search was created
    time_stamp DATE NOT NULL
);
CREATE TABLE IF NOT EXISTS quotes (
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    -- Quote text
    quote TEXT NOT NULL,
    -- Author of the quote
    author TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS tasks(
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    -- Task name
    task TEXT NOT NULL,
    -- Optional description of the task
    desc TEXT,
    -- in progress, completed, incompleted
    status TEXT NOT NULL,
    -- time the task was created
    time_stamp DATE NOT NULL,
    -- time the task is due
    due_date DATE,
    -- renewal duration
    renewal_duration INT
);
CREATE TABLE IF NOT EXISTS task_relations(
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    -- Parent task
    parent_id INT NOT NULL,
    -- Child task
    child_id INT NOT NULL,
    FOREIGN KEY (parent_id) REFERENCES tasks(id),
    FOREIGN KEY (child_id) REFERENCES tasks(id)
);