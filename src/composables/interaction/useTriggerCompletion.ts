import { computed, ref, readonly, type ComputedRef, type Ref } from "vue";
import { useEventListener } from "@vueuse/core";

export interface UseTriggerCompletionOptions {
	triggers: string[];
	input: Ref<HTMLInputElement | null>;
}

export function useTriggerCompletion(options: UseTriggerCompletionOptions) {
	const { triggers, input: inputRef } = options;

	const currentTrigger = ref<string | null>(null);
	const searchText = ref<string>("");
	const isCompleting: ComputedRef<boolean> = computed(() => currentTrigger.value !== null);

	function getSelectionStart(): number {
		const inputEl = inputRef.value;
		if (!inputEl) return -1;
		return inputEl.selectionStart ?? 0;
	}

	function getValue(): string {
		const inputEl = inputRef.value;
		if (!inputEl) return "";
		return inputEl.value;
	}

	function getLastTriggerBeforeCaret(caretIndex: number) {
		const value = getValue();
		return triggers
			.map((trigger) => ({ trigger, triggerIndex: value.lastIndexOf(trigger, caretIndex - 1) }))
			.sort((a, b) => b.triggerIndex - a.triggerIndex)[0];
	}

	function getLastSearchText(
		caretIndex: number,
		keyIndex: number,
	): string | null {
		if (keyIndex === -1) return null;
		const text = getValue().substring(keyIndex + 1, caretIndex);
		return /\s/.test(text) ? null : text;
	}

	function refresh(): boolean {
		const index = getSelectionStart();
		if (index < 0) {
			deactivate();
			return false;
		}

		const { trigger, triggerIndex } = getLastTriggerBeforeCaret(index);
		const text = getLastSearchText(index, triggerIndex);

		if (text != null && (triggerIndex < 1 || /\s/.test(getValue()[triggerIndex - 1]))) {
			activate(trigger);
			searchText.value = text;
			return true;
		}

		deactivate();
		return false;
	}

	function activate(trigger: string) {
		currentTrigger.value = trigger;
	}

	function deactivate() {
		currentTrigger.value = null;
		searchText.value = "";
	}

	function replace(replacement: string) {
		const inputEl = inputRef.value;
		if (!inputEl) return;

		const cursor = inputEl.selectionStart ?? 0;
		const { triggerIndex } = getLastTriggerBeforeCaret(cursor);
		if (triggerIndex < 0) return;

		const before = inputEl.value.slice(0, triggerIndex);
		const after = inputEl.value.slice(cursor);
		inputEl.value = before + replacement + after;

		const newCursor = triggerIndex + replacement.length;
		inputEl.selectionStart = inputEl.selectionEnd = newCursor;

		inputEl.dispatchEvent(new Event("input", { bubbles: true }));
		deactivate();
	}

	useEventListener(inputRef, "input", () => {
		refresh();
	});
	useEventListener(inputRef, "keydown", (e: KeyboardEvent) => {
		if (currentTrigger.value && e.key === "Escape") {
			e.stopPropagation();
			deactivate();
		}
	});
	useEventListener(inputRef, "blur", () => {
		deactivate();
	});

	return {
		currentTrigger: readonly(currentTrigger),
		searchText: readonly(searchText),
		isCompleting,
		replace,
	};
}
