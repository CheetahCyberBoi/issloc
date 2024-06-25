use std::rc::Rc;

use crate::*;
use crate::app::App;

use ratatui::layout::*;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Block, Borders, BorderType, block::Title};
use ratatui::widgets::canvas::*;

use log::{debug,info};


//This is called every frame to render the UI for displaying data.
pub fn ui(app: &mut App, frame: &mut Frame) {
    info!("Constructing UI!");
    let data = &app.current_data;
    debug!("Data in the UI function: {:#?}", data);
    let layout = create_layout(frame);
    render_currently_tracking_pg(data, frame, layout.1[0]);
    render_tracking_info_pg(data, app.delay, frame, layout.1[1]);
    render_map_view(data, frame, layout.0[1]);
}
//Creates the layout for the UI.
//The left part of the tuple is the main layout, the right part is the sublayout for the info.
pub fn create_layout(frame: &mut Frame) -> (Rc<[Rect]>, Rc<[Rect]>) {
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
    (layout, info_layout)
}

//Returns the keybinding instructions to display at the bottom of the screen.
pub fn get_keybinding_instructions() -> Title<'static> {
    Title::from(ratatui::text::Line::from(vec![
        " Ping ".into(),
        "<Enter>".blue().bold(),
        " Increase delay ".into(),
        "<Right>".green().bold(),
        " Decrease delay ".into(),
        "<Left>".green().bold(),
        " Quit ".into(),
        "<Q>".red().bold(),
    ]))
}
//Renders the "Map View" of the ISS.
pub fn render_map_view(data: &IssData, frame: &mut Frame, rect: Rect) {
    //Render the bottom box: A large map with a point in it (eventually :P)
    let canvas = Canvas::default()
        .block(
            Block::default()
                .title(Title::from("Map View").alignment(Alignment::Center))
                .title(
                    get_keybinding_instructions()
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
            ctx.draw(&Points {
                coords: &[(data.longitude, data.latitude)],
                color: Color::White,
            });
            ctx.print(data.longitude, data.latitude, ratatui::text::Line::from(vec![Span::styled("ISS", Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC))]));
        });
    
    frame.render_widget(canvas, rect);
    info!("Map made!");
}

//Renders the 'Tracking Info' paragraph
pub fn render_tracking_info_pg(data: &IssData, ping_delay: u64, frame: &mut Frame, rect: Rect) {
        //Render the rightward paragraph: Contains information about the currently tracked object.
    let mut lines = vec![];
    lines.push(ratatui::text::Line::from(vec![
        Span::styled("Current information as of timestamp ", Style::default().fg(Color::Blue)),
        Span::styled(format!("{}", data.timestamp), Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD).add_modifier(Modifier::ITALIC)),
        Span::styled(format!(" (Ping delay: {}ms)", ping_delay), Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
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
    
    frame.render_widget(tracking_info_pg, rect);
    info!("Tracking info PG made!");
}

//Renders the 'Currently Tracking' paragraph
pub fn render_currently_tracking_pg(data: &IssData, frame: &mut Frame, rect: Rect) {
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
    frame.render_widget(currently_tracking_pg, rect);
    info!("Tracking PG made!");
}