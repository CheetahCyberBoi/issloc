use ratatui::layout::*;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Block, Borders, BorderType, block::Title};
use ratatui::widgets::canvas::*;
use crate::*;

use log::{debug, error, info, trace, warn};


//This is called every frame to render the UI for displaying data.
pub fn ui(app: &mut App, frame: &mut Frame) {
    info!("Constructing UI!");
    let data = &app.current_data;
    //First layout: Large Rect, smaller one taking up minmum 5 lines.
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(8),
            Constraint::Percentage(100),
        ]).split(frame.size());
    
    //Second layout: A 50/50 split of the top section of the first layout.
    let info_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(layout[0]);
    info!("Layout made!");
    

    //Set up the leftward paragraph: displaying the currently tracked target (always the ISS lol :P)
    let mut lines = vec![];
    lines.push(ratatui::text::Line::from(vec![
        Span::styled("Currently tracking: ", Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC)),
    ]));

    lines.push(ratatui::text::Line::from(vec![
        Span::styled("International Space Station ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
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
    info!("Tracking PG made!");

    //Render the rightward paragraph: Contains information about the currently tracked object.
    let mut lines = vec![];
    lines.push(ratatui::text::Line::from(vec![
        Span::styled("Current information as of timestamp ", Style::default().fg(Color::Blue)),
        Span::styled(format!("{}", data.timestamp), Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD).add_modifier(Modifier::ITALIC)),
        Span::styled(format!(" (Ping delay: {}ms)", app.delay), Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
    ]));

    //Latitude
    lines.push(ratatui::text::Line::from(vec![
        Span::styled("Geo Latitude: ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
        Span::styled(format!("{0:.2} degrees", data.latitude), Style::default().fg(Color::Yellow)),
    ]));

    //Longitude
    lines.push(ratatui::text::Line::from(vec![
        Span::styled("Geo longitude: ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
        Span::styled(format!("{0:.2} degrees", data.longitude), Style::default().fg(Color::Yellow)),
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
    info!("Tracking info PG made!");

    //Add smoe instructions
    let instructions = Title::from(ratatui::text::Line::from(vec![
        " Ping ".into(),
        "<Enter>".blue().bold(),
        " Increase delay ".into(),
        "<Right>".green().bold(),
        " Decrease delay ".into(),
        "<Left>".green().bold(),
        " Quit ".into(),
        "<Q>".red().bold(),
    ]));

    //Render the bottom box: A large map with a point in it (eventually :P)
    let canvas = Canvas::default()
        .block(
            Block::default()
                .title(Title::from("Map View").alignment(Alignment::Center))
                .title(
                    instructions
                        .alignment(Alignment::Center)
                        .position(ratatui::widgets::block::title::Position::Bottom)
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            info!("Drawing world!");
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: Color::LightGreen,
            });
        });
    
    frame.render_widget(canvas, layout[1]);
    info!("Map made!");

    //Final things: Add some instructions for the keys for querying the ISS data and exiting


        




    
    
}