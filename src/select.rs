mod from_subquery;

pub use from_subquery::*;

use sea_orm::{
    sea_query::{MysqlQueryBuilder, PostgresQueryBuilder, SelectStatement, SqliteQueryBuilder},
    ConnectionTrait, DatabaseConnection, DbErr, ExecResult, QueryResult, Statement,
};

#[trait_variant::make(Send)]
pub trait Execute {
    async fn query_all(&self, conn: &DatabaseConnection) -> Result<Vec<QueryResult>, DbErr>;

    async fn query_one(&self, conn: &DatabaseConnection) -> Result<Option<QueryResult>, DbErr>;

    async fn execute(&self, conn: &DatabaseConnection) -> Result<ExecResult, DbErr>;
}

impl Execute for SelectStatement {
    async fn query_all(&self, conn: &DatabaseConnection) -> Result<Vec<QueryResult>, DbErr> {
        let db_backend = conn.get_database_backend();
        let (query, values) = match db_backend {
            sea_orm::DatabaseBackend::MySql => self.build(MysqlQueryBuilder),
            sea_orm::DatabaseBackend::Postgres => self.build(PostgresQueryBuilder),
            sea_orm::DatabaseBackend::Sqlite => self.build(SqliteQueryBuilder),
        };

        conn.query_all(Statement::from_sql_and_values(db_backend, query, values))
            .await
    }

    async fn query_one(&self, conn: &DatabaseConnection) -> Result<Option<QueryResult>, DbErr> {
        let db_backend = conn.get_database_backend();
        let (query, values) = match db_backend {
            sea_orm::DatabaseBackend::MySql => self.build(MysqlQueryBuilder),
            sea_orm::DatabaseBackend::Postgres => self.build(PostgresQueryBuilder),
            sea_orm::DatabaseBackend::Sqlite => self.build(SqliteQueryBuilder),
        };

        conn.query_one(Statement::from_sql_and_values(db_backend, query, values))
            .await
    }

    async fn execute(&self, conn: &DatabaseConnection) -> Result<ExecResult, DbErr> {
        let db_backend = conn.get_database_backend();
        let (query, values) = match db_backend {
            sea_orm::DatabaseBackend::MySql => self.build(MysqlQueryBuilder),
            sea_orm::DatabaseBackend::Postgres => self.build(PostgresQueryBuilder),
            sea_orm::DatabaseBackend::Sqlite => self.build(SqliteQueryBuilder),
        };

        conn.execute(Statement::from_sql_and_values(db_backend, query, values))
            .await
    }
}
