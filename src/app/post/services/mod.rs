use crate::app::post::{
    dto::{CreatePostDto, PostResponse, ReactPostDto},
    repositories::PostRepository,
};
use diesel::pg::PgConnection;

pub struct PostService;

impl PostService {
    pub fn get_all_posts(
        conn: &mut PgConnection,
    ) -> Result<Vec<PostResponse>, diesel::result::Error> {
        let posts = PostRepository::find_all(conn)?;
        Ok(posts.into_iter().map(PostResponse::from).collect())
    }

    pub fn get_posts_by_nook_id(
        conn: &mut PgConnection,
        nook_id: &str,
    ) -> Result<Vec<PostResponse>, diesel::result::Error> {
        let posts = PostRepository::find_by_nook_id(conn, nook_id)?;
        Ok(posts.into_iter().map(PostResponse::from).collect())
    }

    pub fn get_post_by_id(
        conn: &mut PgConnection,
        id: i32,
    ) -> Result<PostResponse, diesel::result::Error> {
        let post = PostRepository::find_by_id(conn, id)?;
        Ok(PostResponse::from(post))
    }

    pub fn create_post(
        conn: &mut PgConnection,
        dto: CreatePostDto,
    ) -> Result<PostResponse, diesel::result::Error> {
        let post = PostRepository::create(conn, dto.to_new_post())?;
        Ok(PostResponse::from(post))
    }

    pub fn react(
        conn: &mut PgConnection,
        dto: ReactPostDto,
    ) -> Result<PostResponse, diesel::result::Error> {
        let post = PostRepository::react(conn, dto.to_reaction())?;
        Ok(PostResponse::from(post))
    }
}