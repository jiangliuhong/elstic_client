import { invoke } from '@tauri-apps/api/core'

// 将服务器配置保存到GitHub Gist
export const saveServerConfigToGist = async (serverConfig: any): Promise<string> => {
  try {
    const gistUrl = await invoke<string>('save_server_config_to_gist', { serverConfig })
    return gistUrl
  } catch (error) {
    console.error('Failed to save server config to gist:', error)
    throw error
  }
}

// 从GitHub Gist获取服务器配置
export const getServerConfigFromGist = async (gistUrl: string): Promise<any> => {
  try {
    const serverConfig = await invoke<any>('get_server_config_from_gist', { gistUrl })
    return serverConfig
  } catch (error) {
    console.error('Failed to get server config from gist:', error)
    throw error
  }
}