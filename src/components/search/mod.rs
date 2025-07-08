// use ratatui::{
//     layout::{Constraint, Direction, Layout},
//     prelude::*,
//     widgets::*,
// };
//
// use crate::components::{Component, input::Input};
//
// pub enum SearchMsg {}
// pub enum SearchOutput {}
//
// #[derive(Debug, Default)]
// pub struct Search {
//     pub query: String,
//     pub input: Input,
// }
//
// impl Search {
//     pub fn new() -> Self {
//         Self {
//             query: String::new(),
//             input: Input::new("Search", "foo", 10),
//         }
//     }
//     pub fn handle_key(&self, _event: crossterm::event::KeyEvent) -> Option<SearchMsg> {
//         None
//     }
//     pub fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
//
//         let chunks = Layout::default()
//             .direction(Direction::Horizontal)
//             .constraints([Constraint::Length(8), Constraint::Min(0)])
//             .split(area);
//
//         f.render_widget(Clear, chunks[0]);
//         f.render_widget(Paragraph::new("Search: "), chunks[0]);
//         self.input.draw(f, chunks[1], focused);
//     }
//     pub fn update<ParentMsg>(
//         &mut self,
//         _: SearchMsg,
//         _: impl Fn(SearchOutput) -> ParentMsg,
//     ) -> Option<ParentMsg> {
//         None
//     }
// }
//
// impl Component for Search {
//     type Msg = SearchMsg;
//     type Output = SearchOutput;
//
//     fn update<ParentMsg>(
//         &mut self,
//         msg: Self::Msg,
//         map: impl Fn(Self::Output) -> ParentMsg,
//     ) -> Option<ParentMsg> {
//         self.update(msg, map)
//     }
//
//     fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Self::Msg> {
//         self.handle_key(key)
//     }
//
//     fn draw(&self, f: &mut Frame, area: Rect, focused: bool) {
//         self.draw(f, area, focused)
//     }
// }
