CREATE TABLE 'users' (
  'username' TEXT NOT NULL PRIMARY KEY,
  'password' TEXT NOT NULL,
  'email' TEXT UNIQUE
);
