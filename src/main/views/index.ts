import {Component, InjectionKey, Ref} from "vue";
import {ViewAllSnippets} from "@/main/views/all-snippets";
import {ViewUntagged} from "@/main/views/untagged";
import {ViewByTag} from "@/main/views/by-tag";
import {ViewSettings} from "@/main/views/settings";
import {ViewSnippetById} from "@/main/views/snippet-by-id";

export type ViewState =
  | { id: "all-snippets" }
  | { id: "untagged-snippets" }
  | { id: "snippets-by-tag", tag: string }
  | { id: "settings" }
  | { id: "snippet-by-id", snippetId: string }
export type ViewIds = ViewState["id"];

export const VIEW_TO_COMPONENT: Record<ViewIds, Component> = {
  "all-snippets": ViewAllSnippets,
  "untagged-snippets": ViewUntagged,
  "snippets-by-tag": ViewByTag,
  "settings": ViewSettings,
  "snippet-by-id": ViewSnippetById,
};

export const INJECTION_KEY_MAIN_VIEW: InjectionKey<Ref<ViewState>> = Symbol("view-state");
