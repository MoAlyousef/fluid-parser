use fluid_parser::lexer::Lexer;
use fluid_parser::parser::Parser;

const TEST: &str = r#"# data file for the Fltk User Interface Designer (fluid)
version 1.0400
header_name {.h}
code_name {.cxx}
Function {make_window()} {open
} {
  Fl_Window {} {open
    xywh {732 434 480 320} type Double visible
  } {
    Fl_Grid {} {open selected
      xywh {25 25 240 160}
      dimensions {3 3} margin {1 0 0 0} gap {1 1}
    } {
      Fl_Button {} {
        label Button
        xywh {26 25 85 60}
        parent_properties {
          location {0 0}
        }
      }
      Fl_Button {} {
        label Button
        xywh {179 126 86 59}
        parent_properties {
          location {2 2}
        }
      }
    }
  }
}"#;

fn main() {
    let l = Lexer::new(TEST);
    let mut p = Parser::new(l);
    let a = p.parse();
    println!("{:#?}", a);
}
