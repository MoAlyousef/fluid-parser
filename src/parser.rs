use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    l: Lexer<'a>,
    t: Token<'a>,
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            l: lexer,
            t: Token::default(),
            tokens: vec![],
        }
    }
    pub fn next(&mut self) {
        self.t = self.l.next();
        dbg!(&self.t);
        self.tokens.push(self.t);
    }
    pub fn debug(&self) {
        dbg!(&self.t);
    }
    pub fn parse(&mut self) -> Ast {
        let mut a = Ast::default();
        while self.t.typ != TokenType::Eof {
            self.next();
            if self.t.typ == TokenType::Eof {
                break;
            }

            match self.t.typ {
                TokenType::Eof => break,
                TokenType::Word => match self.t.word {
                    "version" => a.version = {
                        self.next();
                        self.t.word.parse().unwrap()
                    },
                    "i18n_type" => {
                        self.next();
                        a.i18n_type = Some(true);
                        self.next();
                    }
                    "header_name" => {
                        self.next();
                        a.header_name = consume_braced_string(&mut self.l);
                    }
                    "code_name" => {
                        self.next();
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
        dbg!("func");
        let mut f = Function::default();
        self.next();
        self.next();
        f.name = consume_word(&self.t);
        self.next(); // closing parens of function name
        self.next(); // opening parens of props
        while self.t.typ != TokenType::Eof {
            self.next();
            if self.t.typ == TokenType::CloseBrace {
                break;
            }
            match self.t.word {
                "open" => f.props.open = Some(true),
                "C" => f.props.c = Some(true),
                "protected" => f.props.visibility = Some(Visibility::PROTECTED),
                "private" => f.props.visibility = Some(Visibility::PRIVATE),
                "comment" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        f.props.comment = Some(consume_braced_string(&mut self.l));
                    } else {
                        f.props.comment = Some(consume_word(&self.t));
                    }
                }
                "return_type" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        f.props.return_type = Some(consume_braced_string(&mut self.l));
                    } else {
                        f.props.return_type = Some(consume_word(&self.t));
                    }
                }
                _ => (),
            }
        }
        self.next(); // close props parens
        if self.t.typ == TokenType::OpenBrace {
            while self.t.typ != TokenType::CloseBrace {
                self.next();
                if self.t.word.starts_with("Fl_")
                    || self.t.word == "MenuItem"
                    || self.t.word == "Submenu"
                {
                    let w = self.consume_widget();
                    f.widgets.push(w);
                }
                if self.t.word == "code" {
                    self.next();
                    f.code = Some(self.consume_code());
                }
            }
        }
        f
    }
    fn consume_widget(&mut self) -> Widget {
        dbg!("wid");
        let mut w = Widget::default();
        w.typ = consume_word(&self.t);
        self.next();
        if self.t.typ == TokenType::OpenBrace {
            self.next();
        }
        if !self.t.word.is_empty() {
            w.name = consume_word(&self.t);
        } else {
            w.name = String::new();
        }
        while self.t.typ != TokenType::Eof {
            self.next();
            if self.t.typ == TokenType::CloseBrace {
                break;
            }
            match self.t.word {
                "open" => w.props.open = Some(true),
                "hide" => w.props.hide = Some(true),
                "deactivate" => w.props.deactivate = Some(true),
                "divider" => w.props.divider = Some(true),
                "resizable" => w.props.resizable = Some(true),
                "visible" => w.props.visible = Some(true),
                "hotspot" => w.props.hotspot = Some(true),
                "xywh" => {
                    self.next();
                    w.props.xywh = consume_braced_string(&mut self.l);
                }
                "color" => {
                    self.next();
                    w.props.color = Some(consume_word(&self.t).parse().unwrap());
                }
                "selection_color" => {
                    self.next();
                    w.props.selection_color = Some(consume_word(&self.t).parse().unwrap());
                }
                "labelcolor" => {
                    self.next();
                    w.props.labelcolor = Some(consume_word(&self.t).parse().unwrap());
                }
                "textcolor" => {
                    self.next();
                    w.props.textcolor = Some(consume_word(&self.t).parse().unwrap());
                }
                "type" => {
                    self.next();
                    w.props.typ = Some(consume_word(&self.t));
                }
                "labeltype" => {
                    self.next();
                    w.props.labeltype = Some(consume_word(&self.t));
                }
                "labelfont" => {
                    self.next();
                    w.props.labelfont = Some(consume_word(&self.t).parse().unwrap());
                }
                "textfont" => {
                    self.next();
                    w.props.textfont = Some(consume_word(&self.t).parse().unwrap());
                }
                "labelsize" => {
                    self.next();
                    w.props.labelsize = Some(consume_word(&self.t).parse().unwrap());
                }
                "textsize" => {
                    self.next();
                    w.props.textsize = Some(consume_word(&self.t).parse().unwrap());
                }
                "box" => {
                    self.next();
                    w.props.r#box = Some(consume_word(&self.t));
                }
                "down_box" => {
                    self.next();
                    w.props.down_box = Some(consume_word(&self.t));
                }
                "align" => {
                    self.next();
                    w.props.align = Some(consume_word(&self.t).parse().unwrap());
                }
                "when" => {
                    self.next();
                    w.props.when = Some(consume_word(&self.t).parse().unwrap());
                }
                "shortcut" => {
                    self.next();
                    w.props.shortcut = Some(consume_word(&self.t).parse().unwrap());
                }
                "gap" => {
                    self.next();
                    w.props.gap = Some(consume_word(&self.t).parse().unwrap());
                }
                "minimum" => {
                    self.next();
                    w.props.minimum = Some(consume_word(&self.t).parse().unwrap());
                }
                "maximum" => {
                    self.next();
                    w.props.maximum = Some(consume_word(&self.t).parse().unwrap());
                }
                "step" => {
                    self.next();
                    w.props.step = Some(consume_word(&self.t).parse().unwrap());
                }
                "slider_size" => {
                    self.next();
                    w.props.slider_size = Some(consume_word(&self.t).parse().unwrap());
                }
                "size" => {
                    self.next();
                    w.props.size = Some(consume_word(&self.t).parse().unwrap());
                }
                "label" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.label = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.label = Some(consume_word(&self.t));
                    }
                }
                "class" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.class = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.class = Some(consume_word(&self.t));
                    }
                }
                "tooltip" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.tooltip = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.tooltip = Some(consume_word(&self.t));
                    }
                }
                "image" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.image = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.image = Some(consume_word(&self.t));
                    }
                }
                "deimage" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.deimage = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.deimage = Some(consume_word(&self.t));
                    }
                }
                "value" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.value = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.value = Some(consume_word(&self.t));
                    }
                }
                "set_size_tuples" => {
                    self.next();
                    w.props.size_tuple = Some(consume_braced_string(&mut self.l));
                }
                "code0" => {
                    self.next();
                    w.props.code0 = Some(consume_braced_string(&mut self.l));
                }
                "code1" => {
                    self.next();
                    w.props.code1 = Some(consume_braced_string(&mut self.l));
                }
                "code2" => {
                    self.next();
                    w.props.code2 = Some(consume_braced_string(&mut self.l));
                }
                "code3" => {
                    self.next();
                    w.props.code3 = Some(consume_braced_string(&mut self.l));
                }
                "extra_code" => {
                    self.next();
                    w.props.extra_code = Some(consume_braced_string(&mut self.l));
                }
                "callback" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.callback = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.callback = Some(consume_word(&self.t));
                    }
                }
                "user_data" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.user_data = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.user_data = Some(consume_word(&self.t));
                    }
                }
                "user_data_type" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.user_data_type = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.user_data_type = Some(consume_word(&self.t));
                    }
                }
                "comment" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        w.props.comment = Some(consume_braced_string(&mut self.l));
                    } else {
                        w.props.comment = Some(consume_word(&self.t));
                    }
                }
                _ => (),
            }
        }
        self.next();
        // self.debug();
        // We have children
        if self.t.typ == TokenType::OpenBrace {
            while self.t.typ != TokenType::CloseBrace {
                self.next();
                if self.t.word.starts_with("Fl_")
                    || self.t.word == "MenuItem"
                    || self.t.word == "Submenu"
                {
                    let c = self.consume_widget();
                    w.children.push(c);
                }
            }
        }
        w
    }
    fn consume_class(&mut self) -> Class {
        dbg!("class");
        let mut c = Class::default();
        self.next();
        if self.t.typ == TokenType::OpenBrace {
            self.next();
            self.next();
        }
        c.name = consume_word(&self.t);
        self.next();
        // handle props
        while self.t.typ != TokenType::CloseBrace {
            self.next();
            match self.t.word {
                "open" => c.props.open = Some(true),
                "protected" => c.props.visibility = Some(Visibility::PROTECTED),
                "private" => c.props.visibility = Some(Visibility::PRIVATE),
                "comment" => {
                    self.next();
                    if self.t.typ == TokenType::OpenBrace {
                        c.props.comment = Some(consume_braced_string(&mut self.l));
                    } else {
                        c.props.comment = Some(consume_word(&self.t));
                    }
                }
                _ => (),
            }
        }
        self.next();
        if self.t.typ == TokenType::OpenBrace {
            while self.t.typ != TokenType::CloseBrace {
                self.next();
                match self.t.word {
                    "Function" => {
                        let f = self.consume_func();
                        c.functions.push(f);
                    }
                    "comment" => {
                        self.next();
                        if self.t.typ == TokenType::OpenBrace {
                            c.props.comment = Some(consume_braced_string(&mut self.l));
                        } else {
                            c.props.comment = Some(consume_word(&self.t));
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
        self.next();
        c.comment = consume_braced_string(&mut self.l);
        while self.t.typ != TokenType::Eof {
            self.next();
            if self.t.typ == TokenType::CloseBrace {
                break;
            }
            match self.t.word {
                "in_source" => c.props.in_source = Some(true),
                "in_header" => c.props.in_header = Some(true),
                _ => (),
            }
        }
        c
    }
    fn consume_decl(&mut self) -> Decl {
        let mut d = Decl::default();
        self.next();
        d.decl = consume_braced_string(&mut self.l);
        while self.t.typ != TokenType::Eof {
            self.next();
            if self.t.typ == TokenType::CloseBrace {
                break;
            }
            match self.t.word {
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
        self.next();
        self.next();
        s
    }
}

fn consume_word(t: &Token) -> String {
    t.word.to_string()
}

fn consume_braced_string(l: &mut Lexer) -> String {
    let mut t = l.next();
    let start = t.start;
    let mut openbrace = 1;
    while t.typ != TokenType::Eof {
        t = l.next();
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
