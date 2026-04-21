import {computed, Ref, ref} from "vue";
import {LocalSettingsDto} from "@/api/dto.ts";
import {invokeFetchLocalSettings, invokeUpdateLocalSettings} from "@/api/commands";
import {useThrottleFn} from "@vueuse/core";
import {listen} from "@tauri-apps/api/event";


const THROTTLE_DEBOUNCE_MS = 300;

const localSettings = ref<LocalSettingsDto>(null as unknown as LocalSettingsDto);

export async function _initLocalSettings() {
  localSettings.value = await invokeFetchLocalSettings();

  await listen<LocalSettingsDto>("local-settings-changed", event => {
    localSettings.value = event.payload;
  });
}


export function useLocalSettings<K extends DotEndPaths<LocalSettingsDto>>(setting: K): Ref<PathValue<LocalSettingsDto, K>> {
  if (localSettings.value === null) throw new Error("localSettings is not initialized");

  const pathKeys = setting.split(/\./g);

  const scheduleUpdate = useThrottleFn(async () => {
    try {
      await invokeUpdateLocalSettings(toPatch(localSettings.value, pathKeys));
    } catch (error) {
      localSettings.value = await invokeFetchLocalSettings();  // reset optimistic update
      throw error;
    }
  }, THROTTLE_DEBOUNCE_MS, true);

  return computed({
    get: () => {
      return ofPath(localSettings.value, pathKeys) as PathValue<LocalSettingsDto, K>;
    },
    set: async (val: PathValue<LocalSettingsDto, K>) => {
      setByPath(localSettings.value, pathKeys, val);  // optimistic update
      scheduleUpdate();  // sync to backend
    },
  });
}


function ofPath(obj: Record<string, unknown>, pathKeys: string[]): unknown {
  return pathKeys.reduce((o, p) => o[p] as Record<string, unknown>, obj);
}

function setByPath(obj: Record<string, unknown>, pathKeys: string[], value: unknown): void {
  const lastObj = ofPath(obj, pathKeys.slice(0, -1)) as Record<string, unknown>;
  lastObj[pathKeys![pathKeys!.length - 1]] = value;
}

function toPatch(object: Record<string, unknown>, pathKeys: string[]): Record<string, unknown> {
  let patch: Record<string, unknown> = {};
  let currentPatch: Record<string, unknown> = patch;
  let currentObject: Record<string, unknown> = object;

  for (const key of pathKeys.slice(0, -1)) {
    currentPatch[key] = {};
    currentPatch = patch[key] as Record<string, unknown>;
    currentObject = currentObject[key] as Record<string, unknown>;
  }

  const propertyKey = pathKeys[pathKeys.length - 1]
  currentPatch[propertyKey] = currentObject[propertyKey] as unknown;

  return patch;
}
