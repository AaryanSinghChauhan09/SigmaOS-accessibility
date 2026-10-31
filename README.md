# SigmaOS-accessibility

Accessibility framework, screen-reader voice registry, magnifier, and
keyboard support for SigmaOS.

## Screen reader status

The current `SimpleScreenReader` only stores voices and playback state; it is
not connected to a speech engine. `speak` returns `BackendUnavailable` for a
registered voice instead of reporting false success. Braille display updates
clear stale cells when shorter content replaces longer content. Speech
Dispatcher, espeak-ng, and BrlAPI integration remain unimplemented.

## Development

```bash
cargo check
```

Accessibility behavior is a release requirement; document keyboard, contrast,
and assistive-technology evidence for changes.
