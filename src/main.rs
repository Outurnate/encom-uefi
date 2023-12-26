#![no_main]
#![no_std]

extern crate alloc;

mod cast;

use crate::cast::asciicast;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use embedded_canvas::Canvas;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::{MonoTextStyleBuilder, MonoFont, MonoTextStyle};
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Rectangle, PrimitiveStyle};
use embedded_graphics::text::{Text, Alignment};
use embedded_text::TextBox;
use embedded_text::alignment::HorizontalAlignment;
use embedded_text::style::TextBoxStyleBuilder;
use embedded_vintage_fonts::FONT_12X16;
use tinybmp::Bmp;
use uefi::{prelude::*, Error};
use uefi::proto::console::gop::{BltPixel, GraphicsOutput, BltOp, BltRegion};
use uefi::table::boot::ScopedProtocol;
use unwrap_infallible::UnwrapInfallible;

struct Graphics<'a>
{
  gop: ScopedProtocol<'a, GraphicsOutput>,
  width: usize,
  height: usize,
  buffer: Vec<BltPixel>
}

impl<'a> Graphics<'a>
{
  fn new(bt: &'a BootServices) -> Graphics<'a>
  {
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let gop = bt.open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();
    let (width, height) = gop.current_mode_info().resolution();
    Self { gop, width, height, buffer: vec![BltPixel::new(0, 0, 0); width * height] }
  }
}

impl<'a> DrawTarget for Graphics<'a>
{
  type Color = Rgb888;

  type Error = Error<()>;

  fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where I: IntoIterator<Item = Pixel<Self::Color>>
  {
    for pixel in pixels
    {
      if pixel.0.x >= 0 && pixel.0.x < self.width as i32 && pixel.0.y >= 0 && pixel.0.y < self.height as i32
      {
        let target_pixel = self.buffer.get_mut(pixel.0.y as usize * self.width + pixel.0.x as usize).unwrap();
        target_pixel.red = pixel.1.r();
        target_pixel.green = pixel.1.g();
        target_pixel.blue = pixel.1.b();
      }
    }

    self.gop.blt(BltOp::BufferToVideo
      {
        buffer: &self.buffer,
        src: BltRegion::Full,
        dest: (0, 0),
        dims: (self.width, self.height),
      })
  }
}

impl<'a> Dimensions for Graphics<'a>
{
  fn bounding_box(&self) -> Rectangle
  {
    Rectangle::new(Point::new(0, 0), Size::new(self.width as u32, self.height as u32))
  }
}

pub struct Terminal<'a>
{
  position: Point,
  width: u32,
  height: u32,
  text: String,
  font: &'a MonoFont<'a>
}

impl<'a> Terminal<'a>
{
  pub fn new(position: Point, width: u32, height: u32) -> Self
  {
    Self { position, width, height, text: String::new(), font: &FONT_10X20 }
  }

  pub fn o(&mut self, msg: &str)
  {
    self.text += msg;
    if self.text.len() > 3000
    {
      self.text = self.text[(self.text.len() - 3000)..].to_string();
    }
  }
}

impl<'a> Drawable for Terminal<'a>
{
  type Color = Rgb888;

  type Output = ();

  fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where D: DrawTarget<Color = Self::Color>
  {
    let pixel_size = Size::new(self.font.character_size.width * self.width, self.font.character_size.height * self.height);

    let text_canvas = {
      let text_style = MonoTextStyleBuilder::new()
        .font(&self.font)
        .text_color(Rgb888::WHITE)
        .background_color(Rgb888::BLACK)
        .build();
      let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Left)
        .build();
      let mut canvas = Canvas::<Self::Color>::new(pixel_size);
      let text_box = TextBox::with_textbox_style(&self.text, Rectangle::new(Point::zero(), pixel_size), text_style, textbox_style)
        .add_plugin(embedded_text::plugin::ansi::Ansi::new())
        .add_plugin(embedded_text::plugin::tail::Tail {});
      text_box.draw(&mut canvas).unwrap_infallible();
      canvas
    };

    let framed_canvas = {
      let border_thickness = 3;
      let header_height = self.font.character_size.height;

      let header = Rectangle::new(Point::zero(), Size::new(pixel_size.width + (border_thickness * 2), header_height))
        .into_styled(PrimitiveStyle::with_fill(Rgb888::WHITE));
      let border = Rectangle::new(Point::new(0, header_height as i32), Size::new(pixel_size.width + (border_thickness * 2), pixel_size.height + (border_thickness * 2)))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, border_thickness));

      let mut canvas = Canvas::<Self::Color>::new(Size::new(pixel_size.width + (border_thickness * 2), pixel_size.height + (border_thickness * 2) + header_height));
      canvas.clear(Rgb888::BLACK).unwrap_infallible();
      header.draw(&mut canvas).unwrap_infallible();
      border.draw(&mut canvas).unwrap_infallible();
      text_canvas.place_at(Point::new(border_thickness as i32, (border_thickness + header_height) as i32)).draw(&mut canvas).unwrap_infallible();
      canvas
    };

    framed_canvas.place_at(self.position).draw(target)
  }
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status
{
  uefi_services::init(&mut system_table).unwrap();
  let bt = system_table.boot_services();
  let mut display = Graphics::new(&bt);

  // draw logo
  let bmp_data = include_bytes!("encom.bmp");
  let bmp = Bmp::from_slice(bmp_data).unwrap();
  Image::new(&bmp, Point::new((display.bounding_box().size.width - bmp.bounding_box().size.width - 16) as i32, 16)).draw(&mut display).unwrap();

  // header text
  Text::new(
    include_str!("bios.txt"),
    Point::new(8, 24),
    MonoTextStyle::new(&embedded_vintage_fonts::FONT_12X16, Rgb888::WHITE)
  ).draw(&mut display).unwrap();

  // footer
  Text::new(
    "Press F1 to Run SETUP",
    Point::new(8, (display.bounding_box().size.height - 8) as i32),
    MonoTextStyle::new(&embedded_vintage_fonts::FONT_12X16, Rgb888::WHITE)
  ).draw(&mut display).unwrap();

  // terminal
  asciicast(Point::new(290, 240), &mut display, bt).unwrap();

  // final dialog
  let mut blink = false;
  loop
  {
    let dialog_bg = Rgb888::new(139, 128, 0);
    let selected_text_style = if blink
    {
      MonoTextStyleBuilder::new()
        .font(&FONT_12X16)
        .text_color(Rgb888::BLACK)
        .background_color(dialog_bg)
        .build()
    }
    else
    {
      MonoTextStyleBuilder::new()
        .font(&FONT_12X16)
        .text_color(Rgb888::WHITE)
        .background_color(Rgb888::BLACK)
        .build()
    };

    let dialog_width = 512;
    let dialog_height = 128;
    Rectangle::new(Point::new(((display.bounding_box().size.width / 2) - (dialog_width / 2)) as i32, ((display.bounding_box().size.height / 2) - (dialog_height / 2)) as i32), Size::new(dialog_width, dialog_height))
      .into_styled(PrimitiveStyle::with_fill(dialog_bg))
      .draw(&mut display).unwrap();

    Text::with_alignment(
      "Aperture Clear?",
      display.bounding_box().center() - Point::new(0, 28),
      MonoTextStyle::new(&FONT_12X16, Rgb888::BLACK),
      Alignment::Center
    ).draw(&mut display).unwrap();

    Text::with_alignment(
      "< Yes >",
      display.bounding_box().center() + Point::new(-64, 28),
      selected_text_style,
      Alignment::Center
    ).draw(&mut display).unwrap();

    Text::with_alignment(
      "< No >",
      display.bounding_box().center() + Point::new(64, 28),
      MonoTextStyle::new(&FONT_12X16, Rgb888::BLACK),
      Alignment::Center
    ).draw(&mut display).unwrap();

    blink = !blink;

    bt.stall(500000);
  }
}