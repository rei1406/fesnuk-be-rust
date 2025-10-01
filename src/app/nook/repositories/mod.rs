use super::models::{NewNook, Nook, NookChanges};
use crate::schema::nooks;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct NookRepository;

impl NookRepository {
    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Nook>> {
        nooks::table
            .filter(nooks::deleted_at.is_null())
            .select(Nook::as_select())
            .load(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, nook_id: &str) -> QueryResult<Nook> {
        nooks::table
            .filter(nooks::id.eq(nook_id))
            .filter(nooks::deleted_at.is_null())
            .select(Nook::as_select())
            .first(conn)
    }

    pub fn find_by_id_with_deleted(
        conn: &mut PgConnection,
        nook_id: &str,
    ) -> QueryResult<Option<Nook>> {
        nooks::table
            .filter(nooks::id.eq(nook_id))
            .select(Nook::as_select())
            .first(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_nook: NewNook) -> QueryResult<Nook> {
        // Check if there's a soft-deleted nook with the same ID
        match Self::find_by_id_with_deleted(conn, &new_nook.id)? {
            Some(existing_nook) if existing_nook.deleted_at.is_some() => {
                // If exists and is soft-deleted, hard delete it first
                diesel::delete(nooks::table)
                    .filter(nooks::id.eq(&new_nook.id))
                    .execute(conn)?;
            }
            Some(_) => {
                // If exists and not soft-deleted, return error
                return Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    Box::new("Nook with this ID already exists".to_string()),
                ));
            }
            None => {}
        }

        // Create new nook
        diesel::insert_into(nooks::table)
            .values(&new_nook)
            .returning(Nook::as_returning())
            .get_result(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        nook_id: &str,
        update_nook: NookChanges,
    ) -> QueryResult<Nook> {
        // If updating ID, check for conflicts
        if let Some(new_id) = &update_nook.id {
            if new_id != nook_id {
                match Self::find_by_id_with_deleted(conn, new_id)? {
                    Some(existing_nook) if existing_nook.deleted_at.is_some() => {
                        // If target ID exists but is soft-deleted, remove it
                        diesel::delete(nooks::table)
                            .filter(nooks::id.eq(new_id))
                            .execute(conn)?;
                    }
                    Some(_) => {
                        // If target ID exists and is not soft-deleted, return error
                        return Err(diesel::result::Error::DatabaseError(
                            diesel::result::DatabaseErrorKind::UniqueViolation,
                            Box::new("Nook with target ID already exists".to_string()),
                        ));
                    }
                    None => {}
                }
            }
        }

        diesel::update(nooks::table)
            .filter(nooks::id.eq(nook_id))
            .filter(nooks::deleted_at.is_null())
            .set(update_nook)
            .returning(Nook::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, nook_id: &str) -> QueryResult<Nook> {
        diesel::update(nooks::table)
            .filter(nooks::id.eq(nook_id))
            .filter(nooks::deleted_at.is_null())
            .set(nooks::deleted_at.eq(diesel::dsl::now))
            .returning(Nook::as_returning())
            .get_result(conn)
    }

    pub fn hard_delete(conn: &mut PgConnection, nook_id: &str) -> QueryResult<bool> {
        let deleted_count = diesel::delete(nooks::table)
            .filter(nooks::id.eq(nook_id))
            .execute(conn)?;

        Ok(deleted_count > 0)
    }
}
