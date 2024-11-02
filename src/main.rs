use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use std::f64::consts::PI;
use crossterm::{
    cursor::{Hide, Show, MoveTo},
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    style::{SetForegroundColor, Color, Print,},
    ExecutableCommand,
    QueueableCommand,
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    
    // Enable this for better control
    enable_raw_mode()?;
    
    // Clear screen--hide cursor
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(Hide)?;

    // radar parameters (define)
    let center_x = 30;
    let center_y = 15;
    let radius = 14;
    let mut angle = 0.0;
    
    println!("Starting Wi-Fi scan...");
    
    // Main animation loop
    for _ in 0..120 {
        // Clear previous frame
        stdout.execute(Clear(ClearType::All))?;
        
        //  radar circles
        draw_radar_circles(&mut stdout, center_x, center_y, radius)?;
        
        // Draw scanning line
        draw_scan_line(&mut stdout, center_x, center_y, radius, angle)?;
        
        stdout.flush()?;
        
        // angle for next frame
        angle = (angle + 0.2) % (2.0 * PI);
        sleep(Duration::from_millis(50));
    }
    
    // Clean-up
    stdout.execute(Show)?;
    disable_raw_mode()?;
    stdout.execute(Clear(ClearType::All))?;
    println!("Scan complete. Found networks!");
    
    Ok(())
}

fn draw_radar_circles(stdout: &mut io::Stdout, center_x: u16, center_y: u16, max_radius: u16) -> io::Result<()> {
    // Draw multiple concentric circles
    for radius in (max_radius/3..=max_radius).step_by((max_radius/3) as usize) {
        for angle in (0..360).step_by(5) {
            let radian = (angle as f64) * PI / 180.0;
            let x = center_x as f64 + (radius as f64 * radian.cos());
            let y = center_y as f64 + (radius as f64 * radian.sin() * 0.5); // Multiply by 0.5 for oval effect
            
            stdout
                .queue(MoveTo(x as u16, y as u16))?
                .queue(SetForegroundColor(Color::Magenta))?
                .queue(Print("·"))?;
        }
    }
    Ok(())
}

fn draw_scan_line(stdout: &mut io::Stdout, center_x: u16, center_y: u16, radius: u16, angle: f64) -> io::Result<()> {
    // scanning--line
    for r in 1..=radius {
        let x = center_x as f64 + (r as f64 * angle.cos());
        let y = center_y as f64 + (r as f64 * angle.sin() * 0.5); // Multiply by 0.5 for oval effect
        
        stdout
            .queue(MoveTo(x as u16, y as u16))?
            .queue(SetForegroundColor(Color::Magenta))?
            .queue(Print("█"))?;
    }
    
    // bright--point at the end of the scanning-line
    let x = center_x as f64 + (radius as f64 * angle.cos());
    let y = center_y as f64 + (radius as f64 * angle.sin() * 0.5);
    stdout
        .queue(MoveTo(x as u16, y as u16))?
        .queue(SetForegroundColor(Color::White))?
        .queue(Print("●"))?;
    
    Ok(())
}