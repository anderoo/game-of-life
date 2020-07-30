mod cell;
mod universe;

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::Dialog;
use cursive::views::DummyView;
use cursive::views::LinearLayout;
use cursive::views::TextView;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};


fn main() {
    // Arc's for thread access and management
    let finished = Arc::new(AtomicBool::new(false));
    let state = Arc::new(Mutex::new(universe::Universe::new(64)));

    // Create cursive instance
    let mut siv = cursive::default();

    // Allow users to Quit using Q
    let game_exited = Arc::clone(&finished);
    siv.add_global_callback('q', move |s| {
        game_exited.store(true, Ordering::SeqCst);
        s.quit();
    });

    // Sidebar
    let sidebar_view = LinearLayout::vertical()
        .child(TextView::new("Stats"))
        .child(DummyView)
        .child(TextView::new("").with_name("alive_cells"))
        .child(TextView::new("").with_name("dead_cells"));

    // Universe view
    let main_view = Dialog::around(TextView::new("").with_name("universe"));

    // Add layers for GUI
    siv.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(sidebar_view)
        .child(DummyView)
        .child(main_view))
        .title("Game of Life"));

    let cb = siv.cb_sink().clone();

    thread::spawn({
        let finished = Arc::clone(&finished);
        let state = Arc::clone(&state);

        move || {
            while !finished.load(Ordering::SeqCst) {
                let mut game = state.lock().unwrap();

                game.tick();
                let universe_text = format!("{}", game);
                let alive_cells_text = format!("Alive cells: {}", game.alive_cells());
                let dead_cells_text = format!("Dead cells: {}", game.dead_cells());

                cb.send(Box::new(move |s: &mut Cursive| {
                    s.call_on_name("universe", |v: &mut TextView|v.set_content(universe_text));
                    s.call_on_name("alive_cells", |v: &mut TextView| v.set_content(alive_cells_text));
                    s.call_on_name("dead_cells", |v: &mut TextView| v.set_content(dead_cells_text));
                })).expect("Unable to send callback to Cursive");

                thread::sleep(Duration::from_millis(250));
            }
        }
    });

    siv.set_autorefresh(true);
    siv.run();
}
