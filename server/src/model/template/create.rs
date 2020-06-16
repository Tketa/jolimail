use crate::error::ServerError;
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use slug::slugify;

use super::Template;

const QUERY_INSERT: &'static str = r#"
INSERT INTO templates (title, slug, description)
VALUES ($1, $2, $3)
RETURNING id, slug, title, description, current_version_id, created_at, updated_at, deleted_at
"#;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateCreate {
    title: String,
    description: Option<String>,
}

impl TemplateCreate {
    pub async fn save(&self, client: &Client) -> Result<Template, ServerError> {
        debug!("save template {}", self.title);
        let stmt = client.prepare(QUERY_INSERT).await?;
        let slug = slugify(self.title.as_str());
        let rows = client
            .query(&stmt, &[&self.title, &slug, &self.description])
            .await?;
        Ok(rows.first().map(Template::from).unwrap())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::service::database::client::tests::POOL;

    pub async fn create_template(title: &str, description: Option<&str>) -> Template {
        let client = POOL.get().await.unwrap();
        let tmpl = TemplateCreate {
            title: title.to_string(),
            description: description.and_then(|value| Some(value.to_string())),
        };
        tmpl.save(&client).await.unwrap()
    }
}
