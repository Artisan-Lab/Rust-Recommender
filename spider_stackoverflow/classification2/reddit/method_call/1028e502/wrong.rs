fn can_increment(&self) -> bool {
    self.value < u8::MAX
}

fn can_decrement(&self) -> bool {
    self.value > u8::MIN
}

fn view(&mut self) -> iced::Element<'_, Self::Message> {
    let increment_button_enabled = self.can_increment();
    let decrement_button_enabled = self.can_decrement();

    let mut increment_button =
        Button::new(&mut self.increment_button_state, Text::new("+")).padding(20);
    if self.can_increment() {
        increment_button = increment_button.on_press(Message::Increment);
    }

    let mut decrement_button =
        Button::new(&mut self.decrement_button_state, Text::new("-")).padding(20);
    if decrement_button_enabled {
        decrement_button = decrement_button.on_press(Message::Decrement);
    }
    //... (irrelevant code cut)
}