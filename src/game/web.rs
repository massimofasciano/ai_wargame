use std::fmt::Write as FmtWrite;
use std::fmt::Result as FmtResult;

use crate::{Coord, Game};

impl Game {
    pub fn to_html_board_string(&self, css_class: String) -> String {
        let mut html = String::new();
        self.to_html_board_writer(&mut html, css_class).expect("write to string can't fail");
        html
    }
    pub fn to_html_board_writer(&self, w : &mut impl FmtWrite, css_class: String) -> FmtResult {
        let half = (self.dim()+1) / 2;
        write!(w,"<table class=\"{css_class}\">")?;
        write!(w,"<thead>")?;
        write!(w,"<tr>")?;
        write!(w,"<th colspan={half} class=\"{css_class}_info\">")?;
        write!(w,"<div class=\"{css_class}_info_moves\">")?;
        if let Some(max_moves) = self.options.max_moves {
            if self.total_moves() >= max_moves {
                write!(w,"maximum moves played ({})",max_moves)?;
            } else {
                write!(w,"{}/{} moves played",self.total_moves(),max_moves)?;
            }
        } else {
            write!(w,"{} moves played",self.total_moves())?;
        }
        write!(w,"</div>")?;
        write!(w,"</th>")?;
        write!(w,"<th colspan={half} class=\"{css_class}_info\">")?;
        write!(w,"<div class=\"{css_class}_info_next\">")?;
        write!(w,"Next player: {}",self.player())?;
        write!(w,"</div>")?;
        write!(w,"</th>")?;
        write!(w,"</tr>")?;
        write!(w,"<tr>")?;
        write!(w,"<th class=\"{css_class}_row_name\"></th>")?;
        for col in 0..self.dim() {
            write!(w,"<th class=\"{css_class}_col_name\">{}</th>",col)?;
        }
        write!(w,"</tr>")?;
        write!(w,"</thead>")?;
        write!(w,"<tbody>")?;
        for row in 0..self.dim() {
            write!(w,"<tr>")?;
            write!(w,"<th class=\"{}_row_name\">{}</th>",css_class,(row as u8 +'A' as u8) as char)?;
            for col in 0..self.dim() {
                let cell = self[Coord::new(row,col)];
                write!(w,"<td class=\"{css_class}_cell\">")?;
                if !cell.is_empty() {
                    let player = cell.player().expect("cell not empty");
                    let unit = cell.unit().expect("cell not empty");
                    let health = unit.health;
                    let unit_type = unit.unit_type;
                    write!(w,"<div>")?;
                    write!(w,"<span class=\"{css_class}_{player}\">{player}</span>")?;
                    write!(w,"<br>")?;
                    write!(w,"<div>")?;
                    write!(w,"<span>{unit_type}</span>")?;
                    write!(w,"<span>:</span>")?;
                    write!(w,"<span>{health}</span>")?;
                    write!(w,"</div>")?;
                    write!(w,"</div>")?;
                };
                write!(w,"</td>")?;
            }
            write!(w,"</tr>")?;
        }
        write!(w,"</tbody>")?;
        write!(w,"</table>")
    }
}