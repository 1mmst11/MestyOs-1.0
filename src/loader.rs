pub struct BootLoader;

impl BootLoader {
    pub fn show_splash_screen() {
        for _ in 0..50_000_000 {
            core::hint::spin_loop();
        }
    }
}
