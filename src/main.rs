use std::{io::{stdout, Stdout, Write}};

use crossterm::{
    cursor::{self, DisableBlinking, Hide}, event::{read, Event, KeyCode}, style::{Print,}, terminal::{enable_raw_mode, size, Clear, ClearType}, ExecutableCommand
};

struct World {
    player_c: u16,
    player_r: u16,
}

fn draw(mut sc: &Stdout, world: &World){
    sc.execute(cursor::MoveTo(world.player_c, world.player_r));
    sc.execute(Print("P"));
}

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
        player_c: maxc / 2,
        player_r: maxl - 1
    };


    loop {
        sc.execute(Clear(ClearType::All))?;
        draw(&sc, &world);
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => {
                if event.code == KeyCode::Esc {
                    break;
                }

                if event.code == KeyCode::Right {
                    world.player_c += 1;
                }
                if event.code == KeyCode::Left {
                    world.player_c -= 1;
                }
                if event.code == KeyCode::Up {
                    world.player_r -= 1;
                }
                if event.code == KeyCode::Down {
                    world.player_r += 1;
                }
            },
            Event::Mouse(event) => println!("{:?}", event),
            Event::Paste(data) => println!("{:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }

    }
    
    Ok(())
}
