import eslintCdkPlugin from "eslint-cdk-plugin";
import tsEslint from "typescript-eslint";

export default [
  ...tsEslint.configs.recommended,
  {
    files: ["lib/**/*.ts", "bin/*.ts"],
    languageOptions: {
      parserOptions: {
        projectService: true,
        project: "./tsconfig.json",
      },
    },
    plugins: {
      cdk: eslintCdkPlugin,
    },
    rules: {
      ...eslintCdkPlugin.configs.recommended.rules,
    },
  },
  {
    ignores: ["node_modules", "*.js"],
  },
];
