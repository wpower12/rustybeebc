
use crate::beebc;
use crate::beebc::{
    signal
};
use tui::{
    backend::{Backend},
    Terminal,
    widgets::{Block, Borders, Paragraph, List, ListItem},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color}
};

pub fn render<B: Backend>(terminal: &mut Terminal<B>, ebc: &beebc::EBC, ram: [u8; 16],last_cw: u16){
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Length(16),
                Constraint::Min(16),
                Constraint::Length(16),
            ].as_ref())
            .split(f.size());
        { // Left Column
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3)
                ].as_ref())
                .split(chunks[0]);

            f.render_widget(make_u4register("PC", ebc.pc), chunks[0]);
            f.render_widget(make_u4register("SC", ebc.sc), chunks[1]);
            f.render_widget(make_irregister("IR", ebc.ir), chunks[2]);
            f.render_widget(make_u8register("MAR", ebc.mar), chunks[3]);
            f.render_widget(make_help_text(), chunks[4]); 
        }
        { // Middle "Bus"/{Controlwords, RAM}
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(20),
                    Constraint::Length(3)
                ].as_ref())
                .split(chunks[1]);

            f.render_widget(make_u8register("BUS", ebc.bus), chunks[0]);
            
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ].as_ref())
                    .split(chunks[1]);

                f.render_widget(make_ram(ram), chunks[0]);
                f.render_widget(make_cw_list(last_cw, beebc::decode_instruction(ebc)), chunks[1]);
            }
        }
        { // Right Column
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ].as_ref())
                .split(chunks[2]);

            f.render_widget(make_u8register("REG A", ebc.reg_a), chunks[0]);
            f.render_widget(make_u8register("REG B", ebc.reg_b), chunks[1]);
            f.render_widget(make_u8register("REG ALU", ebc.reg_alu), chunks[2]);
            f.render_widget(make_u4register("FLAGS", ebc.reg_flgs), chunks[3]);
            f.render_widget(make_u8register("OUT",   ebc.reg_out), chunks[4]);
            f.render_widget(make_output("OUT-DEC",   ebc.reg_out), chunks[5]);
        }
    }).expect("draw failed.");
}

fn make_u8register(title: &str, value: u8) -> Paragraph {
    return Paragraph::new(format!("{:08b}", value))
        .block(Block::default()
        .title(title)
        .borders(Borders::ALL));
}

fn make_u4register(title: &str, value: u8) -> Paragraph {
    return Paragraph::new(format!("{:04b}", value))
        .block(Block::default()
        .title(title)
        .borders(Borders::ALL));
}

fn make_irregister(title: &str, value: u8) -> Paragraph {
    return Paragraph::new(format!("{:04b} | {:04b}", value >> 4, value & 0b00001111))
        .block(Block::default()
        .title(title)
        .borders(Borders::ALL));
}


fn make_output(title: &str, value: u8) -> Paragraph {
    return Paragraph::new(format!("{:03}", value))
        .block(Block::default()
        .title(title)
        .borders(Borders::ALL));
}

fn make_cw_list(last_cw: u16, next_cw: u16) -> List<'static> {
    fn make_row_str(label:&str, cws1: u16, cws2: u16) -> String {
        let v1 = if cws1 > 0 { 1 } else { 0 };
        let v2 = if cws2 > 0 { 1 } else { 0 };
        return format!("{} |  {}  |  {}  |", label, v1, v2);
    }

    let items = [
            ListItem::new(" sig  last  next | "),
            ListItem::new(make_row_str("HLT ", last_cw & signal::HLT, next_cw & signal::HLT)),
            ListItem::new(make_row_str(" MI ", last_cw & signal::MI,  next_cw & signal::MI)),
            ListItem::new(make_row_str(" RI ", last_cw & signal::RI,  next_cw & signal::RI)),
            ListItem::new(make_row_str(" RO ", last_cw & signal::RO,  next_cw & signal::RO)),
            ListItem::new(make_row_str(" IO ", last_cw & signal::IO,  next_cw & signal::IO)),
            ListItem::new(make_row_str(" II ", last_cw & signal::II,  next_cw & signal::II)),
            ListItem::new(make_row_str(" AI ", last_cw & signal::AI,  next_cw & signal::AI)),
            ListItem::new(make_row_str(" AO ", last_cw & signal::AO,  next_cw & signal::AO)),
            ListItem::new(make_row_str(" EO ", last_cw & signal::EO,  next_cw & signal::EO)),
            ListItem::new(make_row_str(" SU ", last_cw & signal::SU,  next_cw & signal::SU)),
            ListItem::new(make_row_str(" BI ", last_cw & signal::BI,  next_cw & signal::BI)),
            ListItem::new(make_row_str(" OI ", last_cw & signal::OI,  next_cw & signal::OI)),
            ListItem::new(make_row_str(" CE ", last_cw & signal::CE,  next_cw & signal::CE)),
            ListItem::new(make_row_str(" CO ", last_cw & signal::CO,  next_cw & signal::CO)),
            ListItem::new(make_row_str(" J_ ", last_cw & signal::J_,  next_cw & signal::J_)),
            ListItem::new(make_row_str(" FI ", last_cw & signal::FI,  next_cw & signal::FI)),];

    return List::new(items).block(Block::default().title("CW").borders(Borders::ALL))
    .style(Style::default().fg(Color::White));
}

fn make_ram(ram: [u8; 16]) -> List<'static> {
    let items: Vec<ListItem> = ram.iter().enumerate().map(|(c, a)| ListItem::new(format!(" {:04b} | {:08b} ", c, a))).collect();
    return List::new(items).block(Block::default().title("RAM").borders(Borders::ALL))
    .style(Style::default().fg(Color::White));
}

fn make_help_text() -> List<'static> {
    let items = [
        ListItem::new("p - pause"),
        ListItem::new("s - step"),
        ListItem::new("q - quit"),
    ];
    return List::new(items).block(Block::default())
    .style(Style::default().fg(Color::White));
}