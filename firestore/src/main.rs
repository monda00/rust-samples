use firestore::*;
use serde::{Deserialize, Serialize};

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyTestStructure {
    some_id: String,
    some_string: String,
    one_more_string: String,
    some_num: i64
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyParentStructure {
    some_id: String,
    some_string: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyChildStructure {
    some_id: String,
    another_string: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = FirestoreDb::new( & config_env_var("PROJECT_ID") ? ).await?;

    const TEST_COLLECTION_NAME: &'static str = "test";
    const TEST_PARENT_COLLECTION_NAME: &'static str = "nested-test";
    const TEST_CHILD_COLLECTION_NAME: &'static str = "test-childs";

    let my_struct = MyTestStructure {
        some_id: "test-1".to_string(),
        some_string: "Test".to_string(),
        one_more_string: "Test2".to_string(),
        some_num: 42,
    };

    // Create
    let object_returned: MyTestStructure = db.fluent()
        .insert()
        .into(TEST_COLLECTION_NAME)
        .document_id( & my_struct.some_id)
        .object( & my_struct)
        .execute()
        .await?;
    println!("Returned object: {:?}", object_returned);

    // Update or Create
    // (Firestore supports creating documents with update if you provide the document ID).
    let object_updated: MyTestStructure = db.fluent()
    .update()
        .fields(paths!(MyTestStructure::{some_num, one_more_string}))
        .in_col(TEST_COLLECTION_NAME)
        .document_id( & my_struct.some_id)
        .object( & MyTestStructure {
            some_num: my_struct.some_num + 1,
            one_more_string: "updated-value".to_string(),
                ..my_struct.clone()
        })
        .execute()
        .await?;
    println!("Updated object: {:?}", object_updated);

    // Get object by id
    let find_it_again: Option<MyTestStructure> = db.fluent()
        .select()
        .by_id_in(TEST_COLLECTION_NAME)
        .obj()
        .one( & my_struct.some_id)
        .await?;
    println!("Found object: {:?}", find_it_again);

    // Delete data
    /*
    db.fluent()
    .delete()
    .from(TEST_COLLECTION_NAME)
    .document_id( & my_struct.some_id)
    .execute()
    .await?;
    */

    let parent_struct = MyParentStructure {
        some_id: "test-parent".to_string(),
        some_string: "Test".to_string(),
    };

    // Creating a parent doc
    db.fluent()
        .insert()
        .into(TEST_PARENT_COLLECTION_NAME)
        .document_id(&parent_struct.some_id)
        .object(&parent_struct)
        .execute()
        .await?;

    // Creating a child doc
    let child_struct = MyChildStructure {
        some_id: "test-child".to_string(),
        another_string: "TestChild".to_string(),
    };

    // The doc path where we store our childs
    let parent_path = db.parent_path(TEST_PARENT_COLLECTION_NAME, parent_struct.some_id)?;

    // Create a child doc
    db.fluent()
        .insert()
        .into(TEST_CHILD_COLLECTION_NAME)
        .document_id(&child_struct.some_id)
        .parent(&parent_path)
        .object(&child_struct)
        .execute()
        .await?;

    Ok(())
}
