extern crate ncurses;

use std::char;
use ncurses::*;

mod display;

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;

fn main()
{
    let display = display::Display::new();
    display.setup();

    /* Status/help info. */
    printw("Use the arrow keys to move");
    mvprintw(LINES - 1, 0, "Press q to exit");
    refresh();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr, &mut max_y, &mut max_x);

    /* Start in the center. */
    let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
    let mut start_x = (max_x - WINDOW_WIDTH) / 2;
    let mut win = create_win(start_y, start_x);

    let mut ch = getch();
    while char::from_u32(ch as u32).unwrap() != 'q'
    {
        match ch
        {
            KEY_LEFT =>
            {
                start_x -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            KEY_RIGHT =>
            {
                start_x += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            KEY_UP =>
            {
                start_y -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            KEY_DOWN =>
            {
                start_y += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            _ => { }
        }
        ch = getch();
    }
    display.cleanup();

}

fn create_win(start_y: i32, start_x: i32) -> WINDOW
{
    let win = newwin(WINDOW_HEIGHT, WINDOW_WIDTH, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn destroy_win(win: WINDOW)
{
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}
