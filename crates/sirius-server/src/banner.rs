use rand::seq::IndexedRandom;

pub fn print_sirius_banner(env: String) {
    let ascii_art = r#"
        ▄████████  ▄█     ▄████████  ▄█  ███    █▄     ▄████████
       ███    ███ ███    ███    ███ ███  ███    ███   ███    ███
       ███    █▀  ███▌   ███    ███ ███▌ ███    ███   ███    █▀
       ███        ███▌  ▄███▄▄▄▄██▀ ███▌ ███    ███   ███
     ▀███████████ ███▌ ▀▀███▀▀▀▀▀   ███▌ ███    ███ ▀███████████
              ███ ███  ▀███████████ ███  ███    ███          ███
        ▄█    ███ ███    ███    ███ ███  ███    ███    ▄█    ███
      ▄████████▀  █▀     ███    ███ █▀   ████████▀   ▄████████▀
"#;

    let version = env!("CARGO_PKG_VERSION");

    let quotes = [
        "Morningstar said we'd rock in 2026. The Dogstar is ready.",
        "The brightest star in the sky.",
        "Borrow checker approved this emulator.",
        "Safe, concurrent and ready to host.",
        "Thanks for the roadmap, Morningstar.",
        "A new star for a new era.",
        "Rocking in 2026, just like they promised.",
        "Shining brighter than the Morningstar.",
        "Who needs a CMS when the emulator is this beautiful?",
        "Parsing packets faster than you can say bobba.",
    ];

    let mut rng = rand::rng();
    let selected_quote = quotes.choose(&mut rng).unwrap();

    let banner = format!(
        "{}\n      v{} [{}] | {}",
        ascii_art,
        version,
        env.to_uppercase(),
        selected_quote
    );

    let start_color = (200.0, 230.0, 255.0);
    let end_color = (0.0, 80.0, 255.0);

    let lines: Vec<&str> = banner.lines().collect();
    let num_lines = lines.len();

    let mut colored_banner = String::new();

    for (i, line) in lines.iter().enumerate() {
        let ratio = if num_lines > 1 {
            i as f32 / (num_lines - 1) as f32
        } else {
            0.0
        };

        let r = (start_color.0 + ratio * (end_color.0 - start_color.0)) as u8;
        let g = (start_color.1 + ratio * (end_color.1 - start_color.1)) as u8;
        let b = (start_color.2 + ratio * (end_color.2 - start_color.2)) as u8;

        colored_banner.push_str(&format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m\n",
            r, g, b, line
        ));
    }

    println!("{}", colored_banner);

    if env == "development" {
        println!(
            "\x1b[1;33m      WARNING:\x1b[0m Running in DEVELOPMENT mode. Do not use in production!\n"
        );
    }
}
