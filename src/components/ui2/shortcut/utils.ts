
type ChainedShortcut = {
  type: "chained"
  keys: string[]
}
type CombinedShortcut = {
  type: "combined"
  ctrlKey: boolean
  shiftKey: boolean
  altKey: boolean
  key: string
}

export type ParsedShortcut =
  | ChainedShortcut
  | CombinedShortcut

const chainedShortcutRegex = /^[^-]+.*-.*[^-]+$/;
const combinedShortcutRegex = /^[^_]+.*_.*[^_]+$/;

export function parseShortcut(shortcut: string): ParsedShortcut | null {
  if (chainedShortcutRegex.test(shortcut)) {
    return {
      type: "chained",
      keys: shortcut.split("-"),
    };
  }
  if (combinedShortcutRegex.test(shortcut)) {
    const keySplit = shortcut.toLowerCase().split("_");
    return {
      type: "combined",
      key: keySplit.filter(k => !["ctrl", "shift", "alt"].includes(k)).join("_"),
      ctrlKey: keySplit.includes("ctrl"),
      shiftKey: keySplit.includes("shift"),
      altKey: keySplit.includes("alt"),
    };
  }
  console.warn(`Failed to parse shortcut: '${shortcut}'`);
  return null;
}
