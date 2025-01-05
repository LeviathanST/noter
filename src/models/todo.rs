use color_eyre::Result;
use ratatui::{style::Style, widgets::Row};
use sqlx::{
    prelude::FromRow,
    query,
    types::chrono::{self, Local, TimeZone},
    SqlitePool,
};

use crate::utils::into_row;

#[derive(FromRow, Debug)]
pub struct Todo {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<Local>,
}

impl Todo {
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Todo>> {
        let todos = query!(
            r#"SELECT id, name, description, created_at FROM todo ORDER BY created_at DESC"#
        )
        .fetch_all(pool)
        .await?;

        let todos = todos
            .iter()
            .map(|todo| Todo {
                id: todo.id,
                name: todo.name.to_string(),
                description: todo.description.to_string(),
                created_at: Local.from_utc_datetime(&todo.created_at),
            })
            .collect::<Vec<Todo>>();

        Ok(todos)
    }

    pub async fn insert_one(pool: &SqlitePool, description: String) -> Result<()> {
        query!(
            r#"
            INSERT INTO todo (description)
            VALUES (?)
        "#,
            description
        )
        .execute(pool)
        .await?;

        return Ok(());
    }

    pub fn into_row<'a>(&self) -> Row<'a> {
        return into_row(
            [
                &self.id.to_string(),
                &self.name,
                &self.created_at.to_string(),
            ],
            Style::default(),
        );
    }
}
