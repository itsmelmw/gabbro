use crate::debugger::GameboyDebugger;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, debugger: &mut GameboyDebugger) {
    let term_chunks = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(4)].as_ref())
        .split(f.size());

    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(term_chunks[0]);

    let left_chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(info_chunks[0]);

    let right_chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(info_chunks[1]);

    let bottom_right_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(right_chunks[1]);

    draw_input(f, debugger, term_chunks[1]);
    draw_disassembly(f, debugger, left_chunks[0]);
    draw_stack(f, debugger, left_chunks[1]);
    draw_registers(f, debugger, right_chunks[0]);
    draw_breakpoints(f, debugger, bottom_right_chunks[0]);
    draw_watchpoints(f, debugger, bottom_right_chunks[1]);
}

pub fn draw_input<B: Backend>(f: &mut Frame<B>, debugger: &mut GameboyDebugger, area: Rect) {
    let text = vec![
        Spans::from(debugger.output()),
        Spans::from(vec![Span::from("> "), Span::from(debugger.input())]),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::from("Input"));
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

pub fn draw_disassembly<B: Backend>(f: &mut Frame<B>, debugger: &mut GameboyDebugger, area: Rect) {
    let text = debugger
        .disasm(area.height as usize - 2)
        .iter()
        .map(|(pc, addr, bytes, mnemonic)| {
            Spans::from(vec![
                Span::from(if *pc { "PC > " } else { "     " }),
                Span::from(format!("{:#06x} ", addr)),
                Span::from(format!(
                    "{:08} ",
                    bytes
                        .iter()
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<Vec<String>>()
                        .join(" ")
                )),
                Span::from(mnemonic.to_string()),
            ])
        })
        .collect::<Vec<Spans>>();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::from("Disassembly"));
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

pub fn draw_stack<B: Backend>(f: &mut Frame<B>, debugger: &mut GameboyDebugger, area: Rect) {
    let text = debugger
        .stack(area.height as usize - 2)
        .iter()
        .peekable()
        .map(|(sp, addr, val)| {
            Spans::from(vec![
                Span::from(if *sp { "SP > " } else { "     " }),
                Span::from(format!("{:#06x} ", addr)),
                Span::from(format!("{:04x} ", val)),
            ])
        })
        .collect::<Vec<Spans>>();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::from("Stack"));
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

pub fn draw_registers<B: Backend>(f: &mut Frame<B>, debugger: &mut GameboyDebugger, area: Rect) {
    let registers = debugger.registers();
    let text = vec![
        Spans::from(Span::from(format!(
            "A: {:#04x}    F: {}{}{}{}    AF: {:#06x}",
            registers.a(),
            if registers.flags().z() { 'Z' } else { '-' },
            if registers.flags().n() { 'N' } else { '-' },
            if registers.flags().h() { 'H' } else { '-' },
            if registers.flags().c() { 'C' } else { '-' },
            registers.af()
        ))),
        Spans::from(Span::from(format!(
            "B: {:#04x}    C: {:#04x}    BC: {:#06x}",
            registers.b(),
            registers.c(),
            registers.bc()
        ))),
        Spans::from(Span::from(format!(
            "D: {:#04x}    E: {:#04x}    DE: {:#06x}",
            registers.d(),
            registers.e(),
            registers.de()
        ))),
        Spans::from(Span::from(format!(
            "H: {:#04x}    L: {:#04x}    HL: {:#06x}",
            registers.h(),
            registers.l(),
            registers.hl()
        ))),
        Spans::from(Span::from(format!(
            "PC: {:#06x}      SP: {:#06x}",
            registers.pc(),
            registers.sp()
        ))),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::from("Registers"));
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

pub fn draw_breakpoints<B: Backend>(f: &mut Frame<B>, debugger: &mut GameboyDebugger, area: Rect) {
    let text = debugger
        .breakpoints()
        .iter()
        .enumerate()
        .map(|(i, bp)| Spans::from(Span::from(format!("{:02} {:#06x}", i, bp))))
        .collect::<Vec<Spans>>();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::from("Breakpoints"));
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

pub fn draw_watchpoints<B: Backend>(f: &mut Frame<B>, debugger: &mut GameboyDebugger, area: Rect) {
    let text = debugger
        .watchpoints()
        .iter()
        .enumerate()
        .map(|(i, (wp, val))| {
            Spans::from(Span::from(format!("{:02} {:#06x}: {:#04x}", i, wp, val)))
        })
        .collect::<Vec<Spans>>();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::from("Watchpoints"));
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
