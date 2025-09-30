-- Enable ltree extension
CREATE EXTENSION IF NOT EXISTS ltree;

-- Your SQL goes here
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL REFERENCES posts(id),
    path ltree NOT NULL,
    content TEXT NOT NULL,
    attachments JSONB NOT NULL DEFAULT '[]',
    reactions JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Create indexes for optimizing different query patterns
-- GiST index for hierarchical operations (ancestor/descendant queries)
CREATE INDEX comments_path_gist_idx ON comments USING GIST (path);

-- B-tree index for exact path matches and sorting
CREATE INDEX comments_path_btree_idx ON comments USING BTREE (path);

-- Composite index for common queries filtering by post_id and path
CREATE INDEX comments_post_path_idx ON comments USING BTREE (post_id, path);

-- Create trigger for updating path when inserting new comments
CREATE OR REPLACE FUNCTION update_comment_path() RETURNS TRIGGER AS $$
DECLARE
    parent_path ltree;
BEGIN
    IF subltree(NEW.path, 0, nlevel(NEW.path) - 1) = ''::ltree THEN
        -- Root comment: path is just its own ID
        NEW.path = NEW.id::text::ltree;
    ELSE
        -- Get the parent path (all but the last level)
        parent_path = subltree(NEW.path, 0, nlevel(NEW.path) - 1);
        
        -- Verify parent exists and belongs to the same post
        IF NOT EXISTS (
            SELECT 1 
            FROM comments 
            WHERE path = parent_path 
            AND post_id = NEW.post_id
        ) THEN
            RAISE EXCEPTION 'Parent comment does not exist or belongs to different post';
        END IF;
        
        -- Set path as parent path concatenated with new comment id
        NEW.path = parent_path || NEW.id::text;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_comment_path
    BEFORE INSERT ON comments
    FOR EACH ROW
    EXECUTE FUNCTION update_comment_path();

-- Trigger for updating updated_at timestamp
CREATE TRIGGER update_comments_updated_at
    BEFORE UPDATE ON comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
