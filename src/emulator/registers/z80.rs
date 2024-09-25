use crate::emulator::registers::Register;
use crate::emulator::EmuSignals;
use leptos::{component,view, IntoView};
use leptos::prelude::*;

#[component]
pub fn registers() -> impl IntoView {
    let emu_signals = expect_context::<EmuSignals>();
    let af = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:04X}", emu.cpu.registers.gp.af))
    });
    let set_af = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.gp.af = num;
        });
        Ok(())
    };

    let bc = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:04X}", emu.cpu.registers.gp.bc))
    });
    let set_bc = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.gp.bc = num;
        });
        Ok(())
    };

    let de = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:04X}", emu.cpu.registers.gp.de))
    });
    let set_de = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.gp.de = num;
        });
        Ok(())
    };

    let hl = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:04X}", emu.cpu.registers.gp.hl))
    });
    let set_hl = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.gp.hl = num;
        });
        Ok(())
    };

    let sp = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:04X}", emu.cpu.registers.sp))
    });
    let set_sp = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.sp = num;
        });
        Ok(())
    };

    let pc = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:04X}", emu.cpu.registers.pc))
    });
    let set_pc = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.pc = num;
        });
        Ok(())
    };

    let ix = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:04X}", emu.cpu.registers.ix))
    });
    let set_ix = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.ix = num;
        });
        Ok(())
    };

    let iy = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:04X}", emu.cpu.registers.iy))
    });
    let set_iy = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.iy = num;
        });
        Ok(())
    };

    let i = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:02X}", emu.cpu.registers.i))
    });
    let set_i = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u8::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.i = num;
        });
        Ok(())
    };

    let r = Signal::derive(move || {
        emu_signals
            .read
            .with(|emu| format!("{:02X}", emu.cpu.registers.r))
    });
    let set_r = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u8::from_str_radix(value, 16)?;
        emu_signals.write.update(|emu| {
            emu.cpu.registers.r = num;
        });
        Ok(())
    };

    view! {
        <div style:display="flex">
            {Register("AF", 4, af, set_af)} {Register("BC", 4, bc, set_bc)}
            {Register("DE", 4, de, set_de)} {Register("HL", 4, hl, set_hl)}
            {Register("SP", 4, sp, set_sp)} {Register("PC", 4, pc, set_pc)}
            {Register("IX", 4, ix, set_ix)} {Register("IY", 4, iy, set_iy)}
            {Register("I", 2, i, set_i)} {Register("R", 2, r, set_r)}
        </div>
    }
}
