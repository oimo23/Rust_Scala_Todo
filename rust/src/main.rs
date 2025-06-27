// 外部クレート（ライブラリ）のインポート
use serde::{Deserialize, Serialize}; // JSONシリアライズ/デシリアライズ用のライブラリ
use std::collections::HashMap; // キーと値のペアを格納するためのデータ構造
use std::sync::{Arc, Mutex}; // スレッド間で安全にデータを共有するための型
use uuid::Uuid; // ユニークなID生成用のライブラリ
use warp::Filter; // Webフレームワーク

// ToDoアイテムの構造体定義
// #[derive(...)] はRustのアトリビュート（属性）- この構造体に追加機能を提供する
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,      // ToDoのユニークな識別子
    title: String,   // ToDoのタイトル
    completed: bool, // 完了状態（true=完了、false=未完了）
}

// 新規ToDo作成リクエスト用の構造体
#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String, // 作成するToDoのタイトルのみ必要（idとcompletedはサーバー側で設定）
}

// ToDo更新リクエスト用の構造体
#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,    // タイトルの更新（省略可能）
    completed: Option<bool>,  // 完了状態の更新（省略可能）
}

// ToDoデータを保存するストアの型定義
// Arc = Atomic Reference Counter（複数スレッドから参照可能な参照カウンター）
// Mutex = 一度に一つのスレッドからのみアクセスを許可するロック機構
// HashMap = キー(String)と値(Todo)のマップ
type TodoStore = Arc<Mutex<HashMap<String, Todo>>>;

// メイン関数 - 非同期実行のエントリーポイント
#[tokio::main] // tokioの非同期ランタイムを使用
async fn main() {
    // ToDoアイテムを格納するストアの初期化（空のハッシュマップ）
    let store: TodoStore = Arc::new(Mutex::new(HashMap::new()));

    // CORSの設定 - クロスオリジンリクエストを許可
    let cors = warp::cors()
        .allow_any_origin()        // 全てのオリジン（ドメイン）からのアクセスを許可
        .allow_headers(vec!["content-type"]) // Content-Typeヘッダーを許可
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]); // 許可するHTTPメソッド

    // ストアをルートハンドラに渡すためのフィルター
    let store_filter = warp::any().map(move || store.clone());

    // GET /todos - 全ToDoアイテムの取得
    let get_todos = warp::path("todos")
        .and(warp::get())          // GETリクエストにマッチ
        .and(store_filter.clone()) // ストアをハンドラに渡す
        .and_then(get_todos_handler); // 処理をハンドラに委譲

    // POST /todos - 新規ToDoアイテムの作成
    let create_todo = warp::path("todos")
        .and(warp::post())         // POSTリクエストにマッチ
        .and(warp::body::json())   // JSONリクエストボディを解析
        .and(store_filter.clone()) // ストアをハンドラに渡す
        .and_then(create_todo_handler);

    // PUT /todos/:id - 指定されたIDのToDoアイテムの更新
    let update_todo = warp::path!("todos" / String) // パスパラメータとしてIDを取得
        .and(warp::put())          // PUTリクエストにマッチ
        .and(warp::body::json())   // JSONリクエストボディを解析
        .and(store_filter.clone()) // ストアをハンドラに渡す
        .and_then(update_todo_handler);

    // DELETE /todos/:id - 指定されたIDのToDoアイテムの削除
    let delete_todo = warp::path!("todos" / String) // パスパラメータとしてIDを取得
        .and(warp::delete())       // DELETEリクエストにマッチ
        .and(store_filter.clone()) // ストアをハンドラに渡す
        .and_then(delete_todo_handler);

    // 全てのルートを結合し、CORSを適用
    let routes = get_todos
        .or(create_todo)  // ルートを「または」で結合
        .or(update_todo)
        .or(delete_todo)
        .with(cors);      // CORSを適用

    // サーバーの起動メッセージ
    println!("Server starting on http://localhost:8000");
    // サーバーを起動し、全てのIPアドレスの8000ポートで待ち受け
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

// 全ToDoアイテムを取得するハンドラー
async fn get_todos_handler(store: TodoStore) -> Result<impl warp::Reply, warp::Rejection> {
    // ストアのロックを取得（排他アクセス）
    let todos = store.lock().unwrap();
    // 全ToDoをベクターに変換
    let todos_vec: Vec<Todo> = todos.values().cloned().collect();
    // JSONとして応答
    Ok(warp::reply::json(&todos_vec))
}

// 新しいToDoアイテムを作成するハンドラー
async fn create_todo_handler(
    create_todo: CreateTodo,  // POSTリクエストのJSONボディから解析されたデータ
    store: TodoStore,         // ToDoストア
) -> Result<impl warp::Reply, warp::Rejection> {
    // 新しいToDoアイテムの作成
    let todo = Todo {
        id: Uuid::new_v4().to_string(),  // ランダムなUUID生成
        title: create_todo.title,        // リクエストからタイトルを取得
        completed: false,                // 初期状態は未完了
    };

    // ストアに保存
    let id = todo.id.clone();
    store.lock().unwrap().insert(id.clone(), todo.clone());

    // 作成したToDoをステータスコード201(Created)で返す
    Ok(warp::reply::with_status(
        warp::reply::json(&todo),
        warp::http::StatusCode::CREATED,
    ))
}

// ToDoアイテムを更新するハンドラー
async fn update_todo_handler(
    id: String,               // パスパラメータからのID
    update_todo: UpdateTodo,  // PUTリクエストのJSONボディ
    store: TodoStore,         // ToDoストア
) -> Result<impl warp::Reply, warp::Rejection> {
    // ストアのロックを取得
    let mut todos = store.lock().unwrap();
    
    // 指定されたIDのToDoを探す
    if let Some(todo) = todos.get_mut(&id) {
        // タイトルが指定されていれば更新
        if let Some(title) = update_todo.title {
            todo.title = title;
        }
        // 完了状態が指定されていれば更新
        if let Some(completed) = update_todo.completed {
            todo.completed = completed;
        }
        // 更新されたToDoを返す
        Ok(warp::reply::json(todo))
    } else {
        // IDが見つからない場合は404エラー
        Err(warp::reject::not_found())
    }
}

// ToDoアイテムを削除するハンドラー
async fn delete_todo_handler(
    id: String,       // パスパラメータからのID
    store: TodoStore, // ToDoストア
) -> Result<impl warp::Reply, warp::Rejection> {
    // ストアのロックを取得
    let mut todos = store.lock().unwrap();
    
    // 指定されたIDのToDoを削除
    if todos.remove(&id).is_some() {
        // 削除成功の場合、204 No Contentを返す
        Ok(warp::reply::with_status(
            "Todo deleted",
            warp::http::StatusCode::NO_CONTENT,
        ))
    } else {
        // IDが見つからない場合は404エラー
        Err(warp::reject::not_found())
    }
}