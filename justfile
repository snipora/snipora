[private]
@default:
    just --list

# Start the app in development mode
[group: 'development']
dev:
    npx tauri dev

# Build the app without installer bundles
[group: 'build']
build:
    npx tauri build --no-bundle --no-sign

# Build the app with specific bundler
[group: 'build']
build-bundle bundle:
    npx tauri build --bundles "{{bundle}}" --no-sign

# Check frontend and backend for type errors
[group: 'checks']
type-check: type-check-frontend type-check-backend

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
    npx tauri icon assets/snipora.svg
    node scripts/generate-tray-icons.ts

[group: 'tools']
new-migration +DESCRIPTION:
    touch "src-tauri/migrations/`date '+%Y%m%d%H%M%S'`_{{snakecase(lowercase(DESCRIPTION))}}.sql"
