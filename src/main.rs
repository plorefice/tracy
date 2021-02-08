use std::fs;

use trtc::scenes;

fn main() {
    fs::write(
        "chapter-02.ppm",
        scenes::chapter_02(900, 550).convert_to_ppm(),
    )
    .unwrap();
}
