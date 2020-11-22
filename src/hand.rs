use crate::{practice_set::Finger, align};
use std::fmt;

#[derive(Debug)]
pub struct Hand {
    aligner: align::Left,
    selected: Option<Finger>,
}

impl Hand {
    pub const WIDTH: u16 = 35;

    pub fn new(aligner: align::Left) -> Self {
        Hand {
            aligner,
            selected: None,
        }
    }

    pub fn select(&mut self, finger: Option<Finger>) {
        self.selected = finger
    }
}

/*
    .-.                     .-.
  .-| |-.                 .-| |-.
  | | | |                 | | | |
.-| | | |                 | | | |-.
| | | | |                 | | | | |
| | | | |-.             .-| | | | |
| '     | |             | |     ` |
|       | |             | |       |
|         |             |         |
\         /             \         /
 |       |               |       |
 |       |               |       |
*/

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let finger_color = |displayed| match self.selected {
            Some(finger) if displayed == finger => {
                termion::color::Fg(termion::color::Red).to_string()
            }
            _ => termion::style::Reset.to_string(),
        };

        write!(
            f,
            "{align}{xx}  {xx}  {lm}.-{lm}.{xx}                   {xx}  {rm}.-{rm}.{xx}
{align}{xx}  {lr}.-{lm}| {lm}|{li}-.                 {ri}.-{rm}| {rm}|{rr}-{rr}.{xx}
{align}{xx}  {lr}| {lr}| {li}|{li} |                 {ri}| {ri}| {rr}|{rr} {rr}|{xx}
{align}{lp}.-{xx}| {xx}| {xx}|{xx} |                 {xx}| {xx}| {xx}|{xx} {xx}|{rp}-.
{align}{lp}| {lp}| {xx}| {xx}|{xx} |                 {xx}| {xx}| {xx}|{xx} {rp}|{rp} |
{align}{xx}| {xx}| {xx}| {xx}|{xx} |-.             .-{xx}| {xx}| {xx}|{xx} {xx}|{xx} |
{align}{xx}| {xx}' {xx}  {xx} {xx} | |             | {xx}| {xx}  {xx} {xx} {xx}`{xx} |
{align}{xx}| {xx}  {xx}  {xx} {xx} | |             | {xx}| {xx}  {xx} {xx} {xx} {xx} |
{align}{xx}| {xx}  {xx}  {xx} {xx}   |             | {xx}  {xx}  {xx} {xx} {xx} {xx} |
{align}{xx}\\{xx}  {xx}  {xx} {xx}    /             \\{xx} {xx}  {xx} {xx} {xx} {xx}   /
{align}{xx} |{xx}  {xx}  {xx} {xx}  |               |{xx}  {xx}  {xx} {xx} {xx} {xx}|
{align}{xx} |{xx}  {xx}  {xx} {xx}  |               |{xx}  {xx}  {xx} {xx} {xx} {xx}|",
            lp = finger_color(Finger::LeftPinky),
            lr = finger_color(Finger::LeftRing),
            lm = finger_color(Finger::LeftMiddle),
            li = finger_color(Finger::LeftIndex),
            ri = finger_color(Finger::RightIndex),
            rm = finger_color(Finger::RightMiddle),
            rr = finger_color(Finger::RightRing),
            rp = finger_color(Finger::RightPinky),
            xx = termion::style::Reset,
            align = self.aligner,
        )
    }
}
