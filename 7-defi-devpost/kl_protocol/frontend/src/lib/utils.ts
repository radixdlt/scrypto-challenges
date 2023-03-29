export function truncate(text: string | undefined, totalChars = 10, endChars = 6) {

  let _text = text ?? ''
  endChars = Math.min(endChars, totalChars);
  const start = _text.slice(0, totalChars - endChars);
  const end = endChars > 0 ? _text.slice(-endChars) : '';

  if (start.length + end.length < _text.length) {
    return start + '…' + end;
  } else {
    return _text;
  }
}

export function clipboard(node: any, { trigger = "click", text = "" } = {}) {
  const handle = async (e: any) => {
    await navigator.clipboard.writeText(text).then(
      () =>
        node.dispatchEvent(
          new CustomEvent("copied", { detail: { clipboard: text } })
        ),
      (e) =>
        node.dispatchEvent(new CustomEvent("error", { detail: { error: e } }))
    );
  };

  node.addEventListener(trigger, handle, true);

  return {
    update: (params: any) => {
      if (params.trigger !== undefined) trigger = params.trigger;

      if (params.text !== undefined) text = params.text;
    },
    destroy() {
      node.removeEventListener(trigger, handle, true);
    },
  };
}

export function format_number(input: number, notation: "standard" | "scientific" | "engineering" | "compact" | undefined = undefined, default_value = '') {

  if (input === undefined) return default_value

  return input.toLocaleString('en', {
    notation: notation,
    compactDisplay: 'short'
  })

}