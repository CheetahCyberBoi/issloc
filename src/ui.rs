use crate::IssData;
use ratatui::layout::*;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Block, Borders, BorderType};
use ratatui::widgets::canvas::*;


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
    lines.push(ratatui::text::Line::from(vec![
        Span::styled("Currently tracking: ", Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC)),
    ]));

    lines.push(ratatui::text::Line::from(vec![
        Span::styled("International Space Station", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
        Span::styled(format!("ID {}", 25544), Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
    ]));
    
    let currently_tracking_pg = Paragraph::new(Text::from(lines))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Tracking")
                .border_type(BorderType::Thick)
        );
    frame.render_widget(currently_tracking_pg, info_layout[0]);

    //Render the rightward paragraph: Contains information about the currently tracked object.
    let mut lines = vec![];
    lines.push(ratatui::text::Line::from(vec![
        Span::styled("Current information as of timestamp ", Style::default().fg(Color::Blue)),
        Span::styled(format!("{}", data.timestamp), Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD).add_modifier(Modifier::ITALIC))
    ]));

    //Latitude
    lines.push(ratatui::text::Line::from(vec![
        Span::styled("Geo Latitude: ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
        Span::styled(format!("{0:.2}", data.latitude), Style::default().fg(Color::Yellow)),
    ]));

    //Longitude
    lines.push(ratatui::text::Line::from(vec![
        Span::styled("Geo longitude: ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
        Span::styled(format!("{0:.2}", data.longitude), Style::default().fg(Color::Yellow)),
    ]));

    let tracking_info_pg = Paragraph::new(Text::from(lines))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tracking Info")
                .border_type(BorderType::Thick)
        );
    
    frame.render_widget(tracking_info_pg, info_layout[1]);

    //Render the bottom box: A large map with a point in it (eventually :P)
    let canvas = Canvas::default()
        .block(Block::bordered().title("Map View"))
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: Color::LightGreen,
            });
        });
    
    frame.render_widget(canvas, layout[1]);





    
    
}