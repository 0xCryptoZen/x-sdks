# TypeScript SDK Architecture Guidance - Issue #1

**Agent:** typescript-engineer
**Date:** 2025-11-04
**Issue:** #1 - X (Twitter) SDK for TypeScript

---

## 🎯 Architecture Overview

The TypeScript SDK should follow a modular, type-safe architecture:

```
┌─────────────────────────────────┐
│   Public API Layer              │  (XClient class)
├─────────────────────────────────┤
│   Authentication Layer          │  (OAuth 1.0a signing)
├─────────────────────────────────┤
│   HTTP Transport Layer          │  (axios/fetch)
├─────────────────────────────────┤
│   Type System Layer             │  (TypeScript interfaces)
└─────────────────────────────────┘
```

---

## 📦 Project Structure

Recommend monorepo structure using modern TypeScript practices:

```
typescript-sdk/
├── package.json
├── tsconfig.json
├── tsconfig.build.json
├── .npmignore
├── src/
│   ├── index.ts           # Public exports
│   ├── client.ts          # XClient class
│   ├── auth/
│   │   ├── index.ts
│   │   ├── oauth.ts       # OAuth 1.0a implementation
│   │   └── types.ts       # Auth types
│   ├── endpoints/
│   │   ├── index.ts
│   │   └── tweets.ts      # Tweet endpoints
│   ├── types/
│   │   ├── index.ts
│   │   ├── request.ts     # Request types
│   │   ├── response.ts    # Response types
│   │   └── error.ts       # Error types
│   └── utils/
│       ├── index.ts
│       └── http.ts        # HTTP utilities
├── tests/
│   ├── unit/
│   │   ├── auth.test.ts
│   │   ├── client.test.ts
│   │   └── types.test.ts
│   ├── integration/
│   │   └── tweets.test.ts
│   └── fixtures/
│       └── responses.json
├── examples/
│   ├── post-tweet.ts
│   └── with-error-handling.ts
└── dist/                   # Compiled output
    ├── index.js
    ├── index.d.ts
    └── ...
```

---

## 🔧 TypeScript Configuration

**Strict Mode Recommendations:**

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true
  }
}
```

---

## 🔑 Core Components

### 1. Client Design Pattern

**Recommendation:** Class-based with configuration options

**Key Features:**
- Constructor with credentials
- Fluent API for configuration
- Immutable after construction
- Support for custom HTTP clients

**Design Principles:**
- Single responsibility
- Open for extension
- Dependency injection support
- Easy to mock for testing

---

### 2. Authentication Strategy

**OAuth 1.0a Implementation:**

**Option A: Use existing library (RECOMMENDED)**
- Library: `oauth-1.0a` or `twitter-api-v2`
- Pros: Well-tested, maintained, TypeScript support
- Cons: External dependency

**Option B: Custom implementation**
- Pros: Zero dependencies, full control
- Cons: Complex, error-prone
- **NOT RECOMMENDED** unless necessary

**Implementation Components:**
1. Nonce generator (crypto.randomBytes)
2. Timestamp (Unix seconds)
3. Parameter encoding (RFC 3986)
4. Signature base string
5. HMAC-SHA1 signing (crypto module)
6. Authorization header builder

---

### 3. Type System Design

**Core Principles:**
- Strict null checks enabled
- No `any` types (use `unknown` when needed)
- Discriminated unions for variants
- Readonly where appropriate

**Request Types:**
```typescript
interface TweetRequest {
  text: string;
  media?: {
    media_ids?: string[];
    tagged_user_ids?: string[];
  };
  reply?: {
    in_reply_to_tweet_id: string;
    exclude_reply_user_ids?: string[];
  };
  poll?: {
    options: string[];
    duration_minutes: number;
  };
}
```

**Response Types:**
```typescript
interface TweetResponse {
  data: {
    id: string;
    text: string;
    edit_history_tweet_ids?: string[];
  };
  meta?: {
    sent: string;
  };
}
```

**Error Types:**
```typescript
enum XErrorCode {
  AUTHENTICATION_FAILED = 'AUTHENTICATION_FAILED',
  RATE_LIMIT_EXCEEDED = 'RATE_LIMIT_EXCEEDED',
  INVALID_REQUEST = 'INVALID_REQUEST',
  NETWORK_ERROR = 'NETWORK_ERROR',
  API_ERROR = 'API_ERROR',
  UNKNOWN_ERROR = 'UNKNOWN_ERROR'
}

interface XErrorDetails {
  code: XErrorCode;
  message: string;
  statusCode?: number;
  apiError?: {
    title?: string;
    detail?: string;
    type?: string;
  };
  retryAfter?: number;
}
```

---

### 4. HTTP Layer Design

**HTTP Client Choice:**

**Option A: axios (RECOMMENDED)**
- Pros: Feature-rich, interceptors, timeout support
- Cons: Larger bundle size
- Best for: Node.js applications

**Option B: node-fetch**
- Pros: Lightweight, native-like API
- Cons: Fewer built-in features
- Best for: Minimal dependencies

**Request Configuration:**
- Base URL configuration
- Timeout settings (30s default)
- Retry logic with exponential backoff
- User-Agent header
- Accept: application/json
- Content-Type: application/json

**Request Flow:**
1. Validate request object
2. Serialize to JSON
3. Generate OAuth header
4. Configure HTTP request
5. Send request
6. Handle HTTP status
7. Parse response JSON
8. Map to typed response
9. Handle errors with context

---

## 🛡️ Error Handling Strategy

**Error Class Hierarchy:**

```typescript
class XError extends Error {
  constructor(
    public code: XErrorCode,
    public details: XErrorDetails,
    public cause?: Error
  ) {
    super(details.message);
    this.name = 'XError';
    Error.captureStackTrace(this, this.constructor);
  }

  isRetryable(): boolean {
    return [
      XErrorCode.RATE_LIMIT_EXCEEDED,
      XErrorCode.NETWORK_ERROR
    ].includes(this.code);
  }

  getRetryDelay(): number | null {
    return this.details.retryAfter ?? null;
  }
}
```

**Error Categories:**
1. Authentication Errors (401, 403)
2. Validation Errors (400, 422)
3. Rate Limiting (429)
4. Server Errors (5xx)
5. Network Errors
6. Parsing Errors

---

## 🔒 Security Considerations

1. **Credential Management:**
   - Never log credentials
   - Use environment variables
   - Support credential providers
   - No defaults or hardcoded values

2. **Input Validation:**
   - Validate before sending requests
   - Sanitize user input
   - Check length constraints
   - Prevent injection attacks

3. **HTTPS Enforcement:**
   - Only HTTPS endpoints
   - Certificate validation
   - Reject self-signed certs (production)

4. **OAuth Security:**
   - Cryptographically secure nonce
   - Time-based replay protection
   - Proper signature encoding

---

## ⚡ Performance Considerations

1. **HTTP Connection Reuse:**
   - Single axios instance per client
   - Connection pooling
   - Keep-alive enabled

2. **Memory Management:**
   - Avoid memory leaks
   - Clean up resources
   - Stream large responses

3. **Bundle Size:**
   - Tree-shakeable exports
   - No unnecessary dependencies
   - Minimal footprint

4. **Async Operations:**
   - Use async/await
   - Promise-based API
   - No callback hell
   - Proper error propagation

---

## 🧪 Testing Strategy

**Testing Framework:** Jest

**Test Types:**

1. **Unit Tests:**
   - OAuth signature generation
   - Type validation
   - Error handling
   - Utility functions

2. **Integration Tests:**
   - Mock HTTP responses (nock or msw)
   - Full request/response cycle
   - Error scenarios
   - Edge cases

3. **Type Tests:**
   - Use `@typescript-eslint` for type checking
   - Test type inference
   - Test type compatibility

**Code Coverage Target:** >85%

**Testing Utilities:**
```typescript
// Mock factory
function createMockClient(overrides?: Partial<XClientConfig>): XClient

// Mock responses
function mockTweetResponse(overrides?: Partial<TweetResponse>): TweetResponse

// Mock errors
function mockApiError(statusCode: number): XError
```

---

## 📚 Dependencies Recommendation

**Production Dependencies:**
```json
{
  "dependencies": {
    "axios": "^1.6.0",
    "oauth-1.0a": "^2.2.6"
  }
}
```

**Development Dependencies:**
```json
{
  "devDependencies": {
    "typescript": "^5.3.0",
    "@types/node": "^20.0.0",
    "jest": "^29.7.0",
    "@types/jest": "^29.5.0",
    "ts-jest": "^29.1.0",
    "nock": "^13.4.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "eslint": "^8.55.0",
    "prettier": "^3.1.0"
  }
}
```

---

## 🚀 Implementation Phases

### Phase 1: Foundation (Priority: High)
- Project scaffolding
- TypeScript configuration
- Core type definitions
- Error class hierarchy

### Phase 2: Authentication (Priority: High)
- OAuth 1.0a implementation
- Credential configuration
- Signature generation
- Authorization headers

### Phase 3: HTTP Client (Priority: High)
- Axios configuration
- Request builder
- Response parser
- Error mapper

### Phase 4: Tweet Endpoint (Priority: High)
- POST /2/tweets implementation
- Request validation
- Response typing
- Error handling

### Phase 5: Testing (Priority: High)
- Unit tests
- Integration tests
- Example code
- Documentation

### Phase 6: Package Configuration (Priority: High)
- Build configuration
- NPM package setup
- Documentation
- README

### Phase 7: Future Extensions (Priority: Low)
- Media upload
- Rate limit handling
- More endpoints
- Retry mechanisms

---

## 💡 Best Practices

1. **Code Style:**
   - Use ESLint + Prettier
   - Follow Airbnb or Standard style guide
   - Consistent naming conventions
   - Clear file organization

2. **Documentation:**
   - JSDoc for all public APIs
   - README with quick start
   - API reference
   - Usage examples
   - Migration guides

3. **Versioning:**
   - Follow SemVer
   - Changelog for all releases
   - Deprecation warnings
   - Breaking change notices

4. **Package Publishing:**
   - Compiled JavaScript + TypeScript definitions
   - Source maps for debugging
   - Tree-shakeable exports
   - Multiple module formats (CJS, ESM)

---

## ⚠️ Common Pitfalls to Avoid

1. **Type Safety:**
   - Don't use `any`
   - Don't disable strict mode
   - Don't ignore TypeScript errors
   - Don't use type assertions carelessly

2. **OAuth Implementation:**
   - Wrong parameter encoding
   - Incorrect timestamp format
   - Missing parameter sorting
   - Case-sensitive issues

3. **Error Handling:**
   - Swallowing errors
   - Generic error messages
   - Missing stack traces
   - Not providing retry information

4. **Async Operations:**
   - Unhandled promise rejections
   - Race conditions
   - Memory leaks in event handlers
   - Not canceling pending requests

5. **Testing:**
   - Not mocking external HTTP calls
   - Hardcoded credentials in tests
   - Missing edge case tests
   - Not testing error paths

---

## 📋 Package.json Configuration

**Essential Fields:**
```json
{
  "name": "@x-sdks/typescript",
  "version": "1.0.0",
  "description": "TypeScript SDK for Twitter/X API",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "files": ["dist"],
  "scripts": {
    "build": "tsc -p tsconfig.build.json",
    "test": "jest",
    "test:coverage": "jest --coverage",
    "lint": "eslint src --ext .ts",
    "format": "prettier --write \"src/**/*.ts\"",
    "prepublishOnly": "npm run build"
  },
  "keywords": ["twitter", "x", "api", "sdk", "oauth"],
  "author": "",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/0xCryptoZen/x-sdks"
  }
}
```

---

## 🎓 Reference Materials

- [Twitter API v2 Documentation](https://developer.twitter.com/en/docs/twitter-api)
- [OAuth 1.0a Specification](https://oauth.net/core/1.0a/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/)
- [TypeScript Do's and Don'ts](https://www.typescriptlang.org/docs/handbook/declaration-files/do-s-and-don-ts.html)
- [axios Documentation](https://axios-http.com/)

---

## 🔄 ESM vs CJS Considerations

**Dual Package Support:**

For maximum compatibility, support both CommonJS and ES Modules:

1. **Build for both targets:**
   - CJS: `dist/cjs/`
   - ESM: `dist/esm/`

2. **Package.json configuration:**
```json
{
  "main": "./dist/cjs/index.js",
  "module": "./dist/esm/index.js",
  "types": "./dist/types/index.d.ts",
  "exports": {
    ".": {
      "require": "./dist/cjs/index.js",
      "import": "./dist/esm/index.js",
      "types": "./dist/types/index.d.ts"
    }
  }
}
```

---

**Next Steps:**
1. Initialize npm project
2. Configure TypeScript
3. Set up testing framework
4. Implement core types
5. Implement OAuth layer
6. Implement tweet endpoint
7. Add comprehensive tests
8. Write documentation

**Remember:** This is architectural guidance only. You will implement the actual TypeScript code based on these recommendations.
