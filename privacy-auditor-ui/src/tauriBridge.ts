import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type EventCallback<T> = (event: { payload: T }) => void;

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

export function isTauriRuntime() {
  return typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__);
}

export async function invokeIfAvailable<T>(
  command: string,
  args?: Record<string, unknown>,
) {
  if (!isTauriRuntime()) {
    throw new Error("Tauri runtime is not available");
  }

  return invoke<T>(command, args);
}

export async function listenIfAvailable<T>(
  eventName: string,
  callback: EventCallback<T>,
) {
  if (!isTauriRuntime()) {
    return () => {};
  }

  return listen<T>(eventName, callback);
}
