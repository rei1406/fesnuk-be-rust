use super::models::{NewPost, Post, Reaction};
use crate::schema::posts;
use diesel::dsl::sql;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::Jsonb;

pub struct PostRepository;

impl PostRepository {
    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Post>> {
        posts::table
            .filter(posts::deleted_at.is_null())
            .select(Post::as_select())
            .load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, post_id: i32) -> QueryResult<Post> {
        posts::table
            .filter(posts::id.eq(post_id))
            .filter(posts::deleted_at.is_null())
            .select(Post::as_select())
            .first(conn)
    }

    pub fn find_by_nook_id(conn: &mut PgConnection, nook_id: &str) -> QueryResult<Vec<Post>> {
        posts::table
            .filter(posts::nook_id.eq(nook_id))
            .filter(posts::deleted_at.is_null())
            .select(Post::as_select())
            .load(conn)
    }

    pub fn find_by_id_with_deleted(
        conn: &mut PgConnection,
        post_id: i32,
    ) -> QueryResult<Option<Post>> {
        posts::table
            .filter(posts::id.eq(post_id))
            .select(Post::as_select())
            .first(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_post: NewPost) -> QueryResult<Post> {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(conn)
    }

    pub fn react(conn: &mut PgConnection, reaction: Reaction) -> QueryResult<Post> {
        diesel::update(posts::table)
            .filter(posts::id.eq(reaction.post_id))
            .filter(posts::deleted_at.is_null())
            .set(posts::reactions.eq(sql::<Jsonb>(&format!(
                "jsonb_set(
                        COALESCE(reactions, '{{}}'),
                        ARRAY['{}'],
                        to_jsonb(GREATEST(COALESCE((reactions->>'{}')::int, 0) + {}, 0))
                    )",
                reaction.unicode, reaction.unicode, reaction.action
            ))))
            .get_result(conn)
    }
}
