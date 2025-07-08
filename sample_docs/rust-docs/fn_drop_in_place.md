drop_in_place in std::intrinsics - Rust
std
::
intrinsics
Function
drop_in_place
Copy item path
1.8.0
·
Source
pub unsafe fn drop_in_place<T>(to_drop:
*mut T
)
where
    T: ?
Sized
,
👎
Deprecated since 1.52.0: no longer an intrinsic - use
ptr::drop_in_place
directly