import { getIsAuthenticated, getUser, getAccessToken, setAuth, clearAuth } from '../stores/authStore'
import { invoke } from '@tauri-apps/api/core'
import { openUrl } from '@tauri-apps/plugin-opener'
import { listen } from '@tauri-apps/api/event'

// 创建GitHub授权URL
export const createGitHubAuthUrl = async (): Promise<string> => {
  try {
    const authUrl = await invoke<string>('start_github_auth')
    return authUrl
  } catch (error) {
    console.error('Failed to create GitHub auth URL:', error)
    throw error
  }
}

// 处理GitHub回调
export const handleGitHubCallback = async (code: string): Promise<boolean> => {
  try {
    console.log('Invoking finish_github_auth with code:', code)
    const result: any = await invoke('finish_github_auth', { code })
    console.log('finish_github_auth result:', result)
    
    // 检查结果结构
    if (result && result.user) {
      console.log('Setting auth with user:', result.user)
      setAuth(result.access_token, {
        id: result.user.id,
        login: result.user.login,
        avatar_url: result.user.avatar_url,
        name: result.user.name,
        email: result.user.email
      })
      console.log('Auth set successfully')
      return true
    }
    
    console.log('Invalid result structure:', result)
    return false
  } catch (error) {
    console.error('GitHub回调处理失败:', error)
    return false
  }
}

// 打开GitHub登录页面
export const openGitHubLogin = async (): Promise<void> => {
  try {
    const authUrl = await createGitHubAuthUrl()
    // 使用系统默认浏览器打开授权页面
    await openUrl(authUrl)
  } catch (error) {
    console.error('Failed to open GitHub login:', error)
    throw error
  }
}

// 监听OAuth回调事件
export const listenForOAuthCallback = async (callback: (code: string) => void): Promise<(() => void)> => {
  const unlisten = await listen('oauth-callback', (event: any) => {
    if (event.payload) {
      try {
        const url = new URL(event.payload)
        const code = url.searchParams.get('code')
        if (code) {
          callback(code)
        } else {
          console.error('No code in OAuth callback URL')
        }
      } catch (error) {
        console.error('Failed to parse OAuth callback URL:', error)
      }
    }
  })
  
  return unlisten
}

// 登出功能
export const logout = async (): Promise<void> => {
  try {
    await invoke('logout_github')
    clearAuth()
  } catch (error) {
    console.error('登出失败:', error)
    // 即使Tauri调用失败，也要清除本地状态
    clearAuth()
  }
}

// 检查认证状态
export const checkAuthStatus = () => {
  return getIsAuthenticated()
}

// 获取当前用户
export const getCurrentUser = () => {
  return getUser()
}