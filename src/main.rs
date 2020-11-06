use florp_engine::events::term::{events, Event, KeyCode, KeyEvent};
use florp_engine::{term_size, Point, Size, TermCamera, TermLayers, TermRenderer, TermViewport, UI};

mod tiles;

fn main() {
    let _renderer = TermRenderer::new();
    let (width, height) = term_size().expect("can't get the term size? can't play the game!");

    let mut ui = UI::new();
    let ui_viewport = TermViewport::new(Point::zero(), Size::new(width, 3));
    let view_size = Size::new(width / 2, height / 2);

    let mut camera = TermCamera::new(Point::new(width / 2, height / 2), view_size);
    camera.set_limit(5, 5);
    let viewport = TermViewport::new(Point::new(0, 4), view_size);
    let mut layers = TermLayers::new(view_size);
    let mut renderer = TermRenderer::new().expect("Failed to enter raw mode");

    // Player
    let mut player = tiles::Character(camera.position);

    // Walls
    let walls = walls(
        camera.position.x - view_size.width / 2,
        camera.position.y - view_size.height / 2,
    );

    let ground = gravel(
        camera.position.x - view_size.width / 2,
        camera.position.y - view_size.height / 2,
    );

    for event in events(30) {
        match event {
            Event::Tick => {
                let mut ground: Vec<(char, Point)> = ground
                    .iter()
                    .map(|ground| (ground.pixel(), ground.position()))
                    .collect();

                let mut bg: Vec<(char, Point)> = walls
                    .iter()
                    .map(|wall| (wall.pixel(), wall.position()))
                    .collect();

                bg.append(&mut ground);

                let fg = vec![((player.pixel(), player.position()))];

                camera.render(&mut layers, fg, bg);
                let _ = renderer.render(&mut layers, &viewport);
            }
            Event::Key(KeyEvent { code: KeyCode::Esc, ..  }) => break,
            Event::Key(KeyEvent { code: kc, .. }) => {
                camera.clear_bg(&mut layers, player.0);

                match kc {
                    KeyCode::Left => {
                        player.0.x -= 1;
                    }
                    KeyCode::Right => {
                        player.0.x += 1;
                    }
                    KeyCode::Up => {
                        player.0.y -= 1;
                    }
                    KeyCode::Down => {
                        player.0.y += 1;
                    }
                    _ => {}
                }

                let (new, old) = camera.track(player.position());
                let pos = player.position();
                ui.text = format!("x: {} y: {} | old x: {} old y: {}", new.x, new.y, old.x, old.y);
                ui.render(&ui_viewport, &mut renderer.stdout);
            }
        }
    }
}

fn walls(start_x: u16, start_y: u16) -> Vec<tiles::Wall> {
    let mut w = Vec::new();
    let width = 70;
    let height = 30;

    for x in start_x..start_x + width {
        w.push(tiles::Wall(Point::new(x, start_y)));
        w.push(tiles::Wall(Point::new(x, start_y + height)));
    }

    for y in start_y..start_y + height {
        w.push(tiles::Wall(Point::new(start_x, y)));
        w.push(tiles::Wall(Point::new(start_x + width, y)));
    }
    w
}

fn gravel(start_x: u16, start_y: u16) -> Vec<tiles::Gravel> {
    let start_x = start_x + 1;
    let start_y = start_y + 1;
    let mut w = Vec::new();
    let width = 68;
    let height = 28;

    for x in start_x..start_x + width {
        for y in start_y..start_y + height {
            w.push(tiles::Gravel(Point::new(x, y)));
        }
    }

    w
}
