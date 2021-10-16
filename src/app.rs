pub struct App {
    pub contents: Vec<String>,
    pub num_of_page: usize,
    pub num_of_line: Vec<usize>,
    pub current_page: usize,
    pub scroll_offset: u16,
    pub scrolloff_limit: u16,
    pub can_scroll: bool,
}
