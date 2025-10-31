# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## GitHub OAuth Setup

To enable GitHub authentication, you need to:

1. Create a GitHub OAuth App:
   - Go to GitHub Settings > Developer settings > OAuth Apps
   - Click "New OAuth App"
   - Set Authorization callback URL to: `http://localhost:3000/github-callback`
   - Copy the Client ID and replace `your_github_client_id_here` in `src/services/githubAuthService.ts`

2. For production builds, update the redirect URI accordingly.

## Project Setup

```bash
npm install
```

### Compile and Hot-Reload for Development

```bash
npm run tauri dev
```

### Type-Check, Compile and Minify for Production

```bash
npm run tauri build
```
