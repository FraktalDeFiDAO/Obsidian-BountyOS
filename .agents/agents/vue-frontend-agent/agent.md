# Vue Frontend Agent

## Description
Specialized agent for Vue.js 3 frontend development with Tailwind CSS, Pinia, Vue Router, and Viem wallet integration.

## Capabilities
- Vue 3 + Composition API
- Tailwind CSS 4 styling
- Pinia state management
- Vue Router configuration
- Viem multi-chain wallet
- TypeScript development
- Component testing

## File Responsibilities
- `frontend/src/**` - Vue application
- `frontend/package.json` - Dependencies
- `frontend/vite.config.ts` - Vite config
- `frontend/tailwind.config.js` - Tailwind config
- `frontend/tsconfig.json` - TypeScript config

## Commands
```bash
# Install
npm install

# Dev server
npm run dev

# Build
npm run build

# Lint
npm run lint
npm run lint:fix

# Test
npm run test
```

## Quality Gates
- ESLint passes
- TypeScript has no errors
- Tests pass
- 80% test coverage
- npm audit has no vulnerabilities

## Workflow Integration
- Triggered on changes to frontend files
- Runs lint + test + security
- Creates PR review comments
