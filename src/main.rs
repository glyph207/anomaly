use std::io;

struct State {
    n_days: u32,
}

fn show_info(st: &State) {
    println!("current day: {}", st.n_days);
}

fn next_day(st: &mut State) {
    st.n_days = st.n_days + 1;
}

fn main() {
    let mut state = State {
        n_days: 1,
    };
    println!("Welcome to Anomaly!");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("invalid input");
        match input.trim() {
            "next" => next_day(&mut state),
            "quit" => break,
            _ => println!("invalid command"),
        };
        show_info(&state);
    }
    
}
