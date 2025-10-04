module.exports = {
  root: true,
  env: {
    node: true,
    es2022: true
  },
  extends: [
    'eslint:recommended',
    '@typescript-eslint/recommended'
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 2022,
    sourceType: 'module'
  },
  plugins: ['@typescript-eslint'],
  rules: {
    'no-magic-numbers': 'off',
    '@typescript-eslint/no-explicit-any': 'warn',
    'require-await': 'off'
  },
  ignorePatterns: ['dist/', 'node_modules/', '*.js']
};