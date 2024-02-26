use rand::{rngs::ThreadRng, Rng};
use slint::{Model, ModelRc, PlatformError, VecModel};
use std::{cell::RefCell, rc::Rc};

slint::include_modules!();

const N: usize = 4;
const COUNT: usize = 100;
const DIR: [[i32; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

struct AppState {
    main_window: slint::Weak<MainWindow>,
}

fn main() -> Result<(), PlatformError> {
    let main_window: MainWindow = MainWindow::new()?;

    let state: Rc<RefCell<AppState>> = Rc::new(RefCell::new(AppState {
        main_window: main_window.as_weak(),
    }));

    main_window.set_number_array(vec_to_model_rc(random_array()));

    let state_copy: Rc<RefCell<AppState>> = state.clone();
    main_window.on_reset(move || {
        let app: MainWindow = state_copy.borrow().main_window.unwrap();
        app.set_number_array(vec_to_model_rc(random_array()));
        app.set_win(false);
    });

    let state_copy: Rc<RefCell<AppState>> = state.clone();
    main_window.global::<GameLogic>().on_click_cell(move |index: i32, value| {
        if value.len() == 0 {
            return;
        }
        let app: MainWindow = state_copy.borrow().main_window.unwrap();
        if app.get_win() {
            return;
        }
        let mut new_array: Vec<i32> = app.get_number_array().iter().collect();

        let x: i32 = index / N as i32;
        let y: i32 = index % N as i32;
        for d in 0..4 {
            let new_x = x + DIR[d][0];
            let new_y = y + DIR[d][1];
            if in_area(new_x, new_y) {
                let new_index: usize = (new_x * N as i32 + new_y) as usize;
                if new_array[new_index] == -1 {
                    new_array.swap(index as usize, new_index);
                    let is_win: bool = check_win(&new_array);
                    if is_win {
                        app.set_win(true);
                    }
                    app.set_number_array(vec_to_model_rc(new_array));
                    break;
                }
            }
        }
    });
    main_window.run()
}

fn random_array() -> Vec<i32> {
    let length: usize = N * N - 1;
    let mut array: Vec<i32> = Vec::new();
    for i in 0..length {
        array.push(i as i32);
    }
    array.push(-1);

    let mut cur_index: usize = length;
    let mut rng: ThreadRng = rand::thread_rng();
    for _count in 0..COUNT {
        let cur_x: usize = cur_index / N;
        let cur_y: usize = cur_index % N;

        let mut d: usize = rng.gen_range(0..4);
        let mut new_x: i32 = cur_x as i32 + DIR[d][0];
        let mut new_y: i32 = cur_y as i32 + DIR[d][1];
        while !in_area(new_x, new_y) {
            d = rng.gen_range(0..4);
            new_x = cur_x as i32 + DIR[d][0];
            new_y = cur_y as i32 + DIR[d][1];
        }

        let new_x: usize = new_x as usize;
        let new_y: usize = new_y as usize;
        let new_index: usize = new_x * N + new_y;

        array.swap(cur_index, new_index);

        cur_index = new_index;
    }

    array
}

fn vec_to_model_rc(array: Vec<i32>) -> ModelRc<i32> {
    Rc::new(VecModel::from(array)).into()
}

fn check_win(array: &Vec<i32>) -> bool {
    let mut index = 0;
    let mut flag = true;
    for ele in array {
        if index < 15 && *ele != index {
            flag = false;
            break;
        }
        index += 1;
    }
    flag
}

fn in_area(x: i32, y: i32) -> bool {
    let length: i32 = N as i32;
    (x >= 0 && x < length) && (y >= 0 && y < length)
}
