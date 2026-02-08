pub mod common;

use common::dummy_factory;

#[tokio::test]
//create a test function for populating notes table using dummy_factory
async fn populate_notes_table_test() {
    // Populate notes table
    dummy_factory::populate_notes().await.unwrap();
}

#[tokio::test]
//create a test function for clearing notes table using dummy_factory
async fn clear_notes_table_test() {
    // Clear notes table
    dummy_factory::clear_notes().await.unwrap();
}