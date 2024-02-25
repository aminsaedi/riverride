use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};
use rand::Rng;

use crossterm::{
    cursor::{self, DisableBlinking, Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

struct World {
    maxc: u16,
    maxl: u16,
    player_c: u16,
    player_r: u16,
    river: Vec<(u16, u16)>,
}

fn draw(mut sc: &Stdout, world: &World) -> std::io::Result<()> {
    // draw the player on the screen
    sc.queue(cursor::MoveTo(world.player_c, world.player_r))?;
    sc.queue(Print("P"))?;

    // draw the river
    for line in 0..world.maxl {
        sc.queue(MoveTo(0,line))?;
        for _first_row in 0..world.river[line as usize].0 {
            sc.queue(Print("*"))?;
        }
        // sc.queue(Print("*"))?;
        sc.queue(MoveTo(world.river[line as usize].1, line))?;
        for _second_row in world.river[line as usize].1..world.maxc {
            sc.queue(Print("*"))?;
        }
        // sc.queue(Print("\n"))?;
        // sc.queue(MoveTo(0, line))?;
    }

    sc.flush()?;

    return Ok(());
}

// fn update_rive(mut world: &World) -> std::io::Result<()> {

//     let num = rand::thread_rng().gen_range(0..5);

//     for line in 0..world.maxl {
//         let mut river = &world.river;
//         river[line as usize].0 += num;
//         river[line as usize].1 += num;
        
//     }




//     return Ok(());
// }

fn main() -> std::io::Result<()> {
    // or using functions
    // stdout()
    //     .execute(cursor::MoveTo(10,10))?
    //     .execute(Print("Styled text here."))?;

    let mut sc = stdout();
    let (maxc, maxl) = size().unwrap();

    enable_raw_mode()?;
    sc.execute(DisableBlinking)?;
    sc.execute(Hide)?;

    let mut world = World {
        maxc: maxc,
        maxl: maxl,
        player_c: maxc / 2,
        player_r: maxl - 1,
        river: vec![(maxc / 2 - 10, maxc / 2 + 10); maxl as usize],
    };

    loop {
        sc.execute(Clear(ClearType::All))?;
        draw(&sc, &world)?;
        // update_rive(&mut world);

        if poll(Duration::from_millis(1000))? {
            match read()? {
                Event::Key(event) => {
                    let keyCode = event.code;

                    match keyCode {
                        KeyCode::Char('w') => {
                            world.player_r -= 1;
                        }
                        KeyCode::Char('s') => {
                            world.player_r += 1;
                        }
                        KeyCode::Char('a') => {
                            world.player_c -= 1;
                        }
                        KeyCode::Char('d') => {
                            world.player_c += 1;
                        }
                        KeyCode::Esc => {
                            break;
                        }

                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    sc.execute(Clear(ClearType::All))?;

    Ok(())
}
