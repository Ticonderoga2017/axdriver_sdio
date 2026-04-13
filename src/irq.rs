//! SDIO 中断控制抽象  
//!  
//! `SdioIrqControl` 从 `SdioHost` 分离，因为 ISR 中无法持锁获取 `SdioHost` 实例。  
//! 实现者必须保证所有方法可在中断上下文中安全调用。  

/// SDIO 中断控制抽象（ISR 安全）  
///  
/// 约束：所有方法不持锁、不分配堆、不调度。  
pub trait SdioIrqControl: Send + Sync {
    /// 获取 MMIO 基地址（ISR 中用于裸地址操作）  
    fn mmio_base(&self) -> usize;

    /// 屏蔽/恢复 CARD_INT 信号  
    ///  
    /// `mask = true`: 屏蔽（防止电平触发重复进入 ISR）  
    /// `mask = false`: 恢复（RX 处理完毕后重新使能）  
    fn mask_card_irq(&self, mask: bool);

    /// 使能 CARD_INT 中断信号（驱动初始化后调用）  
    fn enable_card_irq(&self);

    /// 禁用所有中断信号  
    fn disable_all_irq(&self);
}
