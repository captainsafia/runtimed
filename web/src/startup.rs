/*
  On start we need to collect all the Jupyter runtimes currently in the system and track new ones.

  With runtimelib, we can detect all the existing Jupyter kernels:

  ```rust
  use runtimelib::jupyter::discovery;

  discovery::get_jupyter_runtime_instances().await;
  ```

*/

use chrono::Utc;
use runtimelib::jupyter::client::JupyterClient;
use sqlx::Sqlite;

/**
 * Wishing for:
 * - What runtime ID did this come from?
 * - What execution did this come from? (likely known with the parent_header.message_id)
 *
 * Note:
 * We could drop any messages that are not outputs or which aren't
 */
pub async fn gather_messages(
    runtime_id: String,
    mut client: JupyterClient,
    db: sqlx::Pool<Sqlite>,
) {
    loop {
        // As each message comes in on iopub, shove to database
        let message = client.next_io().await;

        let created_at = Utc::now();
        let new_id = uuid::Uuid::new_v4();

        if let Ok(message) = message {
            // Database the message
            let res = sqlx::query!(
                r#"INSERT INTO disorganized_messages
                    (id, msg_id, msg_type, content, metadata, runtime_id, parent_msg_id, parent_msg_type, created_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
                new_id,
                message.header["msg_id"],
                message.header["msg_type"],
                message.content,
                message.metadata,
                runtime_id,
                message.parent_header["msg_id"],
                message.parent_header["msg_type"],
                created_at,
            );

            if let Ok(_) = res.execute(&db).await {
                // Log success
            } else {
                // Log error
            }
        } else {
            // Log error
        }
    }
}
