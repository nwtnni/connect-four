extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate minimax;

use relm::*;
use gtk::prelude::*;
use gtk::{Window, WindowType};

use minimax::minimax::AI;
use minimax::engine::*;

#[derive(Msg)]
enum Msg {
    Play(u8),
    Restart,
    Quit,
}

struct Win {
    model: Engine<AI, Human>,
    window: Window,
}

impl Update for Win {
    type Model = Engine<AI, Human>;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Self::Model {
        human_vs_cpu()
    }

    fn update(&mut self, message: Self::Msg) {
        if let Msg::Quit = message {
            gtk::main_quit(); 
        } 
    }
}

impl Widget for Win {
    type Root = Window;
    
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

     fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win {
            model,
            window: window,
        }
    }
}

pub fn main() {
    std::env::set_var("GDK_BACKEND", "x11");
    relm::run::<Win>(()).unwrap();
}
