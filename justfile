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
check-types: check-types-frontend check-types-backend

# Check frontend for type errors
[group: 'checks']
check-types-frontend:
    npx vue-tsc --noEmit

# Check backend for type errors
[group: 'checks']
check-types-backend:
    cargo check --manifest-path src-tauri/Cargo.toml

# Check for missing or unused translation keys
[group: 'checks']
check-i18n:
    npx vue-i18n-extract --ci --vueFiles 'src/**/*.?(ts|vue)' --languageFiles 'src/locales/*.yaml'

# Generate app icons from source image
[group: 'tools']
generate-icons:
    npx tauri icon assets/snipora.svg
    node scripts/generate-tray-icons.ts

[group: 'tools']
new-db-migration +DESCRIPTION:
    touch "src-tauri/migrations/`date '+%Y%m%d%H%M%S'`_{{snakecase(lowercase(DESCRIPTION))}}.sql"
