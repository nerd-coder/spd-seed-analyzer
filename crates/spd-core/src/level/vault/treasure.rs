//! Treasure-room `paint()` dispatch. Split by tier for SMALL-FILES.

mod t1;
mod t2;
mod t3;

pub(in crate::level::vault) use t1::{paint_circle_scan, paint_flame_path, paint_laser};
pub(in crate::level::vault) use t2::{paint_bookcase, paint_flames, paint_single_enemy};
pub(in crate::level::vault) use t3::{paint_hard_laser, paint_many_scans, paint_multiple_enemy};
