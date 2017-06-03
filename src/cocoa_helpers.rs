use cocoa::base::{selector, nil, NO, YES, class};
use cocoa::foundation::{NSRect};
use cocoa::appkit::{NSWindow,
                    NSTitledWindowMask, NSBackingStoreBuffered,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps,
                    NSBorderlessWindowMask, NSWindowAbove};

use std::ops::Deref;

use objc::runtime;
#[allow(non_camel_case_types)]
pub type id = *mut runtime::Object;


pub struct CocoaWindow {
    view: IdRef,
    child_view: IdRef,
}

// impl CocoaWindow {
//     pub fn from_parent(parent_id: id) -> Self {
//         let parent = IdRef(parent_id);
//         CocoaWindow{ view: parent, child_view: view }
//     }
// }


// pub unsafe fn add_button(view: id) -> IdRef {
//     use cocoa::appkit::NSButton;

//     let child_button = IdRef(
//         NSButton::alloc(nil).initWithFrame_(host_window_frame(view))
//     );

//     add_subview(view, child_button.0);
//     child_button
// }

pub unsafe fn add_child_view(view: id) -> id {
    use cocoa::appkit::NSView;

    let child_nsview = IdRef::new(NSView::alloc(nil));
    let child_view = child_nsview.initWithFrame_(host_window_frame(view));

    add_subview(view, child_view);
    child_view
}

/// @property NSRect frame;
pub unsafe fn host_window_frame(view: id) -> NSRect {
    use cocoa::appkit::NSView;
    NSView::frame(view)
}

/// - (void)addSubview:(UIView *)view;
pub unsafe fn add_subview(parent_id: id, child_id: id) {
    msg_send![parent_id, addSubview:child_id];
}

/// - (void)addChildWindow:(NSWindow *)childWin 
///             ordered:(NSWindowOrderingMode)place;
pub unsafe fn add_child_window(parent_id: id, child_id: id) {
    msg_send![parent_id, addChildWindow:child_id ordered: NSWindowAbove];
}

pub unsafe fn class_name(object: id) -> String {
    msg_send![object, class]
}

// pub unsafe fn stats(object: id) {
//     info!("stats");
//     info!("class: {}", class_name(object));
// }

pub struct IdRef(id);

impl IdRef {
    fn new(i: id) -> IdRef {
        IdRef(i)
    }

    #[allow(dead_code)]
    fn retain(i: id) -> IdRef {
        if i != nil {
            let _: id = unsafe { msg_send![i, retain] };
        }
        IdRef(i)
    }

    fn non_nil(self) -> Option<IdRef> {
        if self.0 == nil { None } else { Some(self) }
    }
}

impl Drop for IdRef {
    fn drop(&mut self) {
        if self.0 != nil {
            let _: () = unsafe { msg_send![self.0, release] };
        }
    }
}

impl Deref for IdRef {
    type Target = id;
    fn deref<'a>(&'a self) -> &'a id {
        &self.0
    }
}

impl Clone for IdRef {
    fn clone(&self) -> IdRef {
        if self.0 != nil {
            let _: id = unsafe { msg_send![self.0, retain] };
        }
        IdRef(self.0)
    }
}