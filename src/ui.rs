use crate::IssData;
use ratatui::layout::*;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Block, Borders, BorderType};


//This is called every frame to render the UI for displaying data.
pub fn ui(data: &IssData, frame: &mut Frame) {
    //First layout: Large Rect, smaller one taking up minmum 5 lines.
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(5),
            Constraint::Percentage(100),
        ]).split(frame.size());
    
    //Second layout: A 50/50 split of the top section of the first layout.
    let info_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(layout[0]);
    

    //Set up the leftward paragraph: displaying the currently tracked target (always the ISS lol :P)
    let mut lines = vec![];
    lines.push(Line::from(vec![
        Span::styled("Currently tracking: ", Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("International Space Station", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
        Span::styled(format!("ID {}", 25544), Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
    ]));
    let currently_tracking_pg = Paragraph::new(Text::from(lines))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tracking Info")
                .border_type(BorderType::Thick)
        );
    frame.render_widget(currently_tracking_pg, info_layout[0]);



    
    
}