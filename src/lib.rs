//! SDIO 主机控制器抽象层  

#![no_std]

pub mod cccr;
pub mod cmd;
pub mod error;

pub use cccr::SDIO_DEFAULT_BLOCK_SIZE;
pub use error::SdioError;

/// SDIO 主机控制器抽象  
///  
/// 实现者负责：  
/// - SDHCI 控制器初始化和 SDIO 卡枚举（CMD5 → CMD3 → CMD7）  
/// - CMD52 单字节读写（I/O read/write direct）  
/// - CMD53 多字节/块读写（I/O read/write extended）  
/// - Function 使能和 block size 设置  
pub trait SdioHost: Send + Sync {
    /// 初始化 SDHCI 控制器，执行 SDIO 卡枚举  
    fn init(&mut self) -> Result<(), SdioError>;

    /// CMD52: 单字节读 (I/O read direct)  
    fn read_byte(&self, func: u8, addr: u32) -> Result<u8, SdioError>;

    /// CMD52: 单字节写 (I/O write direct)  
    fn write_byte(&self, func: u8, addr: u32, val: u8) -> Result<(), SdioError>;

    /// CMD53: 多字节/块读 (I/O read extended, fixed address / FIFO 模式)  
    fn read_fifo(&self, func: u8, addr: u32, buf: &mut [u8]) -> Result<(), SdioError>;

    /// CMD53: 多字节/块写 (I/O write extended, fixed address / FIFO 模式)  
    fn write_fifo(&self, func: u8, addr: u32, buf: &[u8]) -> Result<(), SdioError>;

    /// 设置指定 function 的 block size  
    fn set_block_size(&self, func: u8, size: u16) -> Result<(), SdioError>;

    /// 设置 SDIO 时钟频率（Hz）  
    fn set_clock(&self, _hz: u32) -> Result<(), SdioError>;

    /// 使能指定 SDIO function  
    fn enable_func(&self, func: u8) -> Result<(), SdioError>;

    /// 获取 SDIO 卡的 vendor/device ID  
    fn vendor_device_id(&self) -> (u16, u16);

    /// 屏蔽 SDIO 卡中断（CARD_INT）    
    ///    
    /// 在 SDIO 总线操作（CMD52/CMD53）期间调用，防止 CARD_INT    
    /// 电平触发导致 ISR 重入。操作完成后调用 `unmask_card_irq()` 恢复。
    fn mask_card_irq(&self);

    /// 恢复 SDIO 卡中断（CARD_INT）
    fn unmask_card_irq(&self);
}
