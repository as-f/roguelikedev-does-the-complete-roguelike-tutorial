use bear_lib_terminal::terminal;

pub fn hello_world() {
    terminal::open("Test", 80, 30);
    terminal::print_xy(0, 0, "Hello, World!");
    terminal::refresh();
    terminal::wait_event();
    terminal::close();
}