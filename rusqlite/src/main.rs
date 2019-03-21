mod connection_pool;
mod repository;

use connection_pool::SqliteConnectionPool;
use env_logger::{self, Env};
use failure::Error;
use log::info;
use rand::Rng;
use rayon::prelude::*;
use repository::*;
use std::fs;

static TABLE_DDL: &'static str = include_str!("../../up.sql");

fn main() {
    let env = Env::new().filter("MY_LOG");
    env_logger::init_from_env(env);

    info!("Starting rusqlite test");

    fs::create_dir_all("./test_dbs").expect("create dir all");

    let conn_pool = SqliteConnectionPool::new_from_path("./test_dbs/rusqlite.db")
        .expect("create SqliteConnectionPool");

    run_ddl(&conn_pool);

    let task_repo = TaskRepository {
        conn_pool: conn_pool,
    };

    for _n in 1..10001 {
        task_repo
            .insert_task(&InsertTask { status: "WAITING" })
            .expect("insert task");
    }

    while task_repo
        .query_tasks_by_status("WAITING", 0, 100)
        .expect("get tasks")
        .len()
        > 0
    {
        run_test(&task_repo).expect("failed");
    }
}

fn run_ddl(conn_pool: &SqliteConnectionPool) {
    let conn = conn_pool.get().expect("get conn");

    conn.execute_batch(TABLE_DDL).expect("execute DDL");
}

fn run_test(task_repo: &TaskRepository) -> Result<(), Error> {
    let tasks = task_repo.query_tasks_by_status("WAITING", 0, 100)?;

    let res: Result<Vec<_>, Error> = tasks
        .par_iter()
        .map(|task| {
            let mut rng = rand::thread_rng();
            let seed = rng.gen_range(0, 10);
            if seed % 2 == 0 {
                task_repo.update_task_status(task.id, "SUCCESSFUL")
            } else {
                task_repo.set_task_error(task.id, &format!("random error {}", seed))
            }
        })
        .collect();

    res?;

    Ok(())
}
