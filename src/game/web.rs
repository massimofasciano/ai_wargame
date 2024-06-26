use std::fmt::Write as FmtWrite;
use std::fmt::Result as FmtResult;

use crate::UnitType;
use crate::{Coord, Game};

impl Game {
    pub fn to_html_board_string(&self, css_class: String, id: String, fn_click: String) -> String {
        let mut html = String::new();
        self.to_html_board_writer(&mut html, css_class, id, fn_click).expect("write to string can't fail");
        html
    }
    pub fn to_html_board_writer(&self, w : &mut impl FmtWrite, css_class: String, id: String, fn_click: String) -> FmtResult {
        let half = (self.dim()+1) / 2;
        write!(w,"<table id=\"{id}\" class=\"{css_class}\">")?;
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
            write!(w,"<th class=\"{}_row_name\">{}</th>",css_class,(row as u8 + b'A') as char)?;
            for col in 0..self.dim() {
                let cell = self[Coord::new(row,col)];
                write!(w,"<td id=\"{id}-{row}-{col}\" class=\"{css_class}_cell\" onclick=\"{fn_click}({row},{col})\">")?;
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
    fn html_th(s : &String) -> String {
        format!("<th>{s}</th>")
    }
    fn html_td(s : &String) -> String {
        format!("<td>{s}</td>")
    }
    pub fn html_damage_table_string(&self, legend: Option<&str>) -> String {
        let mut html = String::new();
        Self::html_table_writer(&mut html,
            UnitType::damage_table(legend,Self::html_th,Self::html_td))
            .expect("write to string can't fail");
        html
    }
    pub fn html_repair_table_string(&self, legend: Option<&str>) -> String {
        let mut html = String::new();
        Self::html_table_writer(&mut html,
            UnitType::repair_table(legend,Self::html_th,Self::html_td))
            .expect("write to string can't fail");
        html
    }
    pub fn html_table_writer(w : &mut impl FmtWrite, table: Vec<Vec<String>>) -> FmtResult {
        let cls = "stats-table";
        writeln!(w,"<table class=\"{cls}\">")?;
        for row in table {
            writeln!(w,"<tr>")?;
            for cell in row {
                write!(w,"{cell}")?;
            }
            writeln!(w,"</tr>")?;
        }
        writeln!(w,"</table>")?;
        Ok(())
    }
}