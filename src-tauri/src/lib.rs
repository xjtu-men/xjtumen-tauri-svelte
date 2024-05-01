use futures::StreamExt;
use tauri::{Manager, Url, WebviewUrl};
use tokio;
// use tauri_api::config::Config;
use tauri_plugin_shell::ShellExt;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    const URLS: &'static [&'static str] = &["https://raw.githubusercontent.com/xjtu-men/domains/main/xjtu.men.server.ip/",
        "https://xjtu.app/xjtumen-g/server_info/xjtu.men.server.ip",
        "https://gitea.com/xjtu-men/domains/raw/branch/main/xjtu.men.server.ip",
        "https://pastebin.pl/view/raw/ab81176b",
        "https://pastebin.com/raw/vujw32uH"
    ];

    tauri::Builder::default()
            .plugin(tauri_plugin_shell::init())
            .plugin(tauri_plugin_notification::init())
            .plugin(tauri_plugin_http::init())
            .invoke_handler(tauri::generate_handler![greet])
            .setup(|app| {
                // let client = Client::new();
                // let bodies = future::join_all(URLS.into_iter().map(|url| {
                //     let client = &client;
                //     async move {
                //         let resp = client.get(*url).send().await?;
                //         resp.bytes().await
                //     }
                // }));
                let handle = app.handle().clone();
                let shell = handle.shell();
                let output = tauri::async_runtime::block_on(async move {
                    shell
                            .command("echo")
                            .args(["Hello from Rust!"])
                            .output()
                            .await
                            .unwrap()
                });
                if output.status.success() {
                    println!("Result: {:?}", String::from_utf8(output.stdout));
                } else {
                    println!("Exit with code: {}", output.status.code().unwrap());
                }
                // tauri::async_runtime::spawn(async move {
                //     let (mut rx, mut child) = handle.shell().command("/usr/bin/darkhttpd")
                //             .args(["/home/liu/books/rail.eecs.berkeley.edu/", "--port", "5801", "--addr", "127.0.0.1"])
                //             .spawn()
                //             .expect("Failed to spawn darkhttpd server");
                //     println!("Started subprocess: {}", child.pid());
                // });


                // if app.notification().permission_state()? == PermissionState::Unknown {
                //     app.notification().request_permission()?;
                // }
                // if app.notification().permission_state()? == PermissionState::Granted {
                //     app.notification()
                //             .builder()
                //             .body("XJTU.MEN APP welcomes you!")
                //             .show()?;
                // }

                // let response_data = tauri::async_runtime::block_on(async {
                //     let response = reqwest::get(
                //         "https://xjtu.app/none",
                //     )
                //             .await
                //             .unwrap();
                //     response.text().await
                // })?;

                let window = app.get_window("main").unwrap();
                // let _ = window.destroy();
                let window = tauri::window::WindowBuilder::new(app, "webview").build()?;
                // let title = Config::get().unwrap().title.unwrap_or("xmen app".to_string());
                // window.set_title("交大門 Tauri App");

                let url = Url::parse("https://xjtu.men:443")?;
                let webview_builder = tauri::webview::WebviewBuilder::new(
                    "label", WebviewUrl::External(url))
                        // .proxy_url(Url::parse("socks5://127.0.0.1:2588")?) // may cause white screen
                        ;
                let webview = window.add_child(webview_builder,
                                               tauri::LogicalPosition::new(0, 0),
                                               window.inner_size().unwrap());
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
}
