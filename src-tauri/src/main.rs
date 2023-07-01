// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

pub(crate) mod database;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board{
    columns:Vec<Column>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column{
    id:i64,
    title:String,
    cards:Vec<Card>,
}

impl Column {
    pub fn new(id:i64, title: &str) -> Self {
        Column {
            id,
            title:title.to_string(),
            cards:Vec::new(),
        }
    }

    pub fn add_card(&mut self, card:Card){
        self.cards.push(card);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card{
    id:i64,
    title:String,
    description:Option<String>,
}

impl Card {
    pub fn new(id:i64, title:&str, description:Option<String>) -> Self{
        Card {
            id,
            title: title.to_string(),
            description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardPos{
    #[serde(rename = "columunId")]
    column_id:i64,
    position:i64,
}


// ボードのデータを作成して返すハンドラ
#[tauri::command]
async fn get_board(sqlite_pool: State<'_, sqlx::SqlitePool>) -> Result<Board, String>{
    let columns = database::get_columns(&*sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Board { columns })
}

/// カードの追加直後に呼ばれるハンドラ
#[tauri::command]
async fn handle_add_card(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    card: Card,
    pos: CardPos,
) -> Result<(), String> {
    database::insert_card(&*sqlite_pool, card, pos)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// カードの移動直後に呼ばれるハンドラ
#[tauri::command]
async fn handle_move_card(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    card: Card,
    from:CardPos,
    to:CardPos,
) -> Result<(), String> {
    database::move_card(&*sqlite_pool, card, from, to)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// カードの削除直後に呼ばれるハンドラ
#[tauri::command]
async fn handle_remove_card(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    card: Card,
    column_id:i64
) -> Result<(), String> {
    database::delete_card(&*sqlite_pool, card, column_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {

    use tauri::async_runtime::block_on;

    // データベースのファイルパス等を設定する
    const DATABASE_DIR: &str = "kadai-kanban-db";
    const DATABASE_FILE: &str = "db.sqlite";
    // ユーザのホームディレクトリ直下にデータベースのディレクトリを作成する
    // もし、各OSで標準的に使用されるアプリ専用のデータディレクトリに保存したいなら
    // directoriesクレートの`ProjectDirs::data_dir`メソッドなどを使うとよい
    // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.data_dir
    let home_dir = directories::UserDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        // ホームディレクトリが取得できないときはカレントディレクトリを使う
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory"));
    let database_dir = home_dir.join(DATABASE_DIR);
    let database_file = database_dir.join(DATABASE_FILE);

    // データベースファイルが存在するかチェックする
    let db_exists = std::fs::metadata(&database_file).is_ok();
    // 存在しないなら、ファイルを格納するためのディレクトリを作成する
    if !db_exists {
        std::fs::create_dir(&database_dir)?;
    }

    // データベースURLを作成
    let database_dir_str = dunce::canonicalize(&database_dir)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    let database_url = format!("sqlite://{}/{}", database_dir_str, DATABASE_FILE);

    // SQLiteのコネクションプールを作成する
    let sqlite_pool = block_on(database::create_sqlite_pool(&database_url))?;

    //  データベースファイルが存在しなかったなら、マイグレーションSQLを実行する
    if !db_exists {
        block_on(database::migrate_database(&sqlite_pool))?;
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_board,
            handle_add_card,
            handle_move_card,
            handle_remove_card
        ])
        .setup(|app| {
            app.manage(sqlite_pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
