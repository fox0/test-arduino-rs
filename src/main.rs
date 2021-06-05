#![feature(no_core)]
#![feature(lang_items)]
#![feature(start)]
// #![feature(llvm_asm)]
// #![feature(asm)]

#![no_core]

#[lang = "sized"]
trait Sized {}

#[start]
#[allow(unreachable_code)]
fn main(_: isize, _: *const *const u8) -> isize {
    loop {}
    0
}

// /// No Operation
// #[inline(always)]
// #[cfg(target_arch = "avr")]
// pub fn nop() {
//     unsafe { asm!("nop"); };
// }
//
// /// Sleep / Wait For Interrupt
// #[inline(always)]
// pub fn sleep() {
//     cfg_if::cfg_if! {
//         if #[cfg(target_arch = "avr")] {
//             unsafe { llvm_asm!("sleep") }
//         } else {
//             unimplemented!()
//         }
//     }
// }
//
// /// Watchdog Reset
// #[inline(always)]
// pub fn wdr() {
//     cfg_if::cfg_if! {
//         if #[cfg(target_arch = "avr")] {
//             unsafe { llvm_asm!("wdr") }
//         } else {
//             unimplemented!()
//         }
//     }
// }
