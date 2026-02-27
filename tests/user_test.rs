pub mod common;

use common::dummy_factory;

//create a test function for populating users table using dummy_factory
#[tokio::test]
async fn populate_users_table_test() {
    // Populate users table
    dummy_factory::populate_users().await.unwrap();
}

//create a test function for clearing users table using dummy_factory
#[tokio::test]
async fn clear_users_table_test() {
    // Clear users table
    dummy_factory::clear_users().await.unwrap();
}