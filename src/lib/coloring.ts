function hashString(str: string): number {
  let hash = 0;

  for (let i = 0; i < str.length; i++) {
    hash = str.charCodeAt(i) + ((hash << 5) - hash);
    hash |= 0;  // Convert to integer
  }

  return hash;
}


export function stringToColor(text: string): string {
  const hash = hashString(text);

  // Hue: full 360 range
  const hue = Math.abs(hash) % 360;

  // Saturation: avoid washed-out colors
  const saturation = 60 + (Math.abs(hash) % 20);  // 60-80%

  const lightness = 45 + (Math.abs(hash) % 10);  // 45-55%

  return `hsl(${hue}, ${saturation}%, ${lightness}%)`;
}
