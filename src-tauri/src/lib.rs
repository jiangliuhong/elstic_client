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
        .invoke_handler(tauri::generate_handler![
            greet,
            test_elasticsearch_connection,
            get_cluster_health,
            get_cluster_info,
            get_nodes_info,
            get_indices_stats,
            get_shards
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}