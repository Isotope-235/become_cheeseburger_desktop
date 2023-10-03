use crate::*;
pub fn window_conf() -> Conf {
    let small_icon = image::open(r"assets\sprites\icon_s.png").unwrap();
    let medium_icon = image::open(r"assets\sprites\icon_m.png").unwrap();
    let large_icon = image::open(r"assets\sprites\icon_l.png").unwrap();
    Conf {
        window_title: TITLE.to_string(),
        window_width: (center().x() * 2.00 * scale()) as i32,
        window_height: (center().y() * 2.00 * scale()) as i32,
        sample_count: 0,
        window_resizable: false,
        icon: Some(miniquad::conf::Icon {
            small: small_icon.into_bytes().try_into().unwrap(),
            medium: medium_icon.into_bytes().try_into().unwrap(),
            big: large_icon.into_bytes().try_into().unwrap(),
        }),
        ..Default::default()
    }
}
