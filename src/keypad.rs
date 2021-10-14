use sdl2::keyboard::Keycode;

pub fn scan_key(keycode: Keycode) -> Option<usize> {
    return match keycode {
        // numbers
        Keycode::Num1 => Some(0),
        Keycode::Num2 => Some(1),
        Keycode::Num3 => Some(2),
        Keycode::Num4 => Some(3),

        // letters
        Keycode::Q => Some(4),
        Keycode::W => Some(5),
        Keycode::E => Some(6),
        Keycode::R => Some(7),
        Keycode::A => Some(8),
        Keycode::S => Some(9),
        Keycode::D => Some(10),
        Keycode::F => Some(11),
        Keycode::Z => Some(12),
        Keycode::X => Some(13),
        Keycode::C => Some(14),
        Keycode::V => Some(15),

        _ => Option::None,
    };
}
