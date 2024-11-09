use sqlx::{Pool, Any, AnyPool, Database};
//use sqlx::mysql::MySqlPool;
use sqlx::sqlite::SqlitePool;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct Db {
    pub(crate) pool: Arc<AnyPool>,
    pub(crate) db_type: DbType,
}

#[derive(Clone, Copy, Debug)]
pub enum DbType {
//    MySQL,
    SQLite,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
//        let db_type = if database_url.starts_with("mysql") {
//            DbType::MySQL   
        let db_type =  if database_url.starts_with("sqlite") {
            DbType::SQLite
        } else {
            println!("Invalid DB Type");
            panic!()
            //return Err(sqlx::Error::Configuration(
            //    println!("Only MySQL and SQLite are supported. URL must start with 'mysql://' or 'sqlite://'")
            //));
        };

        let pool = match db_type {
        //    DbType::MySQL => {
        //        MySqlPool::connect_with(
        //            sqlx::mysql::MySqlConnectOptions::from_str(database_url)?
        //                .connect_timeout(Duration::from_secs(3))
        //        )
        //        .await?
        //        .into()
        //    },
            DbType::SQLite => {
                SqlitePool::connect_with(
                    sqlx::sqlite::SqliteConnectOptions::from_str(database_url)?
                        .create_if_missing(true)
                )
                .await?
                .into()
            }
        };

        // Run migrations based on database type
        //     match db_type {
        //         DbType::MySQL => {
        //             sqlx::migrate!("./migrations/mysql")
        //                 .run(&pool)
        //                 .await?;
        //         },
        //         DbType::SQLite => {
        //             sqlx::migrate!("./migrations/sqlite")
        //                 .run(&pool)
        //                 .await?;
        //         }
        //     }

        Ok(Self {
            pool: Arc::new(pool),
            db_type,
        })
}

    pub fn db_type(&self) -> DbType {
        self.db_type
    }

    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        Self::new(database_url).await
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }

    pub async fn check_health(&self) -> Result<(), sqlx::Error> {
        self.pool.acquire().await.map(|_| ())
    }

    // Helper method to get the appropriate placeholder syntax for the current database
    pub fn placeholder(&self, index: usize) -> String {
        match self.db_type {
//            DbType::MySQL => "?".to_string(),
            DbType::SQLite => format!("${}", index)
        }
    }

    // Helper method to get the appropriate DATETIME function
    pub fn datetime_now(&self) -> &'static str {
        match self.db_type {
//            DbType::MySQL => "NOW()",
            DbType::SQLite => "DATETIME('now')"
        }
    }

    // Helper method for LIKE case sensitivity
    pub fn like_escape(&self, pattern: &str) -> String {
        match self.db_type {
//            DbType::MySQL => format!("LIKE BINARY '{}'", pattern),
            DbType::SQLite => format!("LIKE '{}' COLLATE BINARY", pattern)
        }
    }
}
