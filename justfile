[private]
@default:
    just --list

# Start the app in development mode
[group: 'development']
dev:
    npx tauri dev

[private]
[no-exit-message]
[group: 'development']
frontend-dev:
    npx vite dev

[private]
[group: 'build']
frontend-build:
    npx vite build

# Build the app for production
[group: 'build']
build:
    npx tauri build

# Build the app without installer bundles
[group: 'build']
build-no-bundle:
    npx tauri build --no-bundle

# Check frontend for type errors
[group: 'checks']
type-check-frontend:
    npx vue-tsc --noEmit

# Check backend for type errors
[group: 'checks']
type-check-backend:
    cargo check --manifest-path src-tauri/Cargo.toml

# Check for missing or unused translation keys
[group: 'checks']
i18n-check:
    npx vue-i18n-extract --ci --vueFiles 'src/**/*.?(ts|vue)' --languageFiles 'src/locales/*.yaml'

# Generate app icons from source image
[group: 'tools']
generate-icons:
    node scripts/generate-icons.ts
