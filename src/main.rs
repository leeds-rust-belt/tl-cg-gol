use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 300;
const HEIGHT: usize = 300;
const BUFSIZE: usize = WIDTH * HEIGHT;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Game of life - press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X4,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    window.set_target_fps(60);
    window.set_background_color(0, 0, 20);

    let mut state: [bool; BUFSIZE] = [false; BUFSIZE];
    for i in 0..BUFSIZE {
        state[i] = rand::random();
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        state = do_gol_generation(state);

        for (i, pixel) in buffer.iter_mut().enumerate() {
            *pixel = if state[i] {
                0xffffff
            } else {
                0x00
            }
        }
        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn coords_to_index(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

fn do_gol_generation(state: [bool; BUFSIZE]) -> [bool; BUFSIZE] {
    let mut newstate: [bool; BUFSIZE] = [false; BUFSIZE];
    for i in 0..BUFSIZE {
        let x = i % WIDTH;
        let y = i / HEIGHT;

        let cur = state[i];
        let mut co = 0;
        let n = if y == 0 {false} else {state[coords_to_index(x, y-1)]};
        let s = if y >= HEIGHT-1 {false} else {state[coords_to_index(x, y+1)]};
        let e = if x >= WIDTH-1 {false} else {state[coords_to_index(x+1, y)]};
        let w = if x == 0 {false} else {state[coords_to_index(x-1, y)]};

        let ne = if y == 0 || x >= WIDTH-1 {false} else { state[(y-1) * WIDTH + x+1] };
        let nw = if y == 0 || x == 0 {false} else { state[(y-1) * WIDTH + x-1] };
        let se = if y >= HEIGHT-1 || x >= WIDTH-1 {false} else { state[(y+1) * WIDTH + x+1] };
        let sw = if y >= HEIGHT-1 || x == 0  {false} else { state[(y+1) * WIDTH + x-1] };

        if n {
            co += 1;
        }
        if s {
            co += 1;
        }
        if e {
            co += 1;
        }
        if w {
            co += 1;
        }
        if ne {
            co += 1;
        }
        if nw {
            co += 1;
        }
        if se {
            co += 1;
        }
        if sw {
            co += 1;
        }

        if cur {
            if co <2 || co > 3 {
                newstate[i] = false;
            } else {
                newstate[i] = true;
            }
        } else {
            if co == 3 {
                newstate[i] = true;
            } else {
                newstate[i] = false;
            }
        }
    }
    newstate
}
