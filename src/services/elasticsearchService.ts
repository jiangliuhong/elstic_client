// Tauri命令方式的Elasticsearch服务
import { invoke } from '@tauri-apps/api/core';

interface ServerConfig {
  url: string;
  username?: string;
  password?: string;
}

interface Server {
  id: string;
  name: string;
  url: string;
  username?: string;
  password?: string;
  connected?: boolean;
  connectionError?: string;
}

interface ClusterHealthResponse {
  cluster_name: string;
  status: string;
  timed_out: boolean;
  number_of_nodes: number;
  number_of_data_nodes: number;
  active_primary_shards: number;
  active_shards: number;
  relocating_shards: number;
  initializing_shards: number;
  unassigned_shards: number;
  delayed_unassigned_shards: number;
  number_of_pending_tasks: number;
  number_of_in_flight_fetch: number;
  task_max_waiting_in_queue_millis: number;
  active_shards_percent_as_number: number;
}

interface ClusterInfoResponse {
  name: string;
  cluster_name: string;
  cluster_uuid: string;
  version: {
    number: string;
    build_flavor: string;
    build_type: string;
    build_hash: string;
    build_date: string;
    build_snapshot: boolean;
    lucene_version: string;
    minimum_wire_compatibility_version: string;
    minimum_index_compatibility_version: string;
  };
  tagline: string;
}

interface NodesInfoResponse {
  nodes: any;
}

interface IndicesStatsResponse {
  indices: any;
}

interface CatShardsResponse {
  [index: number]: {
    index: string;
    shard: string;
    prirep: string;
    state: string;
    docs: string;
    store: string;
    ip: string;
    node: string;
  };
}

class ElasticsearchService {
  private config: ServerConfig;

  constructor(config: ServerConfig) {
    this.config = config;
  }

  // 测试服务器连接
  static async testConnection(server: Server): Promise<{ success: boolean; error?: string }> {
    try {
      const result = await invoke('test_elasticsearch_connection', {
        url: server.url,
        username: server.username,
        password: server.password
      });
      
      return result as { success: boolean; error?: string };
    } catch (error) {
      let errorMessage = '未知错误'
      
      if (error instanceof Error) {
        errorMessage = error.message
      }
      
      return { success: false, error: errorMessage }
    }
  }

  // 获取集群健康状态
  async getClusterHealth(): Promise<ClusterHealthResponse> {
    try {
      const result = await invoke('get_cluster_health', {
        url: this.config.url,
        username: this.config.username,
        password: this.config.password
      });
      return result as ClusterHealthResponse;
    } catch (error) {
      console.error('获取集群健康状态失败:', error);
      throw error;
    }
  }

  // 获取集群信息
  async getClusterInfo(): Promise<ClusterInfoResponse> {
    try {
      const result = await invoke('get_cluster_info', {
        url: this.config.url,
        username: this.config.username,
        password: this.config.password
      });
      return result as ClusterInfoResponse;
    } catch (error) {
      console.error('获取集群信息失败:', error);
      throw error;
    }
  }

  // 获取节点信息
  async getNodesInfo(): Promise<NodesInfoResponse> {
    try {
      const result = await invoke('get_nodes_info', {
        url: this.config.url,
        username: this.config.username,
        password: this.config.password
      });
      return result as NodesInfoResponse;
    } catch (error) {
      console.error('获取节点信息失败:', error);
      throw error;
    }
  }

  // 获取索引统计信息
  async getIndicesStats(): Promise<IndicesStatsResponse> {
    try {
      const result = await invoke('get_indices_stats', {
        url: this.config.url,
        username: this.config.username,
        password: this.config.password
      });
      return result as IndicesStatsResponse;
    } catch (error) {
      console.error('获取索引统计信息失败:', error);
      throw error;
    }
  }

  // 获取分片信息
  async getShards(): Promise<CatShardsResponse> {
    try {
      const result = await invoke('get_shards', {
        url: this.config.url,
        username: this.config.username,
        password: this.config.password
      });
      return result as CatShardsResponse;
    } catch (error) {
      console.error('获取分片信息失败:', error);
      throw error;
    }
  }
}

export default ElasticsearchService;
export type {
  ServerConfig,
  Server,
  ClusterHealthResponse,
  ClusterInfoResponse,
  NodesInfoResponse,
  IndicesStatsResponse,
  CatShardsResponse
};