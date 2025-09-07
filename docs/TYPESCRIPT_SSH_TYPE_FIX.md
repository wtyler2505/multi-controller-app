# TypeScript SSH Transport Type Safety Fix

## Issue
The SSH transport module (`transports/ssh/src/index.ts`) contained two instances of `any` types that needed to be replaced with proper TypeScript interfaces for type safety.

## Changes Made

### 1. Stream Type (Line 12)
**Before:**
```typescript
private stream?: any;
```

**After:**
```typescript
private stream?: ClientChannel;
```

The `stream` property is now properly typed as `ClientChannel` from the `ssh2` package, which represents the SSH channel used for communication.

### 2. Connect Config Type (Line 66)
**Before:**
```typescript
const connectConfig: any = {
  host: this.host,
  port: this.port,
  username: this.username
};
```

**After:**
```typescript
const connectConfig: ConnectConfig = {
  host: this.host,
  port: this.port,
  username: this.username
};
```

The connection configuration now uses the proper `ConnectConfig` interface from `ssh2`, ensuring type safety for SSH connection parameters.

## Benefits
- **Type Safety**: TypeScript can now properly validate SSH-related code at compile time
- **IntelliSense**: IDEs provide better autocomplete and documentation for SSH types
- **Maintenance**: Future developers can understand the expected structure of these objects
- **Error Prevention**: Catches type-related bugs during development rather than runtime

## Testing
Once dependencies are installed, run:
```bash
cd transports/ssh
npm run typecheck
```

This ensures the TypeScript compiler validates all types correctly with strict mode enabled.