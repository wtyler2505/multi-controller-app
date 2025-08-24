import js from '@eslint/js';
import typescript from '@typescript-eslint/eslint-plugin';
import typescriptParser from '@typescript-eslint/parser';
import prettier from 'eslint-config-prettier';
import prettierPlugin from 'eslint-plugin-prettier';

export default [
  // Base JavaScript configuration
  js.configs.recommended,

  // Prettier configuration to disable conflicting rules
  prettier,

  // Global ignores
  {
    ignores: [
      'node_modules/**',
      'dist/**',
      'build/**',
      'out/**',
      'bin/**',
      'obj/**',
      '*.min.js',
      '*.min.css',
      '.claude/**',
      '.taskmaster/**',
      '.desktop-commander/**',
      '.filescope/**',
      '.clear-thought/**',
      '.context7/**',
      '.perplexity-ask/**',
      '.memory/**',
      '.time-server/**',
      '**/*.d.ts',
      'coverage/**',
      '.husky/**',
    ],
  },

  // JavaScript/TypeScript files configuration
  {
    files: ['**/*.{js,jsx,ts,tsx,mjs,cjs}'],
    languageOptions: {
      parser: typescriptParser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        ecmaFeatures: {
          jsx: true,
        },
      },
      globals: {
        console: 'readonly',
        process: 'readonly',
        Buffer: 'readonly',
        __dirname: 'readonly',
        __filename: 'readonly',
        exports: 'writable',
        module: 'writable',
        require: 'readonly',
        global: 'readonly',
        window: 'readonly',
        document: 'readonly',
      },
    },
    plugins: {
      '@typescript-eslint': typescript,
      prettier: prettierPlugin,
    },
    rules: {
      // Prettier integration
      'prettier/prettier': [
        'error',
        {
          endOfLine: 'crlf',
        },
      ],

      // General best practices
      'no-console': ['warn', { allow: ['warn', 'error', 'info'] }],
      'no-debugger': 'error',
      'no-alert': 'error',
      'no-unused-vars': 'off', // Turned off in favor of TypeScript's version
      'no-use-before-define': 'off',
      'no-redeclare': 'off',
      'no-shadow': 'off',

      // TypeScript specific rules
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          ignoreRestSiblings: true,
        },
      ],
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/explicit-function-return-type': 'off',
      '@typescript-eslint/explicit-module-boundary-types': 'off',
      '@typescript-eslint/no-non-null-assertion': 'warn',
      '@typescript-eslint/no-use-before-define': [
        'error',
        {
          functions: false,
          classes: true,
          variables: true,
        },
      ],

      // Code quality
      eqeqeq: ['error', 'always', { null: 'ignore' }],
      'no-duplicate-imports': 'error',
      'no-var': 'error',
      'prefer-const': 'error',
      'prefer-template': 'warn',
      'prefer-arrow-callback': 'warn',
      'arrow-body-style': ['warn', 'as-needed'],

      // Error prevention
      'no-throw-literal': 'error',
      'no-return-await': 'error',
      'no-async-promise-executor': 'error',
      'no-promise-executor-return': 'error',
      'require-await': 'error',

      // Hardware/Device specific rules for Multi-Controller App
      'no-magic-numbers': [
        'warn',
        {
          ignore: [0, 1, -1, 2, 100, 1000],
          ignoreArrayIndexes: true,
          enforceConst: true,
          detectObjects: false,
        },
      ],
    },
  },

  // Test files - relaxed rules
  {
    files: ['**/*.test.{js,ts}', '**/*.spec.{js,ts}', '**/tests/**'],
    rules: {
      'no-console': 'off',
      'no-magic-numbers': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
    },
  },

  // Configuration files
  {
    files: ['*.config.{js,mjs,cjs,ts}', '.*.{js,mjs,cjs,ts}'],
    rules: {
      'no-console': 'off',
      '@typescript-eslint/no-var-requires': 'off',
    },
  },
];
