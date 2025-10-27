// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use elasticsearch::{
    http::transport::Transport,
    Elasticsearch,
    Error,
    CatCatShardsParts,
    ClusterHealthParts,
    NodesParts,
    IndicesIndicesStatsParts,
};
use serde_json::Value;

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
                .info(NodesParts::None)
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
                .stats(IndicesIndicesStatsParts::None)
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
                .shards(CatCatShardsParts::None)
                .format(Some("json"))
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

fn create_elasticsearch_client(url: &str, username: Option<&str>, password: Option<&str>) -> Result<Elasticsearch, Error> {
    let transport = if let (Some(user), Some(pass)) = (username, password) {
        // Create transport with authentication
        Transport::single_node_with_auth(url, user, pass)?
    } else {
        // Create transport without authentication
        Transport::single_node(url)?
    };
    
    Ok(Elasticsearch::new(transport))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_cluster_health,
            get_cluster_info,
            get_nodes_info,
            get_indices_stats,
            get_shards
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
