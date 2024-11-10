use x86_64::{structures::paging::PageTable, VirtAddr};

pub unsafe fn active_level_4_page_table(offset: VirtAddr) -> &'static PageTable {
    use x86_64::registers::control::Cr3;

    // Find the level 4 page table by reading the Cr3 register.
    let (level_4_table_frame, _) = Cr3::read();

    // Create a virtual address by applying our offset.
    let phys_addr = level_4_table_frame.start_address();
    let virt_addr = offset + phys_addr.as_u64();

    // Return a mutable reference to a page table
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();
    return &mut *page_table_ptr;

} 