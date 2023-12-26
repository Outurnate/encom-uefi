extern crate asciicast;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::LineWriter;
use std::io::Write;
use std::path::Path;

use asciicast::Entry;
use asciicast::EventType;
use asciicast::Header;

fn main()
{
  let file = File::create("src/cast.rs").unwrap();
  let mut file = LineWriter::new(file);
  writeln!(file, "use uefi::table::boot::BootServices;").unwrap();
  writeln!(file, "use embedded_graphics::{{Drawable, prelude::{{DrawTarget, Point}}, pixelcolor::Rgb888}};").unwrap();
  writeln!(file, "use crate::Terminal;").unwrap();
  writeln!(file, "pub fn asciicast<'a, D>(position: Point, display: &mut D, bt: &'a BootServices) -> Result<(), D::Error>").unwrap();
  writeln!(file, "  where D: DrawTarget<Color = Rgb888>").unwrap();
  writeln!(file, "{{").unwrap();
  if let Ok(mut lines) = read_lines("src/tron.cast")
  {
    let header = serde_json::from_str::<Header>(&lines.next().unwrap().unwrap()).unwrap();
    writeln!(file, "  let mut terminal = Terminal::new(position, {0}, {1});", header.width, header.height).unwrap();

    let mut time = 0.0;
    for entry in lines.flatten()
    {
      let entry = serde_json::from_str::<Entry>(&entry).unwrap();
      match entry.event_type
      {
        EventType::Input => todo!(),
        EventType::Output =>
        {
          writeln!(file, "  bt.stall({0});", ((entry.time - time) * 1000000.0) as usize).unwrap();
          writeln!(file, "  terminal.o(\"{0}\");", entry.event_data.escape_default()).unwrap();
          writeln!(file, "  terminal.draw(display)?;").unwrap();
          time = entry.time;
        }
      }
    }
  }
  writeln!(file, "  Ok(())").unwrap();
  writeln!(file, "}}").unwrap();
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/sample.cast");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}