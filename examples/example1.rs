use fluid_parser::lexer::Lexer;
use fluid_parser::parser::Parser;

const TEST: &str = r#"# data file for the Fltk User Interface Designer (fluid)
version 1.0400
header_name {.h}
code_name {.cxx}
avoid_early_includes
class UserInterface {open
} {
  Function {make_window()} {open
  } {
    Fl_Window Hello {open
      xywh {821 256 400 344} type Double align 80 resizable visible
    } {
      Fl_Flex {} {open
        xywh {5 5 390 335} resizable gap 5 set_size_tuples {2  0 30  2 30 }
      } {
        Fl_Flex {} {open
          xywh {5 5 390 30} type HORIZONTAL gap 5 set_size_tuples {3  0 80  1 80  3 30 }
        } {
          Fl_Menu_Button {} {
            label menu open
            xywh {5 5 80 30}
          } {}
          Fl_Box {} {
            label {https://}
            xywh {90 5 80 30}
          }
          Fl_Input {} {
            xywh {175 5 185 30}
          }
          Fl_Button {} {
            label I
            xywh {365 5 30 30}
          }
        }
        Fl_Flex Nmae {open
          xywh {5 40 390 265} type HORIZONTAL gap 5
        } {
          Fl_Text_Display {} {
            xywh {5 40 390 265}
          }
        }
      }
    }
  }
}"#;

fn main() {
    let l = Lexer::new(TEST);
    let mut p = Parser::new(l);
    let a = p.parse();
    dbg!(a);
}
