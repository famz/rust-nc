// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! From arch/x86/include/asm/page_types.h

use super::{PMD_SHIFT, PUD_SHIFT, __PAGE_OFFSET, __PHYSICAL_MASK_SHIFT};

/// PAGE_SHIFT determines the page size
pub const PAGE_SHIFT: i32 = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

pub const PMD_PAGE_SIZE: usize = 1 << PMD_SHIFT;
pub const PMD_PAGE_MASK: usize = !(PMD_PAGE_SIZE - 1);

pub const PUD_PAGE_SIZE: usize = 1 << PUD_SHIFT;
pub const PUD_PAGE_MASK: usize = !(PUD_PAGE_SIZE - 1);

/// Cast *PAGE_MASK to a signed type so that it is sign-extended if
/// virtual addresses are 32-bits but physical addresses are larger
/// (ie, 32-bit PAE).
pub const PHYSICAL_PAGE_MASK: i64 = (PAGE_MASK as i64) & (__PHYSICAL_MASK as i64);
//pub const PHYSICAL_PMD_PAGE_MASK: i64 = PMD_PAGE_MASK as i64 & __PHYSICAL_MASK;
//pub const PHYSICAL_PUD_PAGE_MASK: i64 = PUD_PAGE_MASK as i64 & __PHYSICAL_MASK;

pub const HPAGE_SHIFT: i32 = PMD_SHIFT;
pub const HPAGE_SIZE: usize = 1 << HPAGE_SHIFT;
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
pub const HUGETLB_PAGE_ORDER: i32 = HPAGE_SHIFT - PAGE_SHIFT;

pub const HUGE_MAX_HSTATE: i32 = 2;

pub const PAGE_OFFSET: usize = __PAGE_OFFSET;

//pub const VM_DATA_DEFAULT_FLAGS: i32 = VM_DATA_FLAGS_TSK_EXEC;

//pub const __PHYSICAL_START: i32 = ALIGN;(CONFIG_PHYSICAL_START, CONFIG_PHYSICAL_ALIGN);

//pub const __START_KERNEL: isize = 		(__START_KERNEL_map + __PHYSICAL_START);

pub const IOREMAP_MAX_ORDER: i32 = PUD_SHIFT;

// TODO(Shaohua): Handle `#ifdef CONFIG_DYNAMIC_PHYSICAL_MASK`
pub const __PHYSICAL_MASK: usize = (1 << __PHYSICAL_MASK_SHIFT) - 1;
