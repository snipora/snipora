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
    snippetUsageBehavior: "copy-to-clipboard" | "simulate-paste"
  }

  shortcuts: {
    openPopup: string
  }

  appearance: {
    showTagCounts: boolean
    uiTheme: "system" | "light" | "dark"
    trayIconTheme: "app" | "light" | "dark"
  }
}
