// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use elasticsearch::{
    auth::Credentials,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch,
    Error,
    cluster::ClusterHealthParts,
    indices::IndicesStatsParts,
    nodes::NodesInfoParts,
    cat::CatShardsParts,
};
use serde_json::Value;
use url::Url;
use std::sync::Mutex;



// GitHub认证状态管理
struct GitHubAuthState {
    access_token: Option<String>,
    user_info: Option<Value>,
}

impl GitHubAuthState {
    fn new() -> Self {
        GitHubAuthState {
            access_token: None,
            user_info: None,
        }
    }
    
    fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }
    
    fn set_user_info(&mut self, user: Value) {
        self.user_info = Some(user);
    }
    
    fn clear(&mut self) {
        self.access_token = None;
        self.user_info = None;
    }
    
    fn get_access_token(&self) -> Option<&String> {
        self.access_token.as_ref()
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_cluster_health(url: &str, username: Option<&str>, password: Option<&str>) -> Result<Value, String> {
    match create_elasticsearch_client(url, username, password) {
        Ok(client) => {
            match client
                .cluster()
                .health(ClusterHealthParts::None)
                .send()
                .await {
                    Ok(response) => {
                        match response.json::<Value>().await {
                            Ok(json) => Ok(json),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Failed to get cluster health: {}", e)),
                }
        },
        Err(e) => Err(format!("Failed to create Elasticsearch client: {}", e)),
    }
}

#[tauri::command]
async fn get_cluster_info(url: &str, username: Option<&str>, password: Option<&str>) -> Result<Value, String> {
    match create_elasticsearch_client(url, username, password) {
        Ok(client) => {
            // Root endpoint for cluster info
            match client
                .info()
                .send()
                .await {
                    Ok(response) => {
                        match response.json::<Value>().await {
                            Ok(json) => Ok(json),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Failed to get cluster info: {}", e)),
                }
        },
        Err(e) => Err(format!("Failed to create Elasticsearch client: {}", e)),
    }
}

#[tauri::command]
async fn get_nodes_info(url: &str, username: Option<&str>, password: Option<&str>) -> Result<Value, String> {
    match create_elasticsearch_client(url, username, password) {
        Ok(client) => {
            match client
                .nodes()
                .info(NodesInfoParts::None)
                .send()
                .await {
                    Ok(response) => {
                        match response.json::<Value>().await {
                            Ok(json) => Ok(json),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Failed to get nodes info: {}", e)),
                }
        },
        Err(e) => Err(format!("Failed to create Elasticsearch client: {}", e)),
    }
}

#[tauri::command]
async fn get_indices_stats(url: &str, username: Option<&str>, password: Option<&str>) -> Result<Value, String> {
    match create_elasticsearch_client(url, username, password) {
        Ok(client) => {
            match client
                .indices()
                .stats(IndicesStatsParts::None)
                .send()
                .await {
                    Ok(response) => {
                        match response.json::<Value>().await {
                            Ok(json) => Ok(json),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Failed to get indices stats: {}", e)),
                }
        },
        Err(e) => Err(format!("Failed to create Elasticsearch client: {}", e)),
    }
}

#[tauri::command]
async fn get_shards(url: &str, username: Option<&str>, password: Option<&str>) -> Result<Value, String> {
    match create_elasticsearch_client(url, username, password) {
        Ok(client) => {
            match client
                .cat()
                .shards(CatShardsParts::None)
                .format("json")
                .send()
                .await {
                    Ok(response) => {
                        match response.json::<Value>().await {
                            Ok(json) => Ok(json),
                            Err(e) => Err(format!("Failed to parse response: {}", e)),
                        }
                    },
                    Err(e) => Err(format!("Failed to get shards: {}", e)),
                }
        },
        Err(e) => Err(format!("Failed to create Elasticsearch client: {}", e)),
    }
}

#[tauri::command]
async fn test_elasticsearch_connection(url: &str, username: Option<&str>, password: Option<&str>) -> Result<Value, String> {
    match create_elasticsearch_client(url, username, password) {
        Ok(client) => {
            // 先尝试获取集群信息
            match client.info().send().await {
                Ok(response) => {
                    match response.json::<Value>().await {
                        Ok(json) => {
                            // 检查是否是有效的ElasticSearch响应
                            if json.get("tagline").is_some() || json.get("version").is_some() {
                                Ok(serde_json::json!({
                                    "success": true
                                }))
                            } else {
                                Err("服务器响应不符合ElasticSearch格式".to_string())
                            }
                        },
                        Err(_e) => Err("解析响应失败".to_string()),
                    }
                },
                Err(_e) => {
                    // 如果根路径失败，尝试集群健康检查
                    match client.cluster().health(ClusterHealthParts::None).send().await {
                        Ok(response) => {
                            match response.json::<Value>().await {
                                Ok(json) => {
                                    if json.get("cluster_name").is_some() || json.get("status").is_some() {
                                        Ok(serde_json::json!({
                                            "success": true
                                        }))
                                    } else {
                                        Err("服务器响应不符合ElasticSearch格式".to_string())
                                    }
                                },
                                Err(_e) => Err("解析响应失败".to_string()),
                            }
                        },
                        Err(e) => {
                            // 提供友好的错误信息
                            let error_msg = e.to_string();
                            if error_msg.contains("401") {
                                Err("认证失败，请检查用户名和密码".to_string())
                            } else if error_msg.contains("403") {
                                Err("权限不足".to_string())
                            } else if error_msg.contains("404") {
                                Err("找不到ElasticSearch服务，请检查URL".to_string())
                            } else if error_msg.contains("unreachable") || error_msg.contains("connection") {
                                Err("网络连接失败，请检查服务器地址".to_string())
                            } else {
                                Err(format!("连接失败: {}", error_msg))
                            }
                        }
                    }
                }
            }
        },
        Err(e) => Err(format!("创建Elasticsearch客户端失败: {}", e)),
    }
}

// GitHub OAuth相关命令
#[tauri::command]
async fn start_github_auth(app_handle: tauri::AppHandle) -> Result<String, String> {
    // 启动本地OAuth服务器
    let port = start_oauth_server(app_handle.clone()).await?;
    
    // 生成GitHub OAuth授权URL
    let client_id = "Ov23liO0TRgl6Agag1K9"; // GitHub OAuth应用的Client ID
    let redirect_uri = format!("http://localhost:{}/callback", port); // 使用动态端口
    let scope = "read:user,user:email,gist"; // 添加gist权限以存储服务器信息
    let state = generate_random_state();
    
    let auth_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}&state={}",
        client_id, redirect_uri, scope, state
    );
    
    Ok(auth_url)
}

#[tauri::command]
async fn finish_github_auth(
    code: String, 
    state: tauri::State<'_, Mutex<GitHubAuthState>>
) -> Result<Value, String> {
    // 使用code交换access_token
    let client = reqwest::Client::new();
    let params = [
        ("client_id", "Ov23liO0TRgl6Agag1K9"),
        ("client_secret", "a4a181b86835eb17affeff9a6d4027cf759a7f35"),
        ("code", &code),
        ("redirect_uri", "http://localhost:1420/callback"),
    ];
    
    let response = client
        .post("https://github.com/login/oauth/access_token")
        .form(&params)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to exchange code for token: {}", e))?;
    
    let token_response: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;
    
    let access_token = token_response["access_token"]
        .as_str()
        .ok_or("No access token in response")?
        .to_string();
    
    // 获取用户信息
    let user_response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "Elastic Client")
        .send()
        .await
        .map_err(|e| format!("Failed to get user info: {}", e))?;
    
    let user_info: Value = user_response
        .json()
        .await
        .map_err(|e| format!("Failed to parse user info: {}", e))?;
    
    // 更新状态
    {
        let mut state = state.lock().unwrap();
        state.set_access_token(access_token.clone());
        state.set_user_info(user_info.clone());
    }
    
    // 返回用户信息
    Ok(serde_json::json!({
        "access_token": access_token,
        "user": user_info
    }))
}

#[tauri::command]
async fn logout_github(state: tauri::State<'_, Mutex<GitHubAuthState>>) -> Result<(), String> {
    // 清除本地存储的认证信息
    let mut state = state.lock().unwrap();
    state.clear();
    Ok(())
}

// 将服务器信息存储到GitHub Gist
#[tauri::command]
async fn save_server_config_to_gist(
    server_config: Value,
    state: tauri::State<'_, Mutex<GitHubAuthState>>
) -> Result<String, String> {
    // 获取访问令牌
    let access_token;
    {
        let state = state.lock().unwrap();
        access_token = state.get_access_token()
            .ok_or("User not authenticated")?
            .clone();
    }

    // 创建Gist的JSON数据
    let gist_data = serde_json::json!({
        "description": "Elasticsearch Server Configuration",
        "public": false,
        "files": {
            "server-config.json": {
                "content": serde_json::to_string_pretty(&server_config)
                    .map_err(|e| format!("Failed to serialize server config: {}", e))?
            }
        }
    });

    // 发送请求到GitHub API
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.github.com/gists")
        .header("Authorization", format!("token {}", access_token))
        .header("User-Agent", "Elastic Client")
        .json(&gist_data)
        .send()
        .await
        .map_err(|e| format!("Failed to create gist: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to create gist: {}", response.status()));
    }

    let gist_response: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse gist response: {}", e))?;

    // 返回Gist URL
    let gist_url = gist_response["html_url"]
        .as_str()
        .ok_or("No URL in gist response")?
        .to_string();

    Ok(gist_url)
}

// 从GitHub Gist获取服务器信息
#[tauri::command]
async fn get_server_config_from_gist(
    gist_url: &str,
    state: tauri::State<'_, Mutex<GitHubAuthState>>
) -> Result<Value, String> {
    // 获取访问令牌
    let access_token;
    {
        let state = state.lock().unwrap();
        access_token = state.get_access_token()
            .ok_or("User not authenticated")?
            .clone();
    }

    // 从URL中提取Gist ID
    let gist_id = extract_gist_id(gist_url)?;

    // 构造API URL
    let api_url = format!("https://api.github.com/gists/{}", gist_id);

    // 发送请求到GitHub API
    let client = reqwest::Client::new();
    let response = client
        .get(&api_url)
        .header("Authorization", format!("token {}", access_token))
        .header("User-Agent", "Elastic Client")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch gist: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch gist: {}", response.status()));
    }

    let gist_data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse gist data: {}", e))?;

    // 提取服务器配置
    let files = gist_data["files"]
        .as_object()
        .ok_or("No files in gist")?;

    let server_config_content = files
        .get("server-config.json")
        .and_then(|file| file["content"].as_str())
        .ok_or("No server-config.json in gist")?;

    let server_config: Value = serde_json::from_str(server_config_content)
        .map_err(|e| format!("Failed to parse server config: {}", e))?;

    Ok(server_config)
}

// 辅助函数：从Gist URL中提取Gist ID
fn extract_gist_id(url: &str) -> Result<String, String> {
    let url_parts: Vec<&str> = url.split('/').collect();
    if url_parts.len() < 2 {
        return Err("Invalid gist URL".to_string());
    }
    
    let gist_id = url_parts[url_parts.len() - 1].to_string();
    Ok(gist_id)
}

use std::sync::Arc;
use std::thread;
use tiny_http::{Server, Response, StatusCode};
use tauri::{Emitter, Manager};

// 启动本地HTTP服务器处理OAuth回调
#[tauri::command]
async fn start_oauth_server(app_handle: tauri::AppHandle) -> Result<u16, String> {
    // 找一个可用端口
    let port = find_available_port().map_err(|e| format!("Failed to find available port: {}", e))?;
    
    let server_url = format!("http://localhost:{}", port);
    let auth_state = Arc::new(Mutex::new(GitHubAuthState::new()));
    
    // 克隆状态以供服务器使用
    let server_state = auth_state.clone();
    let app_handle_clone = app_handle.clone();
    
    // 在新线程中启动HTTP服务器
    thread::spawn(move || {
        let server = Server::http(format!("localhost:{}", port))
            .expect("Failed to start server");
        
        println!("OAuth callback server started on port {}", port);
        
        for request in server.incoming_requests() {
            let url = request.url();
            
            if url.starts_with("/callback") {
                // 解析URL参数
                let parsed_url = url::Url::parse(&format!("http://localhost:{}{}", port, url))
                    .map_err(|e| format!("Failed to parse callback URL: {}", e));
                
                if let Ok(parsed_url) = parsed_url {
                    let code = parsed_url.query_pairs()
                        .find(|(key, _)| key == "code")
                        .map(|(_, value)| value.to_string());
                    
                    if let Some(code) = code {
                        // 处理GitHub回调
                        match handle_github_callback_internal(&code, &server_state) {
                            Ok(_) => {
                                // 通知前端认证成功，并传递用户信息和访问令牌
                                let user_info = {
                                    let state = server_state.lock().unwrap();
                                    (state.get_access_token().cloned(), state.user_info.clone())
                                };
                                
                                if let (Some(token), Some(user)) = user_info {
                                    println!("发送OAuth成功事件，token: {:?}, user: {:?}", token, user);
                                    let result = app_handle_clone.emit("oauth-success", serde_json::json!({
                                        "access_token": token,
                                        "user": user
                                    }));
                                    match result {
                                        Ok(_) => println!("OAuth成功事件发送成功"),
                                        Err(e) => eprintln!("OAuth成功事件发送失败: {}", e),
                                    }
                                } else {
                                    println!("发送OAuth成功事件（无用户信息）");
                                    let _ = app_handle_clone.emit("oauth-success", ());
                                }
                                
                                // 返回成功页面
                                let response = Response::from_string("<html><head><meta charset=\"UTF-8\"></head><body><h1>认证成功！</h1><p>您可以关闭此浏览器窗口并返回应用程序。</p></body></html>")
                                    .with_status_code(StatusCode(200))
                                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                                let _ = request.respond(response);
                                break; // 停止服务器
                            }
                            Err(e) => {
                                eprintln!("OAuth callback error: {}", e);
                                let response = Response::from_string(format!("<html><head><meta charset=\"UTF-8\"></head><body><h1>认证失败</h1><p>{}</p></body></html>", e))
                                    .with_status_code(StatusCode(400))
                                    .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                                let _ = request.respond(response);
                                break; // 停止服务器
                            }
                        }
                    } else {
                        let response = Response::from_string("<html><head><meta charset=\"UTF-8\"></head><body><h1>错误</h1><p>未找到授权码</p></body></html>")
                            .with_status_code(StatusCode(400))
                            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                        let _ = request.respond(response);
                        break;
                    }
                } else {
                    let response = Response::from_string("<html><head><meta charset=\"UTF-8\"></head><body><h1>错误</h1><p>无效的回调URL</p></body></html>")
                        .with_status_code(StatusCode(400))
                        .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                    let _ = request.respond(response);
                    break;
                }
            } else {
                // 404 for other paths
                let response = Response::from_string("Not Found")
                    .with_status_code(StatusCode(404));
                let _ = request.respond(response);
            }
        }
        
        println!("OAuth callback server stopped");
    });
    
    Ok(port)
}

// 查找可用端口
fn find_available_port() -> Result<u16, Box<dyn std::error::Error>> {
    use std::net::TcpListener;
    
    // 尝试从30000-40000范围内找一个可用端口
    for port in 30000..40000 {
        if let Ok(_) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
            return Ok(port);
        }
    }
    
    Err("No available port found".into())
}

// 内部处理GitHub回调的函数
fn handle_github_callback_internal(
    code: &str,
    state: &Arc<Mutex<GitHubAuthState>>
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::runtime::Runtime;
    
    let rt = Runtime::new()?;
    rt.block_on(async {
        // 使用现有的finish_github_auth逻辑
        let client = reqwest::Client::new();
        let params = [
            ("client_id", "Ov23liO0TRgl6Agag1K9"),
            ("client_secret", "a4a181b86835eb17affeff9a6d4027cf759a7f35"),
            ("code", code),
            ("redirect_uri", "http://localhost:30000/callback"), // 注意：这里应该使用实际的回调URL，但GitHub不验证
        ];
        
        let response = client
            .post("https://github.com/login/oauth/access_token")
            .form(&params)
            .header("Accept", "application/json")
            .send()
            .await?;
        
        let token_response: Value = response.json().await?;
        
        let access_token = token_response["access_token"]
            .as_str()
            .ok_or("No access token in response")?
            .to_string();
        
        // 获取用户信息
        let user_response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "Elastic Client")
            .send()
            .await?;
        
        let user_info: Value = user_response.json().await?;
        
        // 更新状态
        {
            let mut state = state.lock().unwrap();
            state.set_access_token(access_token.clone());
            state.set_user_info(user_info.clone());
        }
        
        Ok(())
    })
}

fn create_elasticsearch_client(url: &str, username: Option<&str>, password: Option<&str>) -> Result<Elasticsearch, Error> {
    let url = Url::parse(url)?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    
    let transport = if let (Some(user), Some(pass)) = (username, password) {
        // Create transport with authentication
        let credentials = Credentials::Basic(user.to_string(), pass.to_string());
        let transport = TransportBuilder::new(conn_pool).auth(credentials).build()?;
        transport
    } else {
        // Create transport without authentication
        TransportBuilder::new(conn_pool).build()?
    };
    
    Ok(Elasticsearch::new(transport))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        
        .plugin(tauri_plugin_deep_link::init())
        .manage(Mutex::new(GitHubAuthState::new()))
        .invoke_handler(tauri::generate_handler![
            greet,
            test_elasticsearch_connection,
            get_cluster_health,
            get_cluster_info,
            get_nodes_info,
            get_indices_stats,
            get_shards,
            start_github_auth,
            finish_github_auth,
            logout_github,
            save_server_config_to_gist,
            get_server_config_from_gist,
            start_oauth_server
        ])
        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_random_state() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const STATE_LEN: usize = 32;
    
    let mut rng = rand::thread_rng();
    (0..STATE_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}