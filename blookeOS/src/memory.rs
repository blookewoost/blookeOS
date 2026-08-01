use x86_64::{ VirtAddr, structures::paging::{OffsetPageTable, PageTable}};

#[allow(clippy::missing_safety_doc)]
unsafe fn active_level_4_page_table(offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;
    let (level_4_table_frame, _) = Cr3::read();
    let phys_addr = level_4_table_frame.start_address();
    let virt_addr = offset + phys_addr.as_u64();

    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();
    &mut *page_table_ptr

} 

#[allow(clippy::missing_safety_doc)]
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let level_4_table = active_level_4_page_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    }
}