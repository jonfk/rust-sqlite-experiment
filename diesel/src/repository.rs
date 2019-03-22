use crate::connection_pool::SqliteConnectionPool;
use diesel::prelude::*;
use failure::Error;
use log::info;
use schema::tasks;

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub status: &'a str,
}

#[derive(Queryable, Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub status: String,
    pub errors: Option<String>,
}

pub struct TaskRepository {
    pub conn_pool: SqliteConnectionPool,
}

impl TaskRepository {
    pub fn new(pool: SqliteConnectionPool) -> TaskRepository {
        TaskRepository { conn_pool: pool }
    }

    pub fn insert_task(&self, task: &NewTask) -> Result<i32, Error> {
        let conn = self.conn_pool.get()?;

        let id = conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(tasks::table)
                .values(task)
                .execute(&conn)?;

            let inserted_task: Task = tasks::table.order(tasks::id.desc()).first(&conn)?;

            info!("inserted task with id={}", inserted_task.id);
            Ok(inserted_task.id)
        })?;
        Ok(id)
    }

    pub fn query_tasks_by_status(
        &self,
        status: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Task>, Error> {
        info!("querying tasks by status={}", status);
        let conn = self.conn_pool.get()?;

        let entities = tasks::table
            .filter(tasks::status.eq(status))
            .offset(offset)
            .limit(limit)
            .load(&conn)?;
        Ok(entities)
    }

    pub fn update_task_status(&self, task_id: i32, status: &str) -> Result<usize, Error> {
        info!("updating task id={}, status={}", task_id, status);
        let conn = self.conn_pool.get()?;
        let affected_rows = diesel::update(tasks::table)
            .set(tasks::status.eq(status))
            .filter(tasks::id.eq(task_id))
            .execute(&conn)?;
        Ok(affected_rows)
    }

    pub fn set_task_error(&self, task_id: i32, errors: &str) -> Result<usize, Error> {
        info!("setting task error id={}, error={}", task_id, errors);
        let conn = self.conn_pool.get()?;

        let affected_rows = diesel::update(tasks::table)
            .set((tasks::status.eq("WAITING"), tasks::errors.eq(errors)))
            .filter(tasks::id.eq(task_id))
            .execute(&conn)?;
        Ok(affected_rows)
    }
}

pub mod schema {
    use diesel::table;
    use diesel::*;

    table! {
        tasks (id) {
            id -> Integer,
            status -> Text,
            errors -> Nullable<Text>,
        }
    }
}
