use pin_project::{pin_project, pinned_drop};
use std::pin::Pin;

#[pin_project(PinnedDrop, project = EnumProj, project_ref = EnumProjRef)]
enum Enum<T, U> {
    Struct {
        #[pin]
        pinned: T,
        unpinned: U,
    },
    Tuple(#[pin] T, U),
    Unit,
}

#[pinned_drop]
impl<T, U> PinnedDrop for Enum<T, U> {
    fn drop(self: Pin<&mut Self>) {
        let _this = self;
    }
}

fn main() {}
