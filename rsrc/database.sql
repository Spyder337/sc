CREATE TABLE
  IF NOT EXISTS ShellCommanders (
    ID INTEGER PRIMARY KEY AUTOINCREMENT,
    VAR TEXT NOT NULL,
    VAL TEXT,
    UNIQUE (VAR)
  );

-- Change the default values to your own.
INSERT
OR IGNORE INTO ShellCommanders (VAR, VAL)
VALUES
  ('GIT_DIR', '~/Code'),
  ('GIT_AUTHOR', 'Author'),
  ('GIT_EMAIL', 'email.address@site.dom'),
  ('GIT_IGNORE_URL', 'https://www.toptal.com/developers/gitignore');

COMMIT;