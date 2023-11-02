Creating the tauri-app
```
npm create tauri-app@latest
Need to install the following packages:
  create-tauri-app@3.9.0
Ok to proceed? (y) y
✔ Project name · finance-app
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, bun)
✔ Choose your package manager · npm
✔ Choose your UI template · Svelte - (https://svelte.dev/)
✔ Choose your UI flavor · TypeScript

Template created! To get started run:
  cd finance-app
  npm install
  npm run tauri dev
```
---
First time running the application to generate cargo lock
```
npm run tauri dev
```

Initializing the git repository
```
git init
git branch -M main
git add .
git commit -m "feat: Initial commit"
git remote add origin git@github.com:Zerkath/finance-app.git
git push -u origin main
```
