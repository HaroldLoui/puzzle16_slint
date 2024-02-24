slint::include_modules!();

fn main() {
    // MainWindow::new().unwrap().run().unwrap();
    let main_window = MainWindow::new().unwrap();

    main_window.run().unwrap();
}
