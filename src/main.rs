use iced::{executor,Application,Command,Element,Settings,Theme,widget::{button,column,text,Column}};


struct GUI{
    counter:Counter,
}

/**Counter Struct*/
struct Counter{
    value: i32,
}

/**Message Struct */
#[derive(Debug, Clone, Copy)]
pub enum Message{
    IncrementPressed,
    DecrementPressed,
}

/**Counter Inc/Dec Logic */

impl Counter{
    pub fn view(&self) -> Column<self::Message>{
        column![
            button("+").on_press(Message::IncrementPressed),

            text(self.value).size(50),

            button("-").on_press(Message::DecrementPressed),
        ]
    }
    pub fn update(&mut self,message:Message){
        match message{
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed =>{
                self.value -= 1;
            }
        }
    }
    pub fn init(&mut self){
        self.value=0;
    }
}

/**implementation for GUI Applications */

impl Application for GUI{
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    
    fn new(_flags:())->(GUI,Command<Self::Message>){
        (GUI{counter:Counter{value:0}},Command::none())
    }

    fn title(&self) -> String{
        String::from("DEMO")
    }

    fn update(&mut self, _message: Self::Message)->Command<Self::Message>{
        self.counter.update(_message);
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        self.counter.view().into()
    }
}

fn main() { 
    GUI::run(Settings::default()).expect("msg");

}
