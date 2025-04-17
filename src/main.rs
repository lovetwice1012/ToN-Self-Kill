use enigo::{Direction::{Press, Release}, Enigo, Key};
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error as WsError;
use winapi::um::winuser::{FindWindowW, SetForegroundWindow};
use std::ptr::null_mut;
use std::ffi::CString;

#[derive(Serialize, Deserialize)]
struct Command {
    #[serde(rename = "type")]
    cmd_type: String,
}

#[tokio::main]
async fn main() -> Result<(), WsError> {
    // WebSocket サーバーの待機
    let listener = TcpListener::bind("127.0.0.1:64000").await?;
    println!("WebSocket サーバーがポート64000で待機しています...");
    
    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = tokio_tungstenite::accept_async(stream).await?;
        println!("クライアントと接続されました!");

        // メッセージを受信
        let mut ws_stream = ws_stream;
        while let Some(Ok(msg)) = ws_stream.next().await {
            if let Message::Text(text) = msg {
                let command: Command = match serde_json::from_str(&text) {
                    Ok(cmd) => cmd,
                    Err(_) => continue,
                };

                // type=suside の場合にキー操作を実行
                if command.cmd_type == "suside" {
                    // VRChat.exe のウィンドウをフォーカス
                    focus_vrchat_window();
                    
                    // キー操作の実行
                    let mut enigo = Enigo::new().unwrap();
                    enigo.key(Key::Other(187), Press).unwrap();
                    enigo.key(Key::Other(222), Press).unwrap();
                    thread::sleep(Duration::from_secs(5));
                    enigo.key(Key::Other(187), Release).unwrap();
                    enigo.key(Key::Other(222), Release).unwrap();
                }
            }
        }
    }
    Ok(())
}

fn focus_vrchat_window() {
    // VRChat.exe のウィンドウを探してフォーカス
    unsafe {
        let window_name = "VRChat"; // ウィンドウのタイトル
        let window_name_wide: Vec<u16> = CString::new(window_name)
            .unwrap()
            .into_bytes_with_nul()
            .iter()
            .map(|&x| x as u16)
            .collect();
        
        let hwnd = FindWindowW(null_mut(), window_name_wide.as_ptr());
        if !hwnd.is_null() {
            SetForegroundWindow(hwnd);
        }
    }
}