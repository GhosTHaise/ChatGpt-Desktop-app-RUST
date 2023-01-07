

pub struct HeadlinesConfig{
    dark_mode : bool,
    api_key : String
}
impl Default for HeadlinesConfig{
    fn Default() -> Self {
        Self {
            dark_mode : Default::default(),
            api_key : String::new()
        }
    }
}
pub struct Headlines{
    config : HeadlinesConfig,
    api_key_initialized : bool
}

pub struct NewBotResponse{
    bot : String
}