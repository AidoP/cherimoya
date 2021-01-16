use core::ops::{Deref, DerefMut};

/// Get a pointer to the level4 page table
fn page_table() -> *mut page::Table<page::Level4Entry> {
    let page_table;
    unsafe {
        asm! {
            "mov rax, cr3",
            lateout("rax") page_table
        }
    }
    page_table
}
/// Change the level4 page table the system uses for virtual memory mapping
fn set_page_table(page_table: &mut page::Table<page::Level4Entry>) {
    unsafe {
        asm! {
            "mov cr3, rax",
            in("rax") page_table
        }
    }
}

/// A 4K-aligned page 
/// ```rust
/// use kalloc::*;
/// use core::mem::{align_of, size_of};
/// assert_eq!(align_of::<Page>(), 4096);
/// assert_eq!(size_of::<Page>(), 4096);
/// ```
#[repr(C, align(4096))]
pub struct Page([u8; 4096]);
impl Deref for Page {
    type Target = [u8; 4096];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Page {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct VirtualAddress(u64);
impl VirtualAddress {
    pub const NULL: Self = Self(0);
    pub fn increment_page(&mut self) {
        **self += 4096;
    }
    #[inline(always)]
    pub fn level4_entry(self) -> usize {
        ((*self >> 39) & 0x1FF) as _
    }
    #[inline(always)]
    pub fn level3_entry(self) -> usize {
        ((*self >> 30) & 0x1FF) as _
    }
    #[inline(always)]
    pub fn level2_entry(self) -> usize {
        ((*self >> 21) & 0x1FF) as _
    }
    #[inline(always)]
    pub fn level1_entry(self) -> usize {
        ((*self >> 12) & 0x1FF) as _
    }
    #[inline(always)]
    pub fn offset(self) -> usize {
        (*self & 0xFFF) as _
    }
    #[inline(always)]
    pub fn page(self) -> usize {
        (*self & 0x000F_FFFF_FFFF_F000) as _
    }
    #[inline(always)]
    pub fn page_table(self) -> usize {
        (*self & 0x000F_FFFF_FFE0_0000) as _
    }
}
impl Deref for VirtualAddress {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for VirtualAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> From<*mut T> for VirtualAddress {
    fn from(r: *mut T) -> Self {
        Self(r as _)
    }
}

pub mod page {
    use core::{ops::{Deref, DerefMut, Index, IndexMut}, ptr::null_mut};
    use super::{Page,VirtualAddress};

    /// A 4K-aligned page of PagePointer<Level> containing 512 pointers to lower level page table entries.
    /// ```rust
    /// use kalloc::*;
    /// use core::mem::{align_of, size_of};
    /// assert_eq!(align_of::<page::Table<page::Level1Entry>>(), 4096);
    /// assert_eq!(size_of::<page::Table<page::Level1Entry>>(), 4096);
    /// ```
    #[repr(C, align(4096))]
    pub struct Table<L: Deref<Target=Pointer>>([L; 512]);
    impl<L: Deref<Target=Pointer>> Deref for Table<L> {
        type Target = [L; 512];
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<L: Deref<Target=Pointer>> DerefMut for Table<L> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<L: Deref<Target=Pointer> + Default + Copy> Default for Table<L> {
        fn default() -> Self {
            Self([Default::default(); 512])
        }
    }
    impl Index<VirtualAddress> for Table<Level4Entry> {
        type Output = Level4Entry;
        fn index(&self, address: VirtualAddress) -> &Self::Output {
            &(**self)[address.level4_entry()]
        }
    }
    impl IndexMut<VirtualAddress> for Table<Level4Entry> {
        fn index_mut(&mut self, address: VirtualAddress) -> &mut Self::Output {
            &mut (**self)[address.level4_entry()]
        }
    }
    impl Index<VirtualAddress> for Table<Level3Entry> {
        type Output = Level3Entry;
        fn index(&self, address: VirtualAddress) -> &Self::Output {
            &(**self)[address.level3_entry()]
        }
    }
    impl IndexMut<VirtualAddress> for Table<Level3Entry> {
        fn index_mut(&mut self, address: VirtualAddress) -> &mut Self::Output {
            &mut (**self)[address.level3_entry()]
        }
    }
    impl Index<VirtualAddress> for Table<Level2Entry> {
        type Output = Level2Entry;
        fn index(&self, address: VirtualAddress) -> &Self::Output {
            &(**self)[address.level2_entry()]
        }
    }
    impl IndexMut<VirtualAddress> for Table<Level2Entry> {
        fn index_mut(&mut self, address: VirtualAddress) -> &mut Self::Output {
            &mut (**self)[address.level2_entry()]
        }
    }
    impl Index<VirtualAddress> for Table<Level1Entry> {
        type Output = Level1Entry;
        fn index(&self, address: VirtualAddress) -> &Self::Output {
            &(**self)[address.level1_entry()]
        }
    }
    impl IndexMut<VirtualAddress> for Table<Level1Entry> {
        fn index_mut(&mut self, address: VirtualAddress) -> &mut Self::Output {
            &mut (**self)[address.level1_entry()]
        }
    }
    impl Table<Level4Entry> {
        #[inline]
        /// Get page pointed to be a virtual address
        pub unsafe fn page(&self, address: VirtualAddress) -> *mut Page {
            let table = self[address].address();
            if table.is_null() { return null_mut() }
            let table = (*table)[address].address();
            if table.is_null() { return null_mut() }
            let table = (*table)[address].address();
            if table.is_null() { return null_mut() }
            (*table)[address].address()
        }
        #[inline]
        /// Get page pointed to be a virtual address
        pub unsafe fn page_entry(&self, address: VirtualAddress) -> Option<&mut Level1Entry> {
            let table = self[address].address();
            if table.is_null() { return None }
            let table = (*table)[address].address();
            if table.is_null() { return None }
            let table = (*table)[address].address();
            if table.is_null() { return None }
            Some(&mut (*table)[address])
        }
        #[inline(always)]
        /// Get the exact physical address for a virtual address
        pub unsafe fn physical<T>(&self, address: VirtualAddress) -> *mut T {
            self.page(address).add(address.offset()) as _
        }
    }

    /// Generic Page Entry for any level
    #[derive(Copy, Clone, Debug)]
    #[repr(transparent)]
    pub struct Pointer(u64);
    impl Pointer {
        #[inline(always)]
        pub fn present(self) -> bool {
            (self.0 & 0x1) != 0
        }
        #[inline(always)]
        pub fn set_present(&mut self) {
            self.0 |= 0x1
        }
        #[inline(always)]
        pub fn unset_present(&mut self) {
            self.0 &= !0x1
        }
        #[inline(always)]
        pub fn write(self) -> bool {
            (self.0 & 0x2) != 0
        }
        #[inline(always)]
        pub fn set_write(&mut self) {
            self.0 |= 0x2
        }
        #[inline(always)]
        pub fn unset_write(&mut self) {
            self.0 &= !0x2
        }
        #[inline(always)]
        pub fn user(self) -> bool {
            (self.0 & 0x4) != 0
        }
        #[inline(always)]
        pub fn set_user(&mut self) {
            self.0 |= 0x4
        }
        #[inline(always)]
        pub fn unset_user(&mut self) {
            self.0 &= !0x4
        }
        #[inline(always)]
        pub fn write_through(self) -> bool {
            (self.0 & 0x8) != 0
        }
        #[inline(always)]
        pub fn set_write_through(&mut self) {
            self.0 |= 0x8
        }
        #[inline(always)]
        pub fn unset_write_through(&mut self) {
            self.0 &= !0x8
        }
        #[inline(always)]
        pub fn cache(self) -> bool {
            (self.0 & 0x10) != 0
        }
        #[inline(always)]
        pub fn set_cache(&mut self) {
            self.0 |= 0x10
        }
        #[inline(always)]
        pub fn unset_cache(&mut self) {
            self.0 &= !0x10
        }
        #[inline(always)]
        pub fn accessed(self) -> bool {
            (self.0 & 0x20) != 0
        }
        #[inline(always)]
        pub fn set_accessed(&mut self) {
            self.0 |= 0x20
        }
        #[inline(always)]
        pub fn unset_accessed(&mut self) {
            self.0 &= !0x20
        }
    }
    /// A Page Mode Level-4 Entry (PML4E)
    #[derive(Copy, Clone, Default, Debug)]
    #[repr(transparent)]
    pub struct Level4Entry(u64);
    impl Level4Entry {
        /// The physical address of a Level3Entry page table
        #[inline(always)]
        pub fn address(&self) -> *mut Table<Level3Entry> {
            unsafe { core::mem::transmute(self.0 & 0x000F_FFFF_FFFF_F000) }
        }
        /// Set the physical address to a Level3Entry page table
        #[inline(always)]
        pub fn set_address(&mut self, table: *mut Table<Level3Entry>) {
            // Note: as a pointer to a physical address it shall not be larger than 52 bits and an `&mut` guarantees alignment
            self.0 = (self.0 & !0x000F_FFFF_FFFF_F000) | table as *mut _ as u64
        }
    }
    impl Deref for Level4Entry {
        type Target = Pointer;
        fn deref(&self) -> &Self::Target {
            // Safe: page::Pointer and page::Level4Entry have the exact same layout, including compatible lifetimes
            unsafe { core::mem::transmute(self) }
        }
    }
    /// A Page Directory Pointer Entry (PDPTE)
    #[derive(Copy, Clone, Default, Debug)]
    #[repr(transparent)]
    pub struct Level3Entry(u64);
    impl Level3Entry {
        /// The physical address of a Level2Entry page table
        #[inline(always)]
        pub fn address(&self) -> *mut Table<Level2Entry> {
            unsafe { core::mem::transmute(self.0 & 0x000F_FFFF_FFFF_F000) }
        }
        /// Set the physical address to a Level2Entry page table
        #[inline(always)]
        pub fn set_address(&mut self, table: *mut Table<Level2Entry>) {
            // Note: as a pointer to a physical address it shall not be larger than 52 bits and an `&mut` guarantees alignment
            self.0 = (self.0 & !0x000F_FFFF_FFFF_F000) | table as *mut _ as u64
        }
    }
    impl Deref for Level3Entry {
        type Target = Pointer;
        fn deref(&self) -> &Self::Target {
            // Safe: page::Pointer and page::Level3Entry have the exact same layout, including compatible lifetimes
            unsafe { core::mem::transmute(self) }
        }
    }
    /// A Page Directory Entry (PDE)
    #[derive(Copy, Clone, Default, Debug)]
    #[repr(transparent)]
    pub struct Level2Entry(u64);
    impl Level2Entry {
        /// The physical address of a Level1Entry page table
        #[inline(always)]
        pub fn address(&self) -> *mut Table<Level1Entry> {
            unsafe { core::mem::transmute(self.0 & 0x000F_FFFF_FFFF_F000) }
        }
        /// Set the physical address to a Level1Entry page table
        #[inline(always)]
        pub fn set_address(&mut self, table: *mut Table<Level1Entry>) {
            // Note: as a pointer to a physical address it shall not be larger than 52 bits and an `&mut` guarantees alignment
            self.0 = (self.0 & !0x000F_FFFF_FFFF_F000) | table as *mut _ as u64
        }
    }
    impl Deref for Level2Entry {
        type Target = Pointer;
        fn deref(&self) -> &Self::Target {
            // Safe: page::Pointer and page::Level2Entry have the exact same layout, including compatible lifetimes
            unsafe { core::mem::transmute(self) }
        }
    }
    /// A Page Entry (PTE)
    #[derive(Copy, Clone, Default, Debug)]
    #[repr(transparent)]
    pub struct Level1Entry(u64);
    impl Level1Entry {
        /// The physical address of a page
        #[inline(always)]
        pub fn address(&self) -> *mut Page {
            unsafe { core::mem::transmute(self.0 & 0x000F_FFFF_FFFF_F000) }
        }
        /// Set the physical address to a page
        #[inline(always)]
        pub fn set_address(&mut self, table: *mut Page) {
            // Note: as a pointer to a physical address it shall not be larger than 52 bits and an `&mut` guarantees alignment
            self.0 = (self.0 & !0x000F_FFFF_FFFF_F000) | table as *mut _ as u64
        }
    }
    impl Deref for Level1Entry {
        type Target = Pointer;
        fn deref(&self) -> &Self::Target {
            // Safe: page::Pointer and page::Level1Entry have the exact same layout, including compatible lifetimes
            unsafe { core::mem::transmute(self) }
        }
    }
}