use cursive::theme::{BaseColor, Color};

#[derive(Debug, Clone, Copy)]
pub enum MessageKind {
    Error,
    Info,
    Hint,
}

impl From<MessageKind> for Color {
    fn from(item: MessageKind) -> Self {
        match item {
            MessageKind::Error => Color::Dark(BaseColor::Red),
            MessageKind::Info => Color::Dark(BaseColor::Yellow),
            MessageKind::Hint => Color::Dark(BaseColor::White),
        }
    }
}

impl<T> From<T> for Message
where
    T: AsRef<str>,
{
    fn from(item: T) -> Self {
        return Message {
            msg: item.as_ref().to_string(),
            kind: MessageKind::Info,
        };
    }
}

pub struct Message {
    msg: String,
    kind: MessageKind,
}

impl Message {
    pub fn startup() -> Self {
        "Type :add <habit-name> <goal> to get started, Ctrl-L to dismiss".into()
    }
    pub fn contents(&self) -> &str {
        &self.msg
    }
    pub fn kind(&self) -> MessageKind {
        self.kind
    }
    pub fn set_kind(&mut self, k: MessageKind) {
        self.kind = k;
    }
    pub fn set_message<S: AsRef<str>>(&mut self, m: S) {
        self.msg = m.as_ref().into();
    }
    pub fn clear(&mut self) {
        self.msg.clear()
    }
}

impl std::default::Default for Message {
    fn default() -> Self {
        Message {
            msg: String::new(),
            kind: MessageKind::Info,
        }
    }
}
