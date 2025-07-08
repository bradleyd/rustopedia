spin_loop_hint in std::sync::atomic - Rust
std
::
sync
::
atomic
Function
spin_loop_hint
Copy item path
1.24.0
·
Source
pub fn spin_loop_hint()
👎
Deprecated since 1.51.0: use hint::spin_loop instead
Expand description
Signals the processor that it is inside a busy-wait spin-loop (“spin lock”).
This function is deprecated in favor of
hint::spin_loop
.