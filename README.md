<h1 align="center">ts-cdk</h1>
<p align="center">CLI tool for TypeScript + AWS CDK project.</p>

<br />

<p align="center">
  <a href="https://github.com/ren-yamanashi/ts-cdk/blob/main/LICENSE" style="margin-right: 10px;">
    <img src="https://img.shields.io/github/license/ren-yamanashi/ts-cdk" alt="License">
  </a>
  <a href="https://github.com/ren-yamanashi/ts-cdk/releases/latest" style="margin-right: 10px;">
    <img src="https://img.shields.io/github/v/release/ren-yamanashi/ts-cdk" alt="Release">
  </a>
  <a href="https://github.com/ren-yamanashi/ts-cdk/actions/workflows/test.yml" style="margin-right: 10px;">
    <img src="https://github.com/ren-yamanashi/ts-cdk/actions/workflows/test.yml/badge.svg" alt="Test">
  </a>
</p>

<p align="center">
  <img src="https://github.com/user-attachments/assets/7c0ace00-55e3-4d80-87ce-a4aa3b340604" alt="ts-cdk">
</p>

## 📦 Installation

### Using Shell Script (Recommended)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/ren-yamanashi/ts-cdk/releases/download/v0.1.4/ts-cdk-installer.sh | sh
```

### Manual Installation

1. Download the appropriate binary for your platform from the [Releases](https://github.com/ren-yamanashi/ts-cdk/releases)
2. Extract the archive and move the binary to a directory in your PATH

## 🚀 Usage

### Create a new project

```bash
ts-cdk init <directory>

# Generating Project Structure Example
# my-cdk-app
# ├── bin
# │   └── my-cdk-app.ts
# ├── lib
# │   └── my-cdk-app-stack.ts
# ├── test
# │   └── my-cdk-app.test.ts
# ├── cdk.json
# ├── eslint.config.mjs
# ├── package.json
# ├── README.md
# ├── tsconfig.json
# └── vitest.config.mjs
```

- directory: required
  - Target directory for the new project(When target directory is not exists, it will be created)

## ❗ Issue

If you have any questions or suggestions, please open an [issue](https://github.com/ren-yamanashi/ts-cdk/issues).

## ©️ License

[MIT](LICENSE)
