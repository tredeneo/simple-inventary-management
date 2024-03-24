#![windows_subsystem = "windows"]

use simple_inventary::ui;
use slint::ComponentHandle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use simple_inventary::App;
    let myapp = App::new().unwrap();
    let _ = ui::user::user_list(&myapp).await;
    let _ = ui::user::user_detail_update(&myapp).await;
    let _ = ui::equipament::change_equipament(&myapp).await;
    let _ = ui::equipament::equipament_list(&myapp).await;
    let _ = ui::brand::brand(&myapp).await;
    let _ = ui::cpu::cpu(&myapp).await;
    let _ = ui::department::department(&myapp).await;

    myapp.run().unwrap();
    Ok(())
}
