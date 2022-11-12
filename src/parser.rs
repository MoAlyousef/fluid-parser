use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    l: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { l: lexer }
    }
    pub fn parse(&mut self) -> Ast {
        let mut a = Ast::default();
        let mut t = Token::default();
        while t.typ != TokenType::Eof {
            t = self.l.next_tok();
            if t.typ == TokenType::Eof {
                break;
            }

            match t.typ {
                TokenType::Eof => break,
                TokenType::Word => match t.word {
                    "version" => a.version = self.l.next_tok().word.parse().unwrap(),
                    "i18n_type" => { 
                        self.l.next_tok();
                        a.i18n_type = Some(true);
                        self.l.next_tok();
                    }
                    "header_name" => {
                        self.l.next_tok();
                        a.header_name = consume_braced_string(&mut self.l);
                    }
                    "code_name" => {
                        self.l.next_tok();
                        a.code_name = consume_braced_string(&mut self.l);
                    }
                    "class" => {
                        let c = self.consume_class();
                        a.classes.push(c);
                    }
                    "Function" => {
                        let f = self.consume_func();
                        a.functions.push(f);
                    }
                    "comment" => {
                        let c = self.consume_comment();
                        a.comments.push(c);
                    }
                    "decl" => {
                        let d = self.consume_decl();
                        a.decls.push(d);
                    }
                    _ => (),
                },
                _ => continue,
            }
        }
        a
    }
    fn consume_func(&mut self) -> Function {
        let mut f = Function::default();
        self.l.next_tok();
        let mut t = self.l.next_tok();
        f.name = consume_word(&t);
        self.l.next_tok(); // closing parens of function name
        self.l.next_tok(); // opening parens of props
        while t.typ != TokenType::Eof {
            t = self.l.next_tok();
            if t.typ == TokenType::CloseBrace {
                break;
            }
            match t.word {
                "open" => f.props.open = Some(true),
                "C" => f.props.c = Some(true),
                "protected" => f.props.visibility = Some(Visibility::PROTECTED),
                "private" => f.props.visibility = Some(Visibility::PRIVATE),
                "comment" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        f.props.comment = Some(consume_braced_string(&mut self.l));
                    } else {
                        f.props.comment = Some(consume_word(&t));
                    }
                }
                "return_type" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        f.props.return_type = Some(consume_braced_string(&mut self.l));
                    } else {
                        f.props.return_type = Some(consume_word(&t));
                    }
                }
                _ => (),
            }
        }
        t = self.l.next_tok(); // close props parens
        if t.typ == TokenType::OpenBrace {
            let mut openbrace = 1;
            while t.typ != TokenType::Eof {
                t = self.l.next_tok();
                if t.typ == TokenType::OpenBrace {
                    openbrace += 1;
                }
                if t.typ == TokenType::CloseBrace {
                    openbrace -= 1;
                }
                if openbrace == 0 {
                    break;
                }
                if t.word.starts_with("Fl_") || t.word == "MenuItem" || t.word == "Submenu" {
                    let mut w = self.consume_widget();
                    w.typ = consume_word(&t);
                    f.widgets.push(w);
                }
                if t.word == "code" {
                    self.l.next_tok();
                    f.code = Some(self.consume_code());
                }
            }
        }
        f
    }
    fn consume_widget(&mut self) -> Widget {
        let mut w = Widget::default();
        let mut t = self.l.next_tok();
        if t.typ == TokenType::OpenBrace {
            t = self.l.next_tok();
        }
        if !t.word.is_empty() {
            w.name = consume_word(&t);
        } else {
            w.name = String::new();
        }
        while t.typ != TokenType::Eof {
            t = self.l.next_tok();
            if t.typ == TokenType::CloseBrace {
                break;
            }
            match t.word {
                "open" => w.props.open = Some(true),
                "hide" => w.props.hide = Some(true),
                "deactivate" => w.props.deactivate = Some(true),
                "divider" => w.props.divider = Some(true),
                "resizable" => w.props.resizable = Some(true),
                "visible" => w.props.visible = Some(true),
                "hotspot" => w.props.hotspot = Some(true),
                "xywh" => {
                    self.l.next_tok();
                    w.props.xywh = consume_braced_string(&mut self.l);
                }
                "color" => {
                    t = self.l.next_tok();
                    w.props.color = Some(consume_word(&t).parse().unwrap());
                }
                "selection_color" => {
                    t = self.l.next_tok();
                    w.props.selection_color = Some(consume_word(&t).parse().unwrap());
                }
                "labelcolor" => {
                    t = self.l.next_tok();
                    w.props.labelcolor = Some(consume_word(&t).parse().unwrap());
                }
                "textcolor" => {
                    t = self.l.next_tok();
                    w.props.textcolor = Some(consume_word(&t).parse().unwrap());
                }
                "type" => {
                    t = self.l.next_tok();
                    w.props.typ = Some(consume_word(&t));
                }
                "labeltype" => {
                    t = self.l.next_tok();
                    w.props.labeltype = Some(consume_word(&t));
                }
                "labelfont" => {
                    t = self.l.next_tok();
                    w.props.labelfont = Some(consume_word(&t).parse().unwrap());
                }
                "textfont" => {
                    t = self.l.next_tok();
                    w.props.textfont = Some(consume_word(&t).parse().unwrap());
                }
                "labelsize" => {
                    t = self.l.next_tok();
                    w.props.labelsize = Some(consume_word(&t).parse().unwrap());
                }
                "textsize" => {
                    t = self.l.next_tok();
                    w.props.textsize = Some(consume_word(&t).parse().unwrap());
                }
                "box" => {
                    self.l.next_tok();
                    w.props.r#box = Some(consume_word(&t));
                }
                "down_box" => {
                    self.l.next_tok();
                    w.props.down_box = Some(consume_word(&t));
                }
                "align" => {
                    t = self.l.next_tok();
                    w.props.align = Some(consume_word(&t).parse().unwrap());
                }
                "when" => {
                    t = self.l.next_tok();
                    w.props.when = Some(consume_word(&t).parse().unwrap());
                }
                "shortcut" => {
                    t = self.l.next_tok();
                    w.props.shortcut = Some(consume_word(&t).parse().unwrap());
                }
                "gap" => {
                    t = self.l.next_tok();
                    w.props.gap = Some(consume_word(&t).parse().unwrap());
                }
                "minimum" => {
                    t = self.l.next_tok();
                    w.props.minimum = Some(consume_word(&t).parse().unwrap());
                }
                "maximum" => {
                    t = self.l.next_tok();
                    w.props.maximum = Some(consume_word(&t).parse().unwrap());
                }
                "step" => {
                    t = self.l.next_tok();
                    w.props.step = Some(consume_word(&t).parse().unwrap());
                }
                "slider_size" => {
                    t = self.l.next_tok();
                    w.props.slider_size = Some(consume_word(&t).parse().unwrap());
                }
                "size" => {
                    t = self.l.next_tok();
                    w.props.size = Some(consume_word(&t).parse().unwrap());
                }
                "label" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.label = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.label = Some(consume_word(&t));
                    }
                }
                "class" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.class = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.class = Some(consume_word(&t));
                    }
                }
                "tooltip" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.tooltip = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.tooltip = Some(consume_word(&t));
                    }
                }
                "image" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.image = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.image = Some(consume_word(&t));
                    }
                }
                "deimage" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.deimage = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.deimage = Some(consume_word(&t));
                    }
                }
                "value" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.value = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.value = Some(consume_word(&t));
                    }
                }
                "set_size_tuples" => {
                    self.l.next_tok();
                    w.props.size_tuple = Some(consume_braced_string(&mut self.l));
                }
                "code0" => {
                    self.l.next_tok();
                    w.props.code0 = Some(consume_braced_string(&mut self.l));
                }
                "code1" => {
                    self.l.next_tok();
                    w.props.code1 = Some(consume_braced_string(&mut self.l));
                }
                "code2" => {
                    self.l.next_tok();
                    w.props.code2 = Some(consume_braced_string(&mut self.l));
                }
                "code3" => {
                    self.l.next_tok();
                    w.props.code3 = Some(consume_braced_string(&mut self.l));
                }
                "extra_code" => {
                    self.l.next_tok();
                    w.props.extra_code = Some(consume_braced_string(&mut self.l));
                }
                "callback" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.callback = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.callback = Some(consume_word(&t));
                    }
                }
                "user_data" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.user_data = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.user_data = Some(consume_word(&t));
                    }
                }
                "user_data_type" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.user_data_type = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.user_data_type = Some(consume_word(&t));
                    }
                }
                "comment" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        w.props.comment = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.comment = Some(consume_word(&t));
                    }
                }
                _ => (),
            }
        }
        t = self.l.next_tok();
        // We have children
        if t.typ == TokenType::OpenBrace {
            let mut openbrace = 1;
            while t.typ != TokenType::Eof {
                t = self.l.next_tok();
                if t.typ == TokenType::OpenBrace {
                    openbrace += 1;
                }
                if t.typ == TokenType::CloseBrace {
                    openbrace -= 1;
                }
                if openbrace == 0 {
                    break;
                }
                if t.word.starts_with("Fl_") || t.word == "MenuItem" {
                    let mut c = self.consume_widget();
                    c.typ = consume_word(&t);
                    if w.children.as_ref().is_none() {
                        w.children = Some(vec![]);
                    }
                    w.children.as_mut().unwrap().push(c);
                }
            }
        }
        w
    }
    fn consume_class(&mut self) -> Class {
        let mut c = Class::default();
        let mut t = self.l.next_tok();
        if t.typ == TokenType::OpenBrace {
            t = self.l.next_tok();
            self.l.next_tok();
        }
        c.name = consume_word(&t);
        self.l.next_tok();
        // handle props
        while t.typ != TokenType::Eof {
            t = self.l.next_tok();
            if t.typ == TokenType::CloseBrace {
                break;
            }
            match t.word {
                "open" => c.props.open = Some(true),
                "protected" => c.props.visibility = Some(Visibility::PROTECTED),
                "private" => c.props.visibility = Some(Visibility::PRIVATE),
                "comment" => {
                    t = self.l.next_tok();
                    if t.typ == TokenType::OpenBrace {
                        c.props.comment = Some(consume_braced_string(&mut self.l));
                    } else {
                        c.props.comment = Some(consume_word(&t));
                    }
                }
                _ => (),
            }
        }
        t = self.l.next_tok();
        if t.typ == TokenType::OpenBrace {
            let mut openbrace = 1;
            while t.typ != TokenType::Eof {
                t = self.l.next_tok();
                if t.typ == TokenType::OpenBrace {
                    openbrace += 1;
                }
                if t.typ == TokenType::CloseBrace {
                    openbrace -= 1;
                }
                if openbrace == 0 {
                    break;
                }
                match t.word {
                    "Function" => {
                        let f = self.consume_func();
                        c.functions.push(f);
                    }
                    "comment" => {
                        t = self.l.next_tok();
                        if t.typ == TokenType::OpenBrace {
                            c.props.comment = Some(consume_braced_string(&mut self.l));
                        } else {
                            c.props.comment = Some(consume_word(&t));
                        }
                    }
                    _ => (),
                }
            }
        }
        c
    }
    fn consume_comment(&mut self) -> Comment {
        let mut c = Comment::default();
        let mut t = self.l.next_tok();
        c.comment = consume_braced_string(&mut self.l);
        while t.typ != TokenType::Eof {
            t = self.l.next_tok();
            if t.typ == TokenType::CloseBrace {
                break;
            }
            match t.word {
                "in_source" => c.props.in_source = Some(true),
                "in_header" => c.props.in_header = Some(true),
                _ => (),
            }
        }
        c
    }
    fn consume_decl(&mut self) -> Decl {
        let mut d = Decl::default();
        let mut t = self.l.next_tok();
        d.decl = consume_braced_string(&mut self.l);
        while t.typ != TokenType::Eof {
            t = self.l.next_tok();
            if t.typ == TokenType::CloseBrace {
                break;
            }
            match t.word {
                "private" => d.props.visibility = Visibility::PRIVATE,
                "public" => d.props.visibility = Visibility::PUBLIC,
                "global" => d.props.global = Some(true),
                "local" => d.props.local = Some(true),
                _ => (),
            }
        }
        d
    }
    fn consume_code(&mut self) -> String {
        let s = consume_braced_string(&mut self.l);
        // skip last 2 braces
        self.l.next_tok();
        self.l.next_tok();
        s
    }
}

fn consume_word(t: &Token) -> String {
    t.word.to_string()
}

fn consume_braced_string(l: &mut Lexer) -> String {
    let mut t = l.next_tok();
    let start = t.start;
    let mut openbrace = 1;
    while t.typ != TokenType::Eof {
        t = l.next_tok();
        if t.typ == TokenType::OpenBrace {
            openbrace += 1;
        }
        if t.typ == TokenType::CloseBrace {
            openbrace -= 1;
        }
        if openbrace == 0 {
            break;
        }
    }
    let end = t.end - 1;
    l.s[start..end].to_string()
}
