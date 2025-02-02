use super::{
    NewPublication, NewPublicationHistory, PatchPublication, Publication, PublicationField,
    PublicationHistory, PublicationOrderBy, PublicationType,
};
use crate::graphql::utils::Direction;
use crate::model::{Crud, DbInsert, HistoryEntry};
use crate::schema::{publication, publication_history};
use crate::{crud_methods, db_insert};
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use thoth_errors::{ThothError, ThothResult};
use uuid::Uuid;

impl Crud for Publication {
    type NewEntity = NewPublication;
    type PatchEntity = PatchPublication;
    type OrderByEntity = PublicationOrderBy;
    type FilterParameter1 = PublicationType;
    type FilterParameter2 = ();

    fn pk(&self) -> Uuid {
        self.publication_id
    }

    fn all(
        db: &crate::db::PgPool,
        limit: i32,
        offset: i32,
        filter: Option<String>,
        order: Self::OrderByEntity,
        publishers: Vec<Uuid>,
        parent_id_1: Option<Uuid>,
        _: Option<Uuid>,
        publication_type: Option<Self::FilterParameter1>,
        _: Option<Self::FilterParameter2>,
    ) -> ThothResult<Vec<Publication>> {
        use crate::schema::publication::dsl;
        let connection = db.get().unwrap();
        let mut query = dsl::publication
            .inner_join(crate::schema::work::table.inner_join(crate::schema::imprint::table))
            .select((
                dsl::publication_id,
                dsl::publication_type,
                dsl::work_id,
                dsl::isbn,
                dsl::created_at,
                dsl::updated_at,
            ))
            .into_boxed();

        match order.field {
            PublicationField::PublicationId => match order.direction {
                Direction::Asc => query = query.order(dsl::publication_id.asc()),
                Direction::Desc => query = query.order(dsl::publication_id.desc()),
            },
            PublicationField::PublicationType => match order.direction {
                Direction::Asc => query = query.order(dsl::publication_type.asc()),
                Direction::Desc => query = query.order(dsl::publication_type.desc()),
            },
            PublicationField::WorkId => match order.direction {
                Direction::Asc => query = query.order(dsl::work_id.asc()),
                Direction::Desc => query = query.order(dsl::work_id.desc()),
            },
            PublicationField::Isbn => match order.direction {
                Direction::Asc => query = query.order(dsl::isbn.asc()),
                Direction::Desc => query = query.order(dsl::isbn.desc()),
            },
            PublicationField::CreatedAt => match order.direction {
                Direction::Asc => query = query.order(dsl::created_at.asc()),
                Direction::Desc => query = query.order(dsl::created_at.desc()),
            },
            PublicationField::UpdatedAt => match order.direction {
                Direction::Asc => query = query.order(dsl::updated_at.asc()),
                Direction::Desc => query = query.order(dsl::updated_at.desc()),
            },
        }
        // This loop must appear before any other filter statements, as it takes advantage of
        // the behaviour of `or_filter` being equal to `filter` when no other filters are present yet.
        // Result needs to be `WHERE (x = $1 [OR x = $2...]) AND ([...])` - note bracketing.
        for pub_id in publishers {
            query = query.or_filter(crate::schema::imprint::publisher_id.eq(pub_id));
        }
        if let Some(pid) = parent_id_1 {
            query = query.filter(dsl::work_id.eq(pid));
        }
        if let Some(pub_type) = publication_type {
            query = query.filter(dsl::publication_type.eq(pub_type));
        }
        if let Some(filter) = filter {
            // ISBN field is nullable, so searching with an empty filter could fail
            if !filter.is_empty() {
                query = query.filter(dsl::isbn.ilike(format!("%{}%", filter)));
            }
        }
        match query
            .limit(limit.into())
            .offset(offset.into())
            .load::<Publication>(&connection)
        {
            Ok(t) => Ok(t),
            Err(e) => Err(ThothError::from(e)),
        }
    }

    fn count(
        db: &crate::db::PgPool,
        filter: Option<String>,
        publishers: Vec<Uuid>,
        publication_type: Option<Self::FilterParameter1>,
        _: Option<Self::FilterParameter2>,
    ) -> ThothResult<i32> {
        use crate::schema::publication::dsl;
        let connection = db.get().unwrap();
        let mut query = dsl::publication
            .inner_join(crate::schema::work::table.inner_join(crate::schema::imprint::table))
            .select((
                dsl::publication_id,
                dsl::publication_type,
                dsl::work_id,
                dsl::isbn,
                dsl::created_at,
                dsl::updated_at,
            ))
            .into_boxed();
        // This loop must appear before any other filter statements, as it takes advantage of
        // the behaviour of `or_filter` being equal to `filter` when no other filters are present yet.
        // Result needs to be `WHERE (x = $1 [OR x = $2...]) AND ([...])` - note bracketing.
        for pub_id in publishers {
            query = query.or_filter(crate::schema::imprint::publisher_id.eq(pub_id));
        }
        if let Some(pub_type) = publication_type {
            query = query.filter(dsl::publication_type.eq(pub_type));
        }
        if let Some(filter) = filter {
            // ISBN field is nullable, so searching with an empty filter could fail
            if !filter.is_empty() {
                query = query.filter(dsl::isbn.ilike(format!("%{}%", filter)));
            }
        }

        // `SELECT COUNT(*)` in postgres returns a BIGINT, which diesel parses as i64. Juniper does
        // not implement i64 yet, only i32. The only sensible way, albeit shameful, to solve this
        // is converting i64 to string and then parsing it as i32. This should work until we reach
        // 2147483647 records - if you are fixing this bug, congratulations on book number 2147483647!
        match query.count().get_result::<i64>(&connection) {
            Ok(t) => Ok(t.to_string().parse::<i32>().unwrap()),
            Err(e) => Err(ThothError::from(e)),
        }
    }

    fn publisher_id(&self, db: &crate::db::PgPool) -> ThothResult<Uuid> {
        crate::model::work::Work::from_id(db, &self.work_id)?.publisher_id(db)
    }

    crud_methods!(publication::table, publication::dsl::publication);
}

impl HistoryEntry for Publication {
    type NewHistoryEntity = NewPublicationHistory;

    fn new_history_entry(&self, account_id: &Uuid) -> Self::NewHistoryEntity {
        Self::NewHistoryEntity {
            publication_id: self.publication_id,
            account_id: *account_id,
            data: serde_json::Value::String(serde_json::to_string(&self).unwrap()),
        }
    }
}

impl DbInsert for NewPublicationHistory {
    type MainEntity = PublicationHistory;

    db_insert!(publication_history::table);
}

impl NewPublication {
    pub fn can_have_isbn(&self, db: &crate::db::PgPool) -> ThothResult<()> {
        publication_can_have_isbn(self.work_id, db)
    }
}

impl PatchPublication {
    pub fn can_have_isbn(&self, db: &crate::db::PgPool) -> ThothResult<()> {
        publication_can_have_isbn(self.work_id, db)
    }
}

fn publication_can_have_isbn(work_id: Uuid, db: &crate::db::PgPool) -> ThothResult<()> {
    use crate::model::work::WorkType;
    use diesel::prelude::*;

    let connection = db.get().unwrap();
    let work_type = crate::schema::work::table
        .select(crate::schema::work::work_type)
        .filter(crate::schema::work::work_id.eq(work_id))
        .first::<WorkType>(&connection)
        .expect("Error loading work type for publication");
    // If a publication's work is of type Book Chapter,
    // it cannot have an ISBN.
    if work_type == WorkType::BookChapter {
        Err(ThothError::ChapterIsbnError)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publication_pk() {
        let publication: Publication = Default::default();
        assert_eq!(publication.pk(), publication.publication_id);
    }

    #[test]
    fn test_new_publication_history_from_publication() {
        let publication: Publication = Default::default();
        let account_id: Uuid = Default::default();
        let new_publication_history = publication.new_history_entry(&account_id);
        assert_eq!(
            new_publication_history.publication_id,
            publication.publication_id
        );
        assert_eq!(new_publication_history.account_id, account_id);
        assert_eq!(
            new_publication_history.data,
            serde_json::Value::String(serde_json::to_string(&publication).unwrap())
        );
    }
}
