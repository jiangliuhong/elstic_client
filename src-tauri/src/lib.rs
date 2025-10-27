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
            get_cluster_health,
            get_cluster_info,
            get_nodes_info,
            get_indices_stats,
            get_shards
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}