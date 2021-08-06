use crossterm::style::Stylize;

pub fn construct_arrow_selection(header: &str, items: Vec<&str>, highlighted: i8, selected: bool) -> String {
    let mut result = String::from(header.to_owned() + "\n");
    let mut i = 0;
    let longest_item = "";
    let mut length_of_longest_item: i64 = 0;
    for item in items.clone() {
        if longest_item.len() < item.len() {
            length_of_longest_item = item.len() as i64;
        }
    }
    loop {
        if selected && highlighted == i as i8 {
            result.push_str(&*format!("{}", format!("{}) {}", i + 1, items[i].clone()).bold()));
        } else {
            result.push_str(&*format!("{}) {}", i + 1, items[i].clone()));
        }

        let mut filler = String::new();
        for _ in 0..(length_of_longest_item - items[i].len() as i64) {
            filler.push(' ');
        }
        result.push_str(&*filler);
        if highlighted == i as i8 && !selected { result.push_str("  <"); }

        result.push_str("\n");
        if i >= items.len() - 1 {
            break;
        }
        i += 1;
    }
    result
}