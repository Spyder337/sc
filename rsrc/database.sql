CREATE TABLE
  IF NOT EXISTS ShellCommanders (
    ID INTEGER PRIMARY KEY AUTOINCREMENT,
    VAR TEXT NOT NULL,
    VAL TEXT
  );

INSERT
OR IGNORE INTO ShellCommanders (VAR, VAL)
VALUES
  ('GIT_DIR', '~/Code'),
  ('GIT_AUTHOR', 'Author'),
  ('GIT_EMAIL', 'email.address@site.dom');

-- -- Create or Update
-- -- SQLite does not support CREATE PROCEDURE, so we use a direct SQL statement
-- -- to handle the insert or update logic
-- INSERT INTO
--   ShellCommanders (VAR, VAL)
-- VALUES
--   ('var', 'val') ON CONFLICT (VAR) DO
-- UPDATE
-- SET
--   VAL = excluded.VAL;
-- -- Read
-- CREATE PROCEDURE GetShellCommanderByID (IN id INTEGER) BEGIN
-- SELECT
--   *
-- FROM
--   ShellCommanders
-- WHERE
--   ID = id;
-- END;
-- CREATE PROCEDURE GetAllShellCommanders () BEGIN
-- SELECT
--   *
-- FROM
--   ShellCommanders;
-- END;
-- -- Update
-- CREATE PROCEDURE UpdateShellCommander (IN id INTEGER, IN var TEXT, IN val TEXT) BEGIN
-- UPDATE ShellCommanders
-- SET
--   VAR = var,
--   VAL = val
-- WHERE
--   ID = id;
-- END;
-- -- Delete
-- CREATE PROCEDURE DeleteShellCommander (IN id INTEGER) BEGIN
-- DELETE FROM ShellCommanders
-- WHERE
--   ID = id;
-- END;
-- -- Check if exists
-- CREATE PROCEDURE ShellCommanderExists (IN id INTEGER) BEGIN
-- SELECT
--   EXISTS (
--     SELECT
--       1
--     FROM
--       ShellCommanders
--     WHERE
--       ID = id
--   ) AS Exists;
-- END;
-- -- Check if exists by VAR
-- CREATE PROCEDURE ShellCommanderExistsByVar (IN var TEXT) BEGIN
-- SELECT
--   EXISTS (
--     SELECT
--       1
--     FROM
--       ShellCommanders
--     WHERE
--       VAR = var
--   ) AS Exists;
-- END;
-- -- Check if exists by VAL
-- CREATE PROCEDURE ShellCommanderExistsByVal (IN val TEXT) BEGIN
-- SELECT
--   EXISTS (
--     SELECT
--       1
--     FROM
--       ShellCommanders
--     WHERE
--       VAL = val
--   ) AS Exists;
-- END;