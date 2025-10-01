use crate::app::nook::{
    dto::{CreateNookDto, NookResponse, UpdateNookDto},
    repositories::NookRepository,
};
use diesel::pg::PgConnection;

pub struct NookService;

impl NookService {
    pub fn get_all_nooks(
        conn: &mut PgConnection,
    ) -> Result<Vec<NookResponse>, diesel::result::Error> {
        let nooks = NookRepository::find_all(conn)?;
        Ok(nooks.into_iter().map(NookResponse::from).collect())
    }

    pub fn get_nook_by_id(
        conn: &mut PgConnection,
        id: &str,
    ) -> Result<NookResponse, diesel::result::Error> {
        let nook = NookRepository::find_by_id(conn, id)?;
        Ok(NookResponse::from(nook))
    }

    pub fn create_nook(
        conn: &mut PgConnection,
        dto: CreateNookDto,
    ) -> Result<NookResponse, diesel::result::Error> {
        let nook = NookRepository::create(conn, dto.to_new_nook())?;
        Ok(NookResponse::from(nook))
    }

    pub fn update_nook(
        conn: &mut PgConnection,
        id: &str,
        dto: UpdateNookDto,
    ) -> Result<NookResponse, diesel::result::Error> {
        let nook = NookRepository::update(conn, id, dto.to_nook_changes())?;
        Ok(NookResponse::from(nook))
    }

    pub fn delete_nook(
        conn: &mut PgConnection,
        id: &str,
    ) -> Result<NookResponse, diesel::result::Error> {
        let nook = NookRepository::delete(conn, id)?;
        Ok(NookResponse::from(nook))
    }
}
