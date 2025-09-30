-- This file should undo anything in `up.sql`
DROP TABLE posts;
DROP TRIGGER update_posts_updated_at ON posts;