-- Create comments table
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL REFERENCES posts(id),
    parent_id INTEGER REFERENCES comments(id),
    content TEXT NOT NULL,
    attachments JSONB NOT NULL DEFAULT '[]',
    reactions JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Create indexes for optimizing different query patterns
-- Index for finding all comments in a post
CREATE INDEX comments_post_id_idx ON comments USING BTREE (post_id);

-- Index for finding child comments of a parent
CREATE INDEX comments_parent_id_idx ON comments USING BTREE (parent_id);

-- Composite index for common queries filtering by post_id and parent_id
CREATE INDEX comments_post_parent_idx ON comments USING BTREE (post_id, parent_id);

-- Index for ordering comments by creation time within a post
CREATE INDEX comments_post_created_idx ON comments USING BTREE (post_id, created_at);

-- Trigger for updating updated_at timestamp
CREATE TRIGGER update_comments_updated_at
    BEFORE UPDATE ON comments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();