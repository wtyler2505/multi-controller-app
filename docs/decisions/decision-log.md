# Decision Log

This log records architectural and technical decisions made during the development of the Windows Multi‑Controller App. Each entry follows an ADR‑style format: Title, Date, Status, Context, Decision and Consequences. Subsequent entries will be appended chronologically. See [architecture.md](../architecture/architecture.md) and [PRD.md](../../PRD.md) for more context.

## 2025‑08‑23 – Evaluate Programming Language & UI Framework

- **Status:** Proposed
- **Context:** The project requires a lightweight GUI on Windows with a single portable executable, fast start‑up and low memory usage. Two candidate technology stacks have been identified: C# with WPF/WinUI 3 compiled using .NET 8 Native AOT, and Rust with a minimal Win32/Rust UI library.
- **Decision:** Build and measure minimal prototypes in both stacks. The C# prototype will use WPF with the `<PublishAot>true</PublishAot>` and `<PublishTrimmed>true</PublishTrimmed>` properties to enable Native AOT. The Rust prototype will use a lightweight GUI (e.g., egui) compiled in release mode. Measurements will include application size, start‑up time, idle CPU and memory usage. The final selection will prioritise meeting the performance budgets and developer productivity.
- **Consequences:** Additional initial effort is required to implement two prototypes and collect metrics, but this de‑risks the choice of stack. Should the Rust prototype significantly outperform the C# version, the team may opt for Rust despite the steeper learning curve. The decision will be revisited after the prototypes are analysed.
