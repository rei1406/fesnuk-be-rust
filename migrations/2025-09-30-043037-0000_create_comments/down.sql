-- This file should undo anything in `up.sql`
DROP TABLE comments;
DROP TRIGGER update_comments_updated_at ON comments;
DROP INDEX IF EXISTS comments_post_parent_idx;
DROP INDEX IF EXISTS comments_post_created_idx;
DROP INDEX IF EXISTS comments_parent_id_idx;
DROP INDEX IF EXISTS comments_post_id_idx;