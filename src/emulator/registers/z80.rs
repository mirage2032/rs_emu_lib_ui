use crate::emulator::registers::Register;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use leptos::{
    component, view, IntoView, ReadSignal, Signal, SignalUpdate, SignalWith,
    WriteSignal,
};

#[component]
pub fn registers(
    emu_read: ReadSignal<Emulator<Z80>>,
    emu_write: WriteSignal<Emulator<Z80>>,
) -> impl IntoView {
    let af =
        Signal::derive(move || emu_read.with(|emu| format!("{:04X}", emu.cpu.registers.gp.af)));
    let set_af = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.gp.af = num;
        });
        Ok(())
    };

    let bc =
        Signal::derive(move || emu_read.with(|emu| format!("{:04X}", emu.cpu.registers.gp.bc)));
    let set_bc = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.gp.bc = num;
        });
        Ok(())
    };

    let de =
        Signal::derive(move || emu_read.with(|emu| format!("{:04X}", emu.cpu.registers.gp.de)));
    let set_de = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.gp.de = num;
        });
        Ok(())
    };

    let hl =
        Signal::derive(move || emu_read.with(|emu| format!("{:04X}", emu.cpu.registers.gp.hl)));
    let set_hl = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.gp.hl = num;
        });
        Ok(())
    };

    let sp = Signal::derive(move || emu_read.with(|emu| format!("{:04X}", emu.cpu.registers.sp)));
    let set_sp = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.sp = num;
        });
        Ok(())
    };

    let pc = Signal::derive(move || emu_read.with(|emu| format!("{:04X}", emu.cpu.registers.pc)));
    let set_pc = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.pc = num;
        });
        Ok(())
    };

    let ix = Signal::derive(move || emu_read.with(|emu| format!("{:04X}", emu.cpu.registers.ix)));
    let set_ix = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.ix = num;
        });
        Ok(())
    };

    let iy = Signal::derive(move || emu_read.with(|emu| format!("{:04X}", emu.cpu.registers.iy)));
    let set_iy = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u16::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.iy = num;
        });
        Ok(())
    };

    let i = Signal::derive(move || emu_read.with(|emu| format!("{:02X}", emu.cpu.registers.i)));
    let set_i = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u8::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.i = num;
        });
        Ok(())
    };

    let r = Signal::derive(move || emu_read.with(|emu| format!("{:02X}", emu.cpu.registers.r)));
    let set_r = move |value: &str| -> Result<(), std::num::ParseIntError> {
        let num = u8::from_str_radix(value, 16)?;
        emu_write.update(|emu| {
            emu.cpu.registers.r = num;
        });
        Ok(())
    };

    view! {
        <div style:display="flex">
            <Register name="AF" maxlength=4 get=af set=set_af />
            <Register name="BC" maxlength=4 get=bc set=set_bc />
            <Register name="DE" maxlength=4 get=de set=set_de />
            <Register name="HL" maxlength=4 get=hl set=set_hl />
            <Register name="SP" maxlength=4 get=sp set=set_sp />
            <Register name="PC" maxlength=4 get=pc set=set_pc />
            <Register name="IX" maxlength=4 get=ix set=set_ix />
            <Register name="IY" maxlength=4 get=iy set=set_iy />
            <Register name="I" maxlength=2 get=i set=set_i />
            <Register name="R" maxlength=2 get=r set=set_r />
        </div>
    }
}
