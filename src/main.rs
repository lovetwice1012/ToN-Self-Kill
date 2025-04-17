use enigo::{Enigo, Keyboard, Key, Direction::{Press, Release}, Settings};
use futures_util::stream::StreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use winapi::um::winuser::{FindWindowW, SetForegroundWindow};
use std::{thread, time::Duration};
use serde::{Deserialize, Serialize};
use std::ptr::null_mut;
use std::ffi::CString;

#[derive(Serialize, Deserialize)]
struct Command {
    #[serde(rename = "type")]
    cmd_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:64000").await?;
    println!("WebSocket サーバーがポート64000で待機しています...");
    
    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = tokio_tungstenite::accept_async(stream).await?;
        println!("クライアントと接続されました!");

        let mut ws_stream = ws_stream;
        while let Some(Ok(msg)) = ws_stream.next().await {
            if let Message::Text(text) = msg {
                let command: Command = match serde_json::from_str(&text) {
                    Ok(cmd) => cmd,
                    Err(_) => continue,
                };

                if command.cmd_type == "suside" {
                    focus_vrchat_window();
                    let mut enigo = Enigo::new(&Settings::default()).unwrap();
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
    unsafe {
        let window_name = "VRChat";
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