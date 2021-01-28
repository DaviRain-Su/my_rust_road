-- -- Your SQL goes here
-- CREATE TABLE IF NOT EXISTS users (
--     id CHARACTER(36) NOT NULL PRIMARY KEY,
--     email VARCHAR(60),
--     phone VARCHAR(20),
--     created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
--     updated_at DATENAME DEFAULT CURRENT_TIMESTAMP NOT NULL
-- );

-- CREATE TRIGGER IF NOT EXISTS UpdateTimestamps AFTER UPDATE NO users 
--     FOR EACH ROW WHEN NEW.updated_at <= OLD.updated_at
-- BEGIN 
--     update users set updated_at=CURRENT_TIMESTAMP where id=0LD.id;
-- END;
CREATE TABLE IF NOT EXISTS users (
  id CHARACTER(36) NOT NULL PRIMARY KEY,
  email VARCHAR(60),
  phone VARCHAR(20),
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER IF NOT EXISTS UpdateTimestamps AFTER UPDATE ON users
  FOR EACH ROW WHEN NEW.updated_at <= OLD.updated_at 
BEGIN 
  update users set updated_at=CURRENT_TIMESTAMP where id=OLD.id;  
END;
