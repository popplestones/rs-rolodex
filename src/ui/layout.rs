// use ratatui::layout::*;
// pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
//     let vertical = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Min((area.height.saturating_sub(height)) / 2),
//             Constraint::Length(height),
//             Constraint::Min((area.height.saturating_sub(height)) / 2),
//         ])
//         .split(area);
//
//     let horizontal = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Min((area.width.saturating_sub(width)) / 2),
//             Constraint::Length(width),
//             Constraint::Min((area.width.saturating_sub(width)) / 2),
//         ])
//         .split(vertical[1]);
//
//     horizontal[1]
// }
