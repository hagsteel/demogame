use florp_engine::events::term::{events, Event, KeyCode, KeyEvent};
use florp_engine::{term_size, WorldPos, WorldSize, ScreenPos, ScreenSize, Camera, Renderer, Viewport, UI};

mod tiles;

fn main() {
    let (width, height) = term_size().expect("can't get the term size? can't play the game!");

    let ui = UI::new();
    let ui_viewport = Viewport::new(ScreenPos::zero(), ScreenSize::new(width, 3));
    let viewport_size = ScreenSize::new(width / 2, height / 2);
    let camera_size = WorldSize::new(width / 2, height / 2);

    let mut camera = Camera::new(WorldPos::new(width / 2, height / 2), camera_size);
    camera.set_limit(5, 5);
    let mut viewport = Viewport::new(ScreenPos::new(0, 4), viewport_size);
    let mut renderer = Renderer::new().expect("Failed to enter raw mode");

    // Player
    let mut player = tiles::Character(camera.position);

    // Walls
    let walls = walls(
        camera.position.x - camera_size.width / 2,
        camera.position.y - camera_size.height / 2,
    );

    let ground = gravel(
        camera.position.x - camera_size.width / 2,
        camera.position.y - camera_size.height / 2,
    );

    for event in events(30) {
        match event {
            Event::Tick => {
                let mut ground: Vec<(char, WorldPos)> = ground
                    .iter()
                    .map(|ground| (ground.pixel(), ground.position()))
                    .collect();

                let mut bg: Vec<(char, WorldPos)> = walls
                    .iter()
                    .map(|wall| (wall.pixel(), wall.position()))
                    .collect();

                bg.append(&mut ground);

                let fg = vec![((player.pixel(), player.position()))];

                viewport.draw(&camera, fg, bg);
                let _ = renderer.render(&mut viewport);
            }
            Event::Key(KeyEvent { code: KeyCode::Esc, ..  }) => break,
            Event::Key(KeyEvent { code: kc, .. }) => {
                // camera.clear_bg(&mut layers, player.0);

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

                camera.track(player.position());
                // let bg_pixel = layers.bg.get_pixel(camera.to_local(new));
                // ui.text = format!("x: {} y: {} | old x: {} old y: {} | bg: {:?}", new.x, new.y, old.x, old.y, bg_pixel);
                ui.render(&ui_viewport, &mut renderer.stdout);
            }
        }
    }
}

fn walls(start_x: u16, start_y: u16) -> Vec<tiles::Wall> {
    let mut w = Vec::new();
    let width = 100;
    let height = 30;

    for x in start_x..start_x + width {
        w.push(tiles::Wall(WorldPos::new(x, start_y)));
        w.push(tiles::Wall(WorldPos::new(x, start_y + height)));
    }

    for y in start_y..start_y + height {
        w.push(tiles::Wall(WorldPos::new(start_x, y)));
        w.push(tiles::Wall(WorldPos::new(start_x + width, y)));
    }
    w
}

fn gravel(start_x: u16, start_y: u16) -> Vec<tiles::Gravel> {
    let start_x = start_x + 1;
    let start_y = start_y + 1;
    let mut w = Vec::new();
    let width = 58;
    let height = 18;

    for x in start_x..start_x + width {
        for y in start_y..start_y + height {
            w.push(tiles::Gravel(WorldPos::new(x, y)));
        }
    }

    w
}
