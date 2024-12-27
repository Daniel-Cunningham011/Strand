use iced::widget::text_editor;
use iced::Element;
pub mod strand_app;

#[derive(Default)]
struct State {
    content: text_editor::Content, 
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
}

fn update(state: &mut State, message: Message) { 
    match message {
        Message::Edit(action) => {
            state.content.perform(action);
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
   text_editor(&state.content)
       .placeholder("")
       .on_action(Message::Edit)
       .into()
} 

pub fn main() -> iced::Result {
    iced::run("Strand", update, view)
}
