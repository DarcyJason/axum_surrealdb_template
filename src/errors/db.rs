use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error")]
    ConnectionError {
        #[source]
        source: anyhow::Error,
        context: String,
    },
    #[error("Query execution failed")]
    QueryError {
        #[source]
        source: anyhow::Error,
        query: Option<String>,
    },
    #[error("Transaction failed")]
    TransactionError {
        #[source]
        source: anyhow::Error,
        operation: String,
    },
    #[error("Record not found")]
    NotFound(String),
    #[error("Constraint violation")]
    ConstraintViolation(String),
}

impl DatabaseError {
    pub fn connection_failed(error: impl Into<anyhow::Error>, context: impl Into<String>) -> Self {
        Self::ConnectionError {
            source: error.into(),
            context: context.into(),
        }
    }

    pub fn query_failed(error: impl Into<anyhow::Error>, query: Option<String>) -> Self {
        Self::QueryError {
            source: error.into(),
            query,
        }
    }

    pub fn transaction_failed(
        error: impl Into<anyhow::Error>,
        operation: impl Into<String>,
    ) -> Self {
        Self::TransactionError {
            source: error.into(),
            operation: operation.into(),
        }
    }
}

impl From<surrealdb::Error> for DatabaseError {
    fn from(error: surrealdb::Error) -> Self {
        match error {
            surrealdb::Error::Db(db_error) => {
                if db_error.to_string().contains("not found") {
                    DatabaseError::NotFound(db_error.to_string())
                } else if db_error.to_string().contains("constraint") {
                    DatabaseError::ConstraintViolation(db_error.to_string())
                } else {
                    DatabaseError::query_failed(anyhow::Error::new(db_error), None)
                }
            }
            _ => DatabaseError::query_failed(anyhow::Error::new(error), None),
        }
    }
}
