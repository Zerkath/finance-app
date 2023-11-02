# Preparing the project for testing

Our frontend is written in typescript / javascript and the frontend components are created by svelte.
The backend code is written in rust.

Tauri-app does not ship with testing libraries, so we need to add them manually.
The frontend is using Vite as the build and server tool and we can add Vitest to create tests for our frontend logic.
To test frontend components we will also need to add testing-library for svelte.

Rust already supports testing out of the box with cargo test. 
Although we still want to add tarpaulin for coverage.

## Installing Vitest

For testing frontend components I decided to use Vitest.

To first time install we need to follow the steps in the [guide](https://vitest.dev/guide/).

`npm install -D vitest`

Then we can create a new file called `vitest.config.ts` to project root.
```ts
import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte({ hot: !process.env.VITEST })],
  test: {
    globals: true,
    environment: 'jsdom'
  }
});
```

Once this is done we should append the following to our `package.json` scripts.
```json
"scripts": {
    ..,
    "test:frontend": "vitest run",
    "test:watch": "vitest",
    ..
}
```

If we test our test with `npm run test:frontend` we will get a exit code of 1 because there are no tests.
Temporarily we can create a file [main.test.ts](./src/main.test.ts)
```ts
import { test, expect } from 'vitest';

// Placeholder test to pass the test phase, as no tests returns a non-zero exit code
test('placeholder', () => {
  expect(true).toBe(true);
});
```

Now our test should pass with `npm run test:frontend`

To prepare for testing the frontend components we should also install testing-library for svelte.
`npm install -D @testing-library/svelte`

## Installing tarpaulin

We should add tarpaulin as a build dependency in the [src-tauri/cargo.toml](./src-tauri/cargo.toml)
While we are adding it we might as well add the dependency for sqlite, 
we are using rusqlite features bundled means the sqlite binary will be included and will not be a requirement in the application once bundled.

```toml
[build-dependencies]
cargo-tarpaulin = { version = "0.27.1", features = [] }
..

[dependencies]
rusqlite = {version = "0.29.0", features = ["bundled"] }
```

Also it might be good to install the dependencies to the system.
```
cargo install cargo-tarpaulin
cargo install cargo-watch
```

To interact with cargo more easily from project root we can create a script to `package.json`.

```json
"scripts": {
    ..,
    "cargo": "cd src-tauri; cargo",
    ..
}
```
This allows us to run `npm run cargo <command>`.

We can create a command to run our tests with this will generate a html report.

```json
"scripts": {
    ..,
    "test:backend": "cd src-tauri; cargo tarpaulin -o html --output-dir target/coverage --skip-clean",
    ..
}
```

## Final setup

For code formatting and linting we can add Prettier + Eslint.

```
npm install -D --save-exact prettier
npm install -D eslint
```

The following files are included to control formatting and ignore some folders and files to save resources.
[.prettierrc](.prettierrc)
[.eslintrc.cjs](.eslintrc.cjs)
[.prettierignore](.prettierignore)
[.eslintignore](.eslintignore)

We want to add a few more scripts and in the end our scripts should look this

```json
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "cargo": "cd src-tauri; cargo",
    "test:backend": "cd src-tauri; cargo tarpaulin -o html --output-dir target/coverage --skip-clean",
    "test:frontend": "vitest run",
    "test:frontend:watch": "vitest",
    "test:backend:watch": "cd src-tauri; cargo watch -x test",
    "test:watch": "concurrently --kill-others \"npm run test:frontend:watch\" \"npm run test:backend:watch\"",
    "test": "concurrently --kill-others-on-fail \"npm run test:backend\" \"npm run test:frontend\"",
    "lint": "prettier --check . && eslint .",
    "format": "prettier --write .",
    "check": "svelte-check --tsconfig ./tsconfig.json",
    "tauri": "tauri"
  },
```
