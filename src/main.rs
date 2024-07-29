#![cfg_attr(
    all(target_os = "windows", build_release),
    windows_subsystem = "windows"
)]

use simple_inventary::ui;
use slint::ComponentHandle;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dbg!("start application");
    let myapp = simple_inventary::App::new().unwrap();
    ui::user::user_list(&myapp).await.ok();
    ui::user::user_detail(&myapp).await;
    ui::equipament::change_equipament(&myapp).await.ok();
    ui::equipament::equipament_list(&myapp).await.ok();
    ui::brand::brand(&myapp).await.ok();
    ui::cpu::cpu(&myapp).await.ok();
    ui::department::department(&myapp).await.ok();
    ui::equipament_model::equipament_list(&myapp).await.ok();
    ui::equipament_model::equipament_detail(&myapp).await;
    ui::gpu::gpu(&myapp).await.ok();

    myapp.run().unwrap();
    Ok(())
}
