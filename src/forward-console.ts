import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
import {getCurrentWebviewWindow} from "@tauri-apps/api/webviewWindow";

const windowLabel = getCurrentWebviewWindow().label;

function stringify(value: unknown): string {
  if (typeof value === "string") {
    return value;
  } else if (value instanceof Error) {
    return `${value.name}: ${value.message}`;
  } else {
    return JSON.stringify(value);
  }
}

function forwardConsole(
    fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
    logger: (message: string) => Promise<void>
) {
  const browserLogger = console[fnName];
  console[fnName] = (...args) => {
    browserLogger(...args);
    logger(`[${windowLabel}]` + args.map(stringify).join(" "));
  };
}

forwardConsole('log', trace);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

window.addEventListener("error", (event) => {
  console.error("Unhandled error:", {
    message: event.message,
    file: event.filename,
    line: event.lineno,
    column: event.colno,
    error: event.error,
  });
});
window.addEventListener("unhandledrejection", (event) => {
  console.error("Unhandled rejection:", event.reason);
});
