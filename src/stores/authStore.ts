import { ref } from 'vue'

interface GitHubUser {
  id: number
  login: string
  avatar_url: string
  name?: string
  email?: string
}

// 从localStorage加载认证状态
const loadAuthState = () => {
  const token = localStorage.getItem('github_access_token')
  const userStr = localStorage.getItem('github_user')
  
  let user: GitHubUser | null = null
  if (userStr) {
    try {
      user = JSON.parse(userStr)
    } catch (e) {
      console.error('Failed to parse user from localStorage:', e)
      localStorage.removeItem('github_user')
    }
  }
  
  return {
    isAuthenticated: token !== null,
    accessToken: token,
    githubUser: user
  }
}

// 认证状态
const initialState = loadAuthState()
const isAuthenticated = ref(initialState.isAuthenticated)
const githubUser = ref<GitHubUser | null>(initialState.githubUser)
const accessToken = ref<string | null>(initialState.accessToken)

// 设置认证状态
const setAuth = (token: string, user: GitHubUser) => {
  console.log('Setting auth state:', { token, user })
  isAuthenticated.value = true
  accessToken.value = token
  githubUser.value = user
  
  // 保存到localStorage
  localStorage.setItem('github_access_token', token)
  localStorage.setItem('github_user', JSON.stringify(user))
  
  console.log('Auth state set:', { isAuthenticated: isAuthenticated.value, accessToken: accessToken.value, githubUser: githubUser.value })
}

// 清除认证状态
const clearAuth = () => {
  isAuthenticated.value = false
  accessToken.value = null
  githubUser.value = null
  
  // 从localStorage清除
  localStorage.removeItem('github_access_token')
  localStorage.removeItem('github_user')
}

// 获取访问令牌
const getAccessToken = () => {
  return accessToken.value
}

// 获取用户信息
const getUser = () => {
  return githubUser.value
}

// 检查是否已认证
const getIsAuthenticated = () => {
  return isAuthenticated.value
}

export {
  type GitHubUser,
  isAuthenticated,
  githubUser,
  accessToken,
  setAuth,
  clearAuth,
  getAccessToken,
  getUser,
  getIsAuthenticated
}