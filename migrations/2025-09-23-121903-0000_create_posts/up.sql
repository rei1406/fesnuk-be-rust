-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    -- content of the post, markdown format
    content TEXT NOT NULL,
    -- array for attachments
    -- example : [{"content": "https://example.com/image.jpg", "type": "image", "format": "jpg", "file_name": "image.jpg"}]
	attachments JSONB NOT NULL DEFAULT '[]',
    -- Reactions contains emoji and count, emoji being represented by unicode. Count 0 should be ignored, but still stored.
    -- example : {"U+1F44D": 10, "U+1F602": 5, "U+1F601": 0}
    reactions JSONB NOT NULL DEFAULT '{}',
    nook_id VARCHAR NOT NULL REFERENCES nooks(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TRIGGER update_posts_updated_at
    BEFORE UPDATE ON posts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
