struct WindowManager {}
use xcb::Connection;
const NEXT_WINDOW: &str = "NEXT_WINDOW";
const PREVIOUS_WINDOW: &str = "PREVIOUS_WINDOW";

impl WindowManager {
    pub fn new() -> Self {
        let (conn, _): (xcb::Connection, i32) = xcb::Connection::connect(None)
            .expect("Unable to access your display. Check your DISPLAY environment variable.");

        let conn = xcb_util::ewmh::Connection::connect(conn);

        WindowManager {}
    }
    pub fn run() {}
}
