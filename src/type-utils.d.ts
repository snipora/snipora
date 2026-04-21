type DotEndPaths<T, Prev extends string = ""> =
    T extends object
        ? {
          [K in keyof T & string]:
          T[K] extends readonly any[]
              ? `${Prev}${K}`
              : T[K] extends object
                  ? DotEndPaths<T[K], `${Prev}${K}.`>
                  : `${Prev}${K}`
        }[keyof T & string]
        : never

type PathValue<T, P extends string> =
  P extends `${infer K}.${infer Rest}`
      ? K extends keyof T
        ? PathValue<T[K], Rest>
        : never
      : P extends keyof T
        ? T[P]
        : never

type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P]
} & {}
