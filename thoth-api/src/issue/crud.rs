use super::model::{Issue, IssueField, IssueHistory, NewIssue, NewIssueHistory, PatchIssue};
use crate::errors::{ThothError, ThothResult};
use crate::graphql::model::IssueOrderBy;
use crate::graphql::utils::Direction;
use crate::model::{Crud, DbInsert, HistoryEntry};
use crate::schema::{issue, issue_history};
use crate::{crud_methods, db_insert};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

impl Crud for Issue {
    type NewEntity = NewIssue;
    type PatchEntity = PatchIssue;
    type OrderByEntity = IssueOrderBy;
    type FilterParameter1 = ();
    type FilterParameter2 = ();

    fn pk(&self) -> uuid::Uuid {
        self.issue_id
    }

    fn all(
        db: &crate::db::PgPool,
        limit: i32,
        offset: i32,
        _: Option<String>,
        order: Self::OrderByEntity,
        publishers: Vec<uuid::Uuid>,
        parent_id_1: Option<uuid::Uuid>,
        parent_id_2: Option<uuid::Uuid>,
        _: Option<Self::FilterParameter1>,
        _: Option<Self::FilterParameter2>,
    ) -> ThothResult<Vec<Issue>> {
        use crate::schema::issue::dsl::*;
        let connection = db.get().unwrap();
        let mut query = issue
            .inner_join(crate::schema::series::table.inner_join(crate::schema::imprint::table))
            .select((
                issue_id,
                series_id,
                work_id,
                issue_ordinal,
                created_at,
                updated_at,
            ))
            .into_boxed();

        match order.field {
            IssueField::IssueId => match order.direction {
                Direction::Asc => query = query.order(issue_id.asc()),
                Direction::Desc => query = query.order(issue_id.desc()),
            },
            IssueField::SeriesId => match order.direction {
                Direction::Asc => query = query.order(series_id.asc()),
                Direction::Desc => query = query.order(series_id.desc()),
            },
            IssueField::WorkId => match order.direction {
                Direction::Asc => query = query.order(work_id.asc()),
                Direction::Desc => query = query.order(work_id.desc()),
            },
            IssueField::IssueOrdinal => match order.direction {
                Direction::Asc => query = query.order(issue_ordinal.asc()),
                Direction::Desc => query = query.order(issue_ordinal.desc()),
            },
            IssueField::CreatedAt => match order.direction {
                Direction::Asc => query = query.order(created_at.asc()),
                Direction::Desc => query = query.order(created_at.desc()),
            },
            IssueField::UpdatedAt => match order.direction {
                Direction::Asc => query = query.order(updated_at.asc()),
                Direction::Desc => query = query.order(updated_at.desc()),
            },
        }
        // This loop must appear before any other filter statements, as it takes advantage of
        // the behaviour of `or_filter` being equal to `filter` when no other filters are present yet.
        // Result needs to be `WHERE (x = $1 [OR x = $2...]) AND ([...])` - note bracketing.
        for pub_id in publishers {
            query = query.or_filter(crate::schema::imprint::publisher_id.eq(pub_id));
        }
        if let Some(pid) = parent_id_1 {
            query = query.filter(work_id.eq(pid));
        }
        if let Some(pid) = parent_id_2 {
            query = query.filter(series_id.eq(pid));
        }
        match query
            .limit(limit.into())
            .offset(offset.into())
            .load::<Issue>(&connection)
        {
            Ok(t) => Ok(t),
            Err(e) => Err(ThothError::from(e)),
        }
    }

    fn count(
        db: &crate::db::PgPool,
        _: Option<String>,
        _: Vec<uuid::Uuid>,
        _: Option<Self::FilterParameter1>,
        _: Option<Self::FilterParameter2>,
    ) -> ThothResult<i32> {
        use crate::schema::issue::dsl::*;
        let connection = db.get().unwrap();

        // `SELECT COUNT(*)` in postgres returns a BIGINT, which diesel parses as i64. Juniper does
        // not implement i64 yet, only i32. The only sensible way, albeit shameful, to solve this
        // is converting i64 to string and then parsing it as i32. This should work until we reach
        // 2147483647 records - if you are fixing this bug, congratulations on book number 2147483647!
        match issue.count().get_result::<i64>(&connection) {
            Ok(t) => Ok(t.to_string().parse::<i32>().unwrap()),
            Err(e) => Err(ThothError::from(e)),
        }
    }

    crud_methods!(issue::table, issue::dsl::issue);
}

impl HistoryEntry for Issue {
    type NewHistoryEntity = NewIssueHistory;

    fn new_history_entry(&self, account_id: &uuid::Uuid) -> Self::NewHistoryEntity {
        Self::NewHistoryEntity {
            issue_id: self.issue_id,
            account_id: *account_id,
            data: serde_json::Value::String(serde_json::to_string(&self).unwrap()),
        }
    }
}

impl DbInsert for NewIssueHistory {
    type MainEntity = IssueHistory;

    db_insert!(issue_history::table);
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for Issue {
        fn default() -> Self {
            Issue {
                issue_id: Default::default(),
                series_id: Default::default(),
                work_id: Default::default(),
                issue_ordinal: Default::default(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }
        }
    }

    #[test]
    fn test_issue_pk() {
        let issue: Issue = Default::default();
        assert_eq!(issue.pk(), issue.issue_id);
    }

    #[test]
    fn test_new_issue_history_from_issue() {
        let issue: Issue = Default::default();
        let account_id: uuid::Uuid = Default::default();
        let new_issue_history = issue.new_history_entry(&account_id);
        assert_eq!(new_issue_history.issue_id, issue.issue_id);
        assert_eq!(new_issue_history.account_id, account_id);
        assert_eq!(
            new_issue_history.data,
            serde_json::Value::String(serde_json::to_string(&issue).unwrap())
        );
    }
}
