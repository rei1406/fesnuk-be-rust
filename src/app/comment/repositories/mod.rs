use crate::app::comment::models::{Comment, NewComment};
use crate::schema::comments;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

pub struct CommentRepository;

impl CommentRepository {
    pub fn get_root_comments_by_post_id(
        conn: &mut PgConnection,
        post_id: i32,
    ) -> Result<Vec<Comment>, Error> {
        comments::table
            .filter(comments::post_id.eq(post_id))
            .filter(comments::parent_id.is_null())
            .filter(comments::deleted_at.is_null())
            .order(comments::created_at.asc())
            .load::<Comment>(conn)
    }

    pub fn get_direct_children_by_parent_id(
        conn: &mut PgConnection,
        parent_id: i32,
    ) -> Result<Vec<Comment>, Error> {
        comments::table
            .filter(comments::parent_id.eq(parent_id))
            .filter(comments::deleted_at.is_null())
            .order(comments::created_at.asc())
            .load::<Comment>(conn)
    }

    pub fn create_comment(
        conn: &mut PgConnection,
        new_comment: NewComment,
    ) -> Result<Comment, Error> {
        diesel::insert_into(comments::table)
            .values(&new_comment)
            .get_result(conn)
    }

    pub fn get_comment_by_id(
        conn: &mut PgConnection,
        comment_id: i32,
    ) -> Result<Comment, Error> {
        comments::table
            .filter(comments::id.eq(comment_id))
            .filter(comments::deleted_at.is_null())
            .first(conn)
    }

    pub fn update_comment_reactions(
        conn: &mut PgConnection,
        comment_id: i32,
        reactions: serde_json::Value,
    ) -> Result<Comment, Error> {
        diesel::update(comments::table.filter(comments::id.eq(comment_id)))
            .set(comments::reactions.eq(reactions))
            .get_result(conn)
    }

    pub fn count_replies_by_parent_id(
        conn: &mut PgConnection,
        parent_id: i32,
    ) -> Result<i64, Error> {
        comments::table
            .filter(comments::parent_id.eq(parent_id))
            .filter(comments::deleted_at.is_null())
            .count()
            .get_result(conn)
    }
}