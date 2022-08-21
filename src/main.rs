use clap::Parser;
use gen::Nature;
use std::{thread, time};

mod gen;
mod space;

#[derive(Parser)]
#[clap(about, long_about = None)]
struct Cli {
    /// Custom basic configuration
    #[clap(short, long)]
    custom: bool,

    /// Generation speed in milliseconds
    #[clap(long, value_parser, value_name = "MILLISECONDS")]
    speed: Option<u64>,

    /// Random basic configuration
    #[clap(short, long)]
    random: bool,
}

fn render(space: &space::Space) {
    print!("{esc}c", esc = 27 as char); // clean previous output

    let height = space.len();
    let width = space.first().unwrap().len();

    println!("\r");
    for r in 0..height {
        print!("|");
        for c in 0..(width - 1) {
            let t = space.get(r).unwrap().get(c).unwrap();
            if *t == space::State::Alive {
                print!("▇");
            } else {
                print!(" ");
            }

            print!("|");
        }
        println!();
    }
}

fn main() {
    let args = Cli::parse();

    let speed_milliseconds = args.speed.unwrap_or_default();
    let five_hundred_millisecond = time::Duration::from_millis(if speed_milliseconds == 0 {
        500
    } else {
        speed_milliseconds
    });

    let nature = if args.random {
        Nature::Random
    } else if args.custom {
        Nature::Custom
    } else {
        Nature::Default
    };
    let mut space = gen::gen_space(nature);

    let mut i = 1;
    loop {
        i += 1;

        render(&space);
        space = space::evaluate(&space);
        println!("{} iter", i);

        thread::sleep(five_hundred_millisecond);
    }
}
