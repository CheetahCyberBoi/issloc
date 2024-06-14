use crate::IssData;
use ratatui::layout::*;
use ratatui::Frame;


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
    

    
    
}