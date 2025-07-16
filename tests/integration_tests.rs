use rolodex::{
    components::app::{App, AppMode, AppMsg},
    model::Contact,
    Db,
};
use tempfile::tempdir;

fn create_test_app() -> (App, tempfile::TempDir) {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Db::open(&db_path).unwrap();
    let app = App::new(db).unwrap();
    (app, temp_dir)
}

#[test]
fn test_app_initialization() {
    let (app, _temp_dir) = create_test_app();
    
    assert_eq!(app.mode, AppMode::Browse);
    assert!(!app.should_quit);
    assert!(app.selected_contact.is_none());
}

#[test]
fn test_app_quit_message() {
    let (mut app, _temp_dir) = create_test_app();
    
    let result = app.update(AppMsg::Quit, |msg| msg);
    assert!(result.is_none());
    assert!(app.should_quit);
    assert!(app.selected_contact.is_none());
}

#[test]
fn test_app_add_contact_mode() {
    let (mut app, _temp_dir) = create_test_app();
    
    let result = app.update(AppMsg::AddContact, |msg| msg);
    assert!(result.is_none());
    assert_eq!(app.mode, AppMode::Form);
}

#[test]
fn test_app_edit_contact_mode() {
    let (mut app, _temp_dir) = create_test_app();
    
    let contact = Contact {
        id: 1,
        name: "Test User".to_string(),
        email: Some("test@example.com".to_string()),
        phone: Some("0412345678".to_string()),
        company: Some("Test Corp".to_string()),
    };
    
    app.selected_contact = Some(contact.clone());
    
    let result = app.update(AppMsg::EditContact(contact), |msg| msg);
    assert!(result.is_none());
    assert_eq!(app.mode, AppMode::Form);
}

#[test]
fn test_app_confirm_delete_mode() {
    let (mut app, _temp_dir) = create_test_app();
    
    let contact = Contact {
        id: 1,
        name: "Test User".to_string(),
        email: Some("test@example.com".to_string()),
        phone: Some("0412345678".to_string()),
        company: Some("Test Corp".to_string()),
    };
    
    let result = app.update(AppMsg::ConfirmDelete(contact), |msg| msg);
    assert!(result.is_none());
    assert_eq!(app.mode, AppMode::Delete);
}

#[test]
fn test_app_show_error_mode() {
    let (mut app, _temp_dir) = create_test_app();
    
    let error_message = "Test error message".to_string();
    let result = app.update(AppMsg::ShowError(error_message), |msg| msg);
    assert!(result.is_none());
    assert_eq!(app.mode, AppMode::Error);
}

#[test]
fn test_app_with_seeded_data() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let mut db = Db::open(&db_path).unwrap();
    
    db.seed(3).unwrap();
    
    let app = App::new(db).unwrap();
    let contacts = app.db.load_customers().unwrap();
    assert_eq!(contacts.len(), 3);
}