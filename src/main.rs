#![windows_subsystem = "windows"]

use cast::{i32, usize};
use int_cmp::IntCmp;
use rand::{rngs::ThreadRng, Rng};
use slint::{Model, ModelRc, PlatformError, SharedString, VecModel};
use std::{cell::RefCell, rc::Rc, usize};

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
        let app: MainWindow = state_copy
            .borrow()
            .main_window
            .unwrap();
        app.set_number_array(vec_to_model_rc(random_array()));
        app.set_win(false);
    });

    let state_copy: Rc<RefCell<AppState>> = state.clone();
    main_window.global::<GameLogic>().on_click_cell(move |index: i32, value: SharedString| {
        if value.len() == 0 {
            return;
        }
        let app: MainWindow = state_copy
            .borrow()
            .main_window
            .unwrap();
        if app.get_win() {
            return;
        }
        let mut new_array: Vec<i32> = app.get_number_array().iter().collect();

        let usize_index: usize = usize(index).unwrap();
        let point: (usize, usize) = (usize_index / N, usize_index % N);
        for d in 0..4 {
            let new_point: (i32, i32) = get_new_point(point, (DIR[d][0], DIR[d][1]));
            if in_area(new_point) {
                let new_index: usize = point_to_index(new_point);
                if new_array[new_index] == -1 {
                    new_array.swap(usize(index).unwrap(), new_index);
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
        array.push(i32(i).unwrap());
    }
    array.push(-1);

    let mut cur_index: usize = length;
    let mut rng: ThreadRng = rand::thread_rng();
    for _ in 0..COUNT {
        let cur_point: (usize, usize) = (cur_index / N, cur_index % N);

        let mut d: usize = rng.gen_range(0..4);
        let mut new_point: (i32, i32) = get_new_point(cur_point, (DIR[d][0], DIR[d][1]));
        while !in_area(new_point) {
            d = rng.gen_range(0..4);
            new_point = get_new_point(cur_point, (DIR[d][0], DIR[d][1]));
        }

        let new_index: usize = point_to_index(new_point);

        array.swap(cur_index, new_index);

        cur_index = new_index;
    }

    array
}

fn get_new_point(cur: (usize, usize), d: (i32, i32)) -> (i32, i32) {
    (i32(cur.0).unwrap() + d.0, i32(cur.1).unwrap() + d.1)
}

fn point_to_index(point: (i32, i32)) -> usize {
    let x: usize = usize(point.0).unwrap();
    let y: usize = usize(point.1).unwrap();
    x * N + y
}

fn vec_to_model_rc(array: Vec<i32>) -> ModelRc<i32> {
    Rc::new(VecModel::from(array)).into()
}

fn check_win(array: &Vec<i32>) -> bool {
    let length: usize = array.len() - 1;
    let mut flag: bool = true;
    for (i, ele) in array.iter().enumerate() {
        if i < 15 && (*ele).cmp_ne(i) {
            flag = false;
            break;
        }
        if i == length {
            flag = (*ele) == -1;
        }
    }
    flag
}

fn in_area(p: (i32, i32)) -> bool {
    let length: i32 = i32(N).unwrap();
    (p.0 >= 0 && p.0 < length) && (p.1 >= 0 && p.1 < length)
}
