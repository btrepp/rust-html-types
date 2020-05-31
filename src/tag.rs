use derive_more::Into;
/// A html tag definition. Most of the time you will want the static constants
/// defined on Tag, rather than constructing this yourself
#[derive(Into,Clone,Eq, PartialEq)]
pub struct Tag<'a>(&'a str);

/// Returned when the tag didn't contain valid characters
pub struct TagParseError();

macro_rules! tag {
    ($name:ident $tag:ident) => {
        pub const $name : Tag<'static> = Tag(stringify!($tag));
    };
}

impl<'a> Tag<'a> {

    /// Try to create the tag from a string
    /// Note this will currently fail if the tag is not alphabetic
    ///
    /// Prefer to use the contant forms if possible. However HTML can have custom
    /// tags, so we do need this possibility
    pub fn try_create<S>(text:S) -> Result<Self,TagParseError> 
        where S: Into<&'a str> {
        let string = text.into();
        match string.chars().all(char::is_alphabetic) {
            false => {
                Err(TagParseError())
            },
            true => {
                let tag = Tag(string);
                Ok(tag)
            }
        }
    }
}

impl Tag<'static> {

    /// Escape hatch for creating a tag in an external crate.
    /// NOTE: we can't verify correctness here, so take care
    /// (hence why it's marked as unsafe)
    pub const unsafe fn create_unsafe(tag:&'static str) -> Tag {
        Tag(tag)
    }

    // obtained from https://eastmanreference.com/complete-list-of-html-tags
    tag!(A a);
    tag!(ABBR abbr);
    tag!(ADDRESS address);
    tag!(AREA area);
    tag!(ARTICLE article);
    tag!(ASIDE aside);
    tag!(AUDIO audio);
    tag!(B b);
    tag!(BASE base);
    tag!(BDI bdi);
    tag!(BDO bdo);
    tag!(BLOCKQUOTE blockquote);
    tag!(BODY body);
    tag!(BR br);
    tag!(BUTTON button);
    tag!(CANVAS canvas);
    tag!(CAPTION caption);
    tag!(CITE cite);
    tag!(CODE code);
    tag!(COL col);
    tag!(COLGROUP colgroup);
    tag!(DATA data);
    tag!(DATALIST datalist);
    tag!(DD dd);
    tag!(DEL del);
    tag!(DETAILS details);
    tag!(DFN dfn);
    tag!(DIALOG dialog);
    tag!(DIV div);
    tag!(DL dl);
    tag!(DT dt);
    tag!(EM em);
    tag!(EMBED embed);
    tag!(FIELDSET fieldset);
    tag!(FIGURE figure);
    tag!(FOOTER footer);
    tag!(FORM form);
    tag!(H1 h1);
    tag!(H2 h2);
    tag!(H3 h3);
    tag!(H4 h4);
    tag!(H5 h5);
    tag!(H6 h6);
    tag!(HEAD head);
    tag!(HEADER header);
    tag!(HGROUP hgroup);
    tag!(HR hr);
    tag!(HTML html);
    tag!(I i);
    tag!(IFRAME iframe);
    tag!(IMG img);
    tag!(INPUT input);
    tag!(INS ins);
    tag!(KBD kbd);
    tag!(KEYGEN keygen);
    tag!(LABEL label);
    tag!(LEGEND legend);
    tag!(LI li);
    tag!(LINK link);
    tag!(MAIN main);
    tag!(MAP map);
    tag!(MARK mark);
    tag!(MENU menu);
    tag!(MENUITEM menuitem);
    tag!(META meta);
    tag!(METER meter);
    tag!(NAV nav);
    tag!(NOSCRIPT noscript);
    tag!(OBJECT object);
    tag!(OL ol);
    tag!(OPTGROUP optgroup);
    tag!(OPTION option);
    tag!(OUTPUT output);
    tag!(P p);
    tag!(PARAM param);
    tag!(PRE pre);
    tag!(PROGRESS progress);
    tag!(Q q);
    tag!(RB rb);
    tag!(RP rp);
    tag!(RT rt);
    tag!(RTC rtc);
    tag!(RUBY ruby);
    tag!(S s);
    tag!(SAMP samp);
    tag!(SCRIPT script);
    tag!(SECTION section);
    tag!(SELECT select);
    tag!(SMALL small);
    tag!(SOURCE source);
    tag!(SPAN span);
    tag!(STRONG strong);
    tag!(STYLE style);
    tag!(SUB sub);
    tag!(SUMMARY summary);
    tag!(SUP sup);
    tag!(TABLE table);
    tag!(TBODY tbody);
    tag!(TD td);
    tag!(TEMPLATE template);
    tag!(TEXTAREA textarea);
    tag!(TFOOT tfoot);
    tag!(TH th);
    tag!(THEAD thead);
    tag!(TIME time);
    tag!(TITLE title);
    tag!(TR tr);
    tag!(TRACK track);
    tag!(U u);
    tag!(UL ul);
    tag!(VAR var);
    tag!(VIDEO video);
    tag!(WBR wbr);
     
}