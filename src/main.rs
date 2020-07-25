use sysinfo::SystemExt;

struct Bar {
    max: u64,               // Value when 100% full
    value: u64,             // Current value
    length: usize,          // Length of bar in characters
    empty_character: char,  // Char to draw for empty
    filled_character: char, // Char to draw for full
}

impl Bar {
    fn new(max: u64, length: usize, empty_character: char, filled_character: char) -> Bar {
        Bar {
            max: max,
            length: length,
            value: 0,
            empty_character: empty_character,
            filled_character: filled_character,
        }
    }

    fn set_value(&mut self, value: u64) {
        self.value = value;
    }

    fn get_fill_percent(&self) -> f64 {
        self.value as f64 / self.max as f64
    }

    fn make_bar(&self) -> String {
        let mut out = String::new();
        // Calculate number of segments to show
        let n_filled = (self.get_fill_percent() * self.length as f64) as usize;
        let n_empty = self.length as usize - n_filled;
        // Build the bar
        out.push('[');
        let filled = std::iter::repeat(self.filled_character)
            .take(n_filled)
            .collect::<String>();
        let empty = std::iter::repeat(self.empty_character)
            .take(n_empty)
            .collect::<String>();
        out.push_str(&filled);
        out.push_str(&empty);
        out.push(']');
        // Build the percent sign
        let percent_filled = self.get_fill_percent();
        let percent_str = format!(" ({:>.2}%)", percent_filled * 100f64);
        out.push_str(&percent_str);
        out
    }

    fn show(&self) {
        let bar = self.make_bar();
        println!("{}", bar);
    }
}

fn get_mem_usage(s: &mut sysinfo::System) -> u64 {
    s.refresh_all();
    s.get_used_memory()
}

fn main() {
    let mut s = sysinfo::System::new();
    s.refresh_all();
    let total = s.get_total_memory();

    let mut b = Bar::new(total, 100, ' ', '=');
    let mut i = 0;
    loop {
        if i >= 10 {
            break;
        }
        b.set_value(get_mem_usage(&mut s));
        b.show();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        i += 1;
    }
}
