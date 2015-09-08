use bindings::ffi;
use bindings::AsNative;
use random::Rng;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct BSP {
    bsp: *mut ffi::TCOD_bsp_t,
    root: bool
}

impl Deref for BSP {
    type Target = ffi::TCOD_bsp_t;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.bsp }
    }
}

impl DerefMut for BSP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.bsp }
    }
}

impl BSP {
    pub fn new_with_size(x: i32, y: i32, w: i32, h: i32) -> Self {
        let bsp = unsafe {
            ffi::TCOD_bsp_new_with_size(x, y, w, h)
        };
        BSP { bsp: bsp, root: true }
    }

    pub fn remove_sons(&self) {
        unsafe { ffi::TCOD_bsp_remove_sons(self.bsp) }
    }

    pub fn split_once(&self, horizontal: bool, position: i32) {
        unsafe { ffi::TCOD_bsp_split_once(self.bsp, horizontal as u8, position) }
    }

    pub fn split_recursive(&self,
                           randomizer: Rng,
                           nb: i32,
                           min_h_size: i32,
                           min_v_size: i32,
                           max_h_ratio: f32,
                           max_v_ratio: f32) {
        unsafe {
            ffi::TCOD_bsp_split_recursive(self.bsp,
                                          *randomizer.as_native(),
                                          nb,
                                          min_h_size,
                                          min_v_size,
                                          max_h_ratio,
                                          max_v_ratio)
        }
    }

    pub fn resize(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { ffi::TCOD_bsp_resize(self.bsp, x, y, w, h) }
    }

    pub fn left(&self) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_left(self.bsp) },
            root: false
        }
    }

    pub fn right(&self) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_right(self.bsp) },
            root:false
        }
    }

    pub fn father(&self) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_father(self.bsp) },
            root: false,
        }
    }

    pub fn is_leaf(&self) -> bool {
        unsafe { ffi::TCOD_bsp_is_leaf(self.bsp) != 0 }
    }

    pub fn contains(&self, cx: i32, cy: i32) -> bool {
        unsafe { ffi::TCOD_bsp_contains(self.bsp, cx, cy) != 0 }
    }

    pub fn find_node(&self, cx: i32, cy: i32) -> Self {
        BSP {
            bsp: unsafe { ffi::TCOD_bsp_find_node(self.bsp, cx, cy) },
            root: false
        }
    }

    pub fn horizontal(&self) -> bool {
        self.horizontal != 0
    }

    pub fn set_horizontal(&mut self, h: bool) {
        self.horizontal = h as u8;
    }
}

impl Drop for BSP {
    fn drop(&mut self) {
        if self.root {
            unsafe { ffi::TCOD_bsp_delete(self.bsp) }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BSP;

    #[test]
    #[allow(unused_variables)]
    fn created_destroyed_no_panic() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);
        let left = bsp.left(); // left has null .bsp
    }

    #[test]
    fn accessors() {
        let mut bsp = BSP::new_with_size(0, 0, 50, 60);

        assert_eq!(bsp.x, 0);
        assert_eq!(bsp.y, 0);
        assert_eq!(bsp.w, 50);
        assert_eq!(bsp.h, 60);
        assert_eq!(bsp.horizontal(), false);
        bsp.x = 10;
        bsp.y = 20;
        bsp.set_horizontal(true);
        assert_eq!(bsp.x, 10);
        assert_eq!(bsp.y, 20);
        assert_eq!(bsp.horizontal(), true);
    }

    #[test]
    fn split() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);

        assert_eq!(bsp.position, 0);
        assert_eq!(bsp.horizontal(), false);

        bsp.split_once(true, 20);
        assert_eq!(bsp.position, 20);
        assert_eq!(bsp.horizontal(), true);
    }

    #[test]
    fn children() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);

        assert!(bsp.left().bsp.is_null());
        assert_eq!(bsp.level, 0);

        bsp.split_once(false, 20);
        assert!(!bsp.left().bsp.is_null());
        assert!(!bsp.right().bsp.is_null());
        assert_eq!(bsp.left().level, 1);
        assert_eq!(bsp.right().level, 1);
    }

    #[test]
    fn father() {
        let bsp = BSP::new_with_size(0, 0, 50, 50);
        assert!(bsp.father().bsp.is_null());

        bsp.split_once(false, 30);
        assert!(!bsp.left().father().bsp.is_null());
        assert!(!bsp.right().father().bsp.is_null());
    }
}
