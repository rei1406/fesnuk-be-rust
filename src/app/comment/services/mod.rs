use crate::app::comment::{
    dto::{CommentResponse, CreateCommentDto, ReactCommentDto, ReplyCommentDto},
    repositories::CommentRepository,
};
use diesel::result::Error;
use diesel::PgConnection;
use serde_json::{Map, Value};

pub struct CommentService;

impl CommentService {
    pub fn get_root_comments_by_post_id(
        conn: &mut PgConnection,
        post_id: i32,
    ) -> Result<Vec<CommentResponse>, Error> {
        let comments = CommentRepository::get_root_comments_by_post_id(conn, post_id)?;
        
        let mut comment_responses = Vec::new();
        for comment in comments {
            let reply_count = CommentRepository::count_replies_by_parent_id(conn, comment.id)?;
            comment_responses.push(CommentResponse::from_comment_with_reply_count(comment, reply_count));
        }
        
        Ok(comment_responses)
    }

    pub fn get_direct_children_by_parent_id(
        conn: &mut PgConnection,
        parent_id: i32,
    ) -> Result<Vec<CommentResponse>, Error> {
        let comments = CommentRepository::get_direct_children_by_parent_id(conn, parent_id)?;
        
        let mut comment_responses = Vec::new();
        for comment in comments {
            let reply_count = CommentRepository::count_replies_by_parent_id(conn, comment.id)?;
            comment_responses.push(CommentResponse::from_comment_with_reply_count(comment, reply_count));
        }
        
        Ok(comment_responses)
    }

    pub fn create_root_comment(
        conn: &mut PgConnection,
        create_dto: CreateCommentDto,
    ) -> Result<CommentResponse, Error> {
        let new_comment = create_dto.to_new_comment();
        let comment = CommentRepository::create_comment(conn, new_comment)?;
        let reply_count = CommentRepository::count_replies_by_parent_id(conn, comment.id)?;
        Ok(CommentResponse::from_comment_with_reply_count(comment, reply_count))
    }

    pub fn reply_to_comment(
        conn: &mut PgConnection,
        reply_dto: ReplyCommentDto,
    ) -> Result<CommentResponse, Error> {
        let new_comment = reply_dto.to_new_comment();
        let comment = CommentRepository::create_comment(conn, new_comment)?;
        let reply_count = CommentRepository::count_replies_by_parent_id(conn, comment.id)?;
        Ok(CommentResponse::from_comment_with_reply_count(comment, reply_count))
    }

    pub fn react_to_comment(
        conn: &mut PgConnection,
        react_dto: ReactCommentDto,
    ) -> Result<CommentResponse, Error> {
        let reaction = react_dto.to_comment_reaction();
        let comment = CommentRepository::get_comment_by_id(conn, reaction.comment_id)?;
        
        let mut reactions_map = match comment.reactions.as_object() {
            Some(map) => map.clone(),
            None => Map::new(),
        };

        let current_count = reactions_map
            .get(&reaction.unicode)
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        let new_count = current_count + reaction.action as i64;
        
        if new_count <= 0 {
            reactions_map.remove(&reaction.unicode);
        } else {
            reactions_map.insert(reaction.unicode, Value::Number(new_count.into()));
        }

        let updated_reactions = Value::Object(reactions_map);
        let updated_comment = CommentRepository::update_comment_reactions(
            conn,
            reaction.comment_id,
            updated_reactions,
        )?;
        
        let reply_count = CommentRepository::count_replies_by_parent_id(conn, updated_comment.id)?;
        Ok(CommentResponse::from_comment_with_reply_count(updated_comment, reply_count))
    }
}