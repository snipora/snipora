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
