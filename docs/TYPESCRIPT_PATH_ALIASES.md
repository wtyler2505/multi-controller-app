# TypeScript Path Aliases Configuration

## Overview
TypeScript path aliases have been configured for the Multi-Controller App to provide cleaner import statements and better code organization. This setup uses `tsconfig-paths` for runtime resolution.

## Configuration

### Path Aliases Defined (tsconfig.json)
The following path aliases are available for use throughout the TypeScript codebase:

- `@app/*` → `src/app/*` - Application core modules
- `@drivers/*` → `src/drivers/*` - Device driver implementations  
- `@transports/*` → `src/transports/*` - Transport layer modules (Serial, TCP/UDP, SSH)
- `@interfaces/*` → `src/interfaces/*` - TypeScript interfaces and type definitions
- `@utils/*` → `src/utils/*` - Utility functions and helpers
- `@config/*` → `src/config/*` - Configuration files and constants
- `@tests/*` → `tests/*` - Test utilities and helpers

### Runtime Resolution
- **Package**: `tsconfig-paths` (v4.2.0) installed as devDependency
- **Configuration**: Automatically loaded via `ts-node` configuration in tsconfig.json
- **Nodemon**: Explicitly configured with `-r tsconfig-paths/register` flag

### Testing Support
- **Jest**: Configured with `moduleNameMapper` in jest.config.js to resolve aliases during testing
- **Coverage**: Path aliases work correctly with code coverage tools

## Usage Examples

### Before (Relative Imports)
```typescript
import { DeviceDriver } from '../../../interfaces/device-driver';
import { SerialTransport } from '../../../transports/serial';
import { logger } from '../../../utils/logger';
```

### After (Path Aliases)
```typescript
import { DeviceDriver } from '@interfaces/device-driver';
import { SerialTransport } from '@transports/serial';
import { logger } from '@utils/logger';
```

## Development Workflow

### Running with ts-node
```bash
# Path aliases are automatically resolved
npx ts-node src/app/main.ts
```

### Running with nodemon
```bash
# Uses tsconfig-paths/register automatically
npm run dev
```

### Building for Production
```bash
# TypeScript compiler resolves aliases during build
npm run build
```

### Running Tests
```bash
# Jest moduleNameMapper handles alias resolution
npm test
```

## IDE Support

### Visual Studio Code
VS Code automatically recognizes path aliases from tsconfig.json. IntelliSense and auto-imports work out of the box.

### WebStorm/IntelliJ IDEA
JetBrains IDEs automatically detect and use the path mappings from tsconfig.json.

### Other Editors
Most modern TypeScript-aware editors will recognize the path mappings defined in tsconfig.json.

## Troubleshooting

### Module Resolution Errors
If you encounter "Cannot find module" errors:

1. Ensure `tsconfig-paths` is installed:
   ```bash
   pnpm add -D tsconfig-paths
   ```

2. Verify the `baseUrl` is set correctly in tsconfig.json:
   ```json
   "baseUrl": "."
   ```

3. Check that the path mapping matches your file structure

### Runtime Errors
If aliases work during development but fail at runtime:

1. Ensure ts-node configuration includes tsconfig-paths:
   ```json
   "ts-node": {
     "require": ["tsconfig-paths/register"]
   }
   ```

2. For production builds, consider using a bundler that handles path aliases (webpack, esbuild, etc.)

### Test Failures
If tests fail to resolve aliases:

1. Verify jest.config.js has the correct moduleNameMapper configuration
2. Ensure the mappings in jest.config.js match those in tsconfig.json

## Best Practices

1. **Consistency**: Use path aliases consistently throughout the codebase
2. **Organization**: Keep related modules together under their respective alias paths
3. **Documentation**: Document any new aliases added to the configuration
4. **Testing**: Always verify that aliases work in development, testing, and production environments

## Migration Note
Currently, the codebase uses a monorepo structure with workspace references (`@multi-controller/*` packages). The path aliases are configured for future use when transitioning to or adding traditional TypeScript modules that aren't separate packages.

## Related Files
- `/tsconfig.json` - Main TypeScript configuration with path mappings
- `/jest.config.js` - Jest configuration with moduleNameMapper
- `/nodemon.json` - Development server configuration with tsconfig-paths
- `/package.json` - Contains tsconfig-paths dependency

## Task Reference
This configuration was implemented as part of Task 19: Fix TypeScript Path Aliases Resolution and Ensure Runtime Compatibility.