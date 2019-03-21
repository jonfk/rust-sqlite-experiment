use crate::connection_pool::SqliteConnectionPool;
use failure::Error;
use log::{info, trace};
use rusqlite::{types::ToSql, Row, Rows};
use std::fs;

pub struct InsertTask<'a> {
    pub status: &'a str,
}

pub struct TaskEntity {
    pub id: i32,
    pub status: String,
    pub errors: Option<String>,
}

pub struct TaskRepository {
    pub conn_pool: SqliteConnectionPool,
}

impl TaskRepository {
    pub fn insert_task(&self, task: &InsertTask) -> Result<i64, Error> {
        let conn = self.conn_pool.get()?;

        let mut stmt = conn.prepare("INSERT INTO tasks (status) VALUES (:status)")?;

        let id = stmt.insert(&[&task.status as &ToSql])?;
        trace!("inserted {:?} with id={}", task.status, id);
        Ok(id)
    }

    pub fn query_tasks_by_status(
        &self,
        status: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<TaskEntity>, Error> {
        trace!("querying tasks by status={}", status);
        let conn = self.conn_pool.get()?;

        let mut stmt = conn.prepare(
            "SELECT id, status, errors FROM tasks WHERE status = :status ORDER BY id ASC LIMIT :limit OFFSET :offset",
        )?;
        let mut rows = stmt.query_named(&[
            (":status", &status as &ToSql),
            (":limit", &limit),
            (":offset", &offset),
        ])?;
        tasks_from_rows(&mut rows)
    }

    pub fn update_task_status(&self, task_id: i32, status: &str) -> Result<usize, Error> {
        trace!("updating task id={}, status={}", task_id, status);
        let conn = self.conn_pool.get()?;
        let mut stmt = conn.prepare("UPDATE tasks SET status = :status WHERE id = :task_id")?;

        let rows_affected = stmt.execute(&[&status, &task_id as &ToSql])?;
        Ok(rows_affected)
    }

    pub fn set_task_error(&self, task_id: i32, errors: &str) -> Result<usize, Error> {
        trace!("setting task error id={}, error={}", task_id, errors);
        let conn = self.conn_pool.get()?;

        let mut stmt = conn
            .prepare("UPDATE tasks SET status = :status, errors = :errors WHERE id = :task_id")?;

        let rows_affected = stmt.execute(&[&"WAITING" as &ToSql, &errors as &ToSql, &task_id])?;
        Ok(rows_affected)
    }
}

pub fn tasks_from_rows(rows: &mut Rows) -> Result<Vec<TaskEntity>, Error> {
    let mut tasks = Vec::new();
    while let Some(result_row) = rows.next() {
        let row = result_row?;
        let task = task_from_row(&row)?;
        tasks.push(task);
    }
    Ok(tasks)
}

pub fn task_from_row(row: &Row) -> Result<TaskEntity, Error> {
    Ok(TaskEntity {
        id: row.get(0),
        status: row.get(1),
        errors: row.get(2),
    })
}
