use crate::matcher::{Match, MatchReceiver};
use crate::keyboard::KeyboardSender;
use crate::config::Configs;

pub struct Engine<S> where S: KeyboardSender {
    sender: S,
    configs: Configs,
}

impl <S> Engine<S> where S: KeyboardSender{
    pub fn new(sender: S, configs: Configs) -> Engine<S> where S: KeyboardSender {
        Engine{sender, configs }
    }
}

impl <S> MatchReceiver for Engine<S> where S: KeyboardSender{
    fn on_match(&self, m: &Match) {
        self.sender.delete_string(m.trigger.len() as i32);

        // To handle newlines, substitute each "\n" char with an Enter key press.
        let splits = m.replace.lines();

        for (i, split) in splits.enumerate() {
            if i > 0 {
                self.sender.send_enter();
            }

            self.sender.send_string(split);
        }
    }
}