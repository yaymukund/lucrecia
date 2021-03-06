use anyhow::Result;
use std::convert::TryInto;

use crate::ui::{Event, IntoListener, Listener};
use crate::util::{channel, terminal, SendDiscard};

pub struct Terminal;

pub struct TerminalListener {
    sender: channel::Sender<Event>,
}

impl IntoListener for Terminal {
    type LType = TerminalListener;
    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        Self::LType::new(sender)
    }
}

impl Listener for TerminalListener {
    fn on_event(&mut self, event: &Event) -> Result<()> {
        match *event {
            // Event::FocusNext => self.focus_next(),
            Event::Tick => self.wait_event()?,
            Event::Resize(_cols, _rows) => {
                terminal::clear_all();
                self.sender.send_discard(Event::Draw)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl TerminalListener {
    pub fn new(sender: channel::Sender<Event>) -> Self {
        terminal::init();
        Self { sender }
    }

    fn next_event(&self) -> Option<Event> {
        terminal::poll_event().and_then(|ev| ev.try_into().ok())
    }

    fn wait_event(&self) -> Result<()> {
        if let Some(event) = self.next_event() {
            self.sender.send_discard(event)?;
        }

        Ok(())
    }
}

impl Drop for TerminalListener {
    fn drop(&mut self) {
        terminal::deinit();
    }
}
