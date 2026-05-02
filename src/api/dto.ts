export type RuntimeInfo = {
  os: "windows" | "macos" | "linux" | (string & {})
  arch: "x86" | "x86_64" | "arm" | "aarch64" | (string & {})
  bundleType: "deb" | "rpm" | "appimage" | "msi" | "nsis" | "app" | "dmg" | "unknown"
}

export type Tag = string

export type SnippetDto = {
  id: string
  label: string
  snippet: string
  tags: Tag[]
  created_at: number
  updated_at: number
  last_used_at: number | null
}

export type LocalSettingsDto = {
  general: {
    locale: string
  }

  shortcuts: {
    openPopup: string
  }

  popup: {
    snippetUsageBehavior: "copy-to-clipboard" | "simulate-paste"
  }

  ui: {
    showTagCounts: boolean
    theme: "system" | "light" | "dark"
  }

  tray: {
    iconTheme: "app" | "light" | "dark"
  }
}
