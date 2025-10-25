// 浏览器兼容的Elasticsearch服务
interface ServerConfig {
  url: string;
  username?: string;
  password?: string;
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

  private getAuthHeaders() {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    };

    if (this.config.username && this.config.password) {
      const auth = btoa(`${this.config.username}:${this.config.password}`);
      headers['Authorization'] = `Basic ${auth}`;
    }

    return headers;
  }

  private async request(endpoint: string, options: RequestInit = {}) {
    const url = `${this.config.url}${endpoint}`;
    const headers = {
      ...this.getAuthHeaders(),
      ...options.headers,
    };

    try {
      const response = await fetch(url, {
        ...options,
        headers,
        mode: 'cors',
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error(`Request to ${url} failed:`, error);
      throw error;
    }
  }

  // 获取集群健康状态
  async getClusterHealth(): Promise<ClusterHealthResponse> {
    return await this.request('/_cluster/health');
  }

  // 获取集群信息
  async getClusterInfo(): Promise<ClusterInfoResponse> {
    return await this.request('/');
  }

  // 获取节点信息
  async getNodesInfo(): Promise<NodesInfoResponse> {
    return await this.request('/_nodes');
  }

  // 获取索引统计信息
  async getIndicesStats(): Promise<IndicesStatsResponse> {
    return await this.request('/_stats');
  }

  // 获取分片信息
  async getShards(): Promise<CatShardsResponse> {
    return await this.request('/_cat/shards?format=json');
  }
}

export default ElasticsearchService;
export type {
  ServerConfig,
  ClusterHealthResponse,
  ClusterInfoResponse,
  NodesInfoResponse,
  IndicesStatsResponse,
  CatShardsResponse
};