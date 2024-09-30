#![no_std]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptType {
    Sync,
    Async,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    Edge,
    Level,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Fixed(u8),
    Dynamic(u8),
}

pub struct Irq {
    irq_type: InterruptType,
    trigger: Trigger,
    priority: Priority,
    maskable: bool,
    shared: bool,
    irq_num: usize,
}

pub trait GlobalIrqVector {
    const MAX_IRQS: usize;
    fn register_irq(&mut self, irq: Irq, irq_handler: &'static dyn IrqHandler);
    fn release_irq(&mut self, irq: usize);
    fn route_irq(&self, irq_num: usize);
}

pub trait IrqContext {}

pub trait IrqHandler {
    fn handler(&self, irq_context: &dyn IrqContext);
}
