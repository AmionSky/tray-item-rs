#[cfg(target_os = "macos")]
fn main() {
    use tray_item::{TrayItem, IconSource};

    let mut tray = TrayItem::new("Tray Example", IconSource::Resource("")).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    }).unwrap();

    let mut inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();

}

#[cfg(not(target_os = "macos"))]
fn main() {}