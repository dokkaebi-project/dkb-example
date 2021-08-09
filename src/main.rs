use std::fs;
use dkb_rs::hangul::Dkb844;
use dkb_rs::latin::FNT;
use dkb_rs::kanakanji::FONTX;
use dkb_rs::CharacterRenderer;
use pretty_hex::{HexConfig, config_hex};

fn fontprinter(buf: &[u8], width: usize) {
    let mut idx: usize = 0;
    for byte in buf.iter() {
        for off in 0..8 {
            let bit: u8 = (byte << (off)) & 0x80;
            match bit {
                0 => print!("□"),
                _ => print!("■"),
            };

            idx += 1;
            if idx % width == 0 {
                print!("\n");
            }
        }
    }
}

fn main() {
    let dkbfnt = &fs::read("./font-archive/hangul/free/kimjoongtaefonts/H04.FNT").unwrap();
    let latifnt = &fs::read("./font-archive/hangul/free/kimjoongtaefonts/E1.FNT").unwrap();
    let fontxfnt = &fs::read("./font-archive/kana-kanji/free/FreeDOSV-Suzu/04GZN16X.FNT").unwrap();
    
    let dkb = Dkb844::new(&dkbfnt, 16, 16);
    let fnt = FNT::new(&latifnt, 8, 16);
    let fontx = FONTX::new(&fontxfnt).unwrap();

    let mut buf: [u8; 2 * 16] = [0; 2 * 16];
    match dkb.render('가', &mut buf) {
        Ok(_) => {},
        Err(_) => panic!("Failed to render font!"),
    }

    let cfg = HexConfig {title: true, width: 16, group: 0, ..HexConfig::default() };
    println!("{}", config_hex(&buf, cfg));
    fontprinter(&buf, 16);

    let mut buf2: [u8; 1 * 16] = [0; 1 * 16];
    match fnt.render('A', &mut buf2) {
        Ok(_) => {},
        Err(_) => panic!("Failed to render font!"),
    }

    println!("{}", config_hex(&buf2, cfg));
    fontprinter(&buf2, 8);

    match fontx.render('ツ', &mut buf) {
        Ok(_) => {},
        Err(_) => panic!("Failed to render font!"),
    }
    println!("{}", config_hex(&buf, cfg));
    fontprinter(&buf, 16);

    println!("Hello, world!");
}
