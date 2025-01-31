# ts-cdk

CLI tool for TypeScript + AWS CDK project.

## Installation

### Using Shell Script (Recommended)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/ren-yamanashi/ts-cdk/releases/download/v0.1.4/ts-cdk-installer.sh | sh
```

### Manual Installation

1. Download the appropriate binary for your platform from the [releases page](https://github.com/ren-yamanashi/ts-cdk/releases)
2. Extract the archive and move the binary to a directory in your PATH

## Usage

### Create a new project

```bash
ts-cdk init <directory>

# e.g.
# ts-cdk init my-cdk-app
```

#### Arguments

- `<directory>`: Target directory for the new project (required)

## Generated Project Structure

```
my-cdk-app
├── bin
│   └── my-cdk-app.ts
├── lib
│   └── my-cdk-app-stack.ts
├── test
│   └── my-cdk-app.test.ts
├── cdk.json
├── eslint.config.mjs
├── package.json
├── README.md
├── tsconfig.json
└── vitest.config.mjs
```

## License

[MIT](LICENSE)
