-- Your SQL goes here
CREATE TABLE nooks (
  id VARCHAR PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  image VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP
);

CREATE TRIGGER update_nooks_updated_at
    BEFORE UPDATE ON nooks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();