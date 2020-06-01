use derive_more::Display;
#[derive(Clone, Debug, Copy, PartialEq, Eq, Ord, PartialOrd, Display, Hash)]
pub struct Attribute<'a>(&'a str);
#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub struct Value(String);

pub struct InvalidKeyError {}

#[derive(Debug)]
pub struct InvalidValueError {}

impl Value {
    pub fn create<S>(str: S) -> Result<Value,InvalidValueError>
    where
        S: Into<String>,
    {
        let string: String = str.into();
        match string.chars().all(char::is_alphabetic) {
            true => Ok(Value(string)),
            false => Err (InvalidValueError{})
        }
    }

}

macro_rules! key {
    ($name:ident $tag:expr) => {
        pub const $name: Attribute<'static> = Attribute($tag);
    };
}

impl<'a> Attribute<'a> {
    /// Try to create a key from a string reference
    pub fn create(attribute: &'a str) -> Result<Attribute<'a>, InvalidKeyError> {
        match attribute.chars().all(char::is_alphabetic) {
            false => Err(InvalidKeyError {}),
            true => Ok(Attribute(attribute)),
        }
    }

    key!(ACCEPT "accept");
    key!(ACCEPT_CHARSET "accept-charset");
    key!(ACCESSKEY "accesskey");
    key!(ACTION "action");
    key!(ALIGN "align");
    key!(ALLOW "allow");
    key!(ALT "alt");
    key!(ASYNC "async");
    key!(AUTOCAPITALIZE "autocapitalize");
    key!(AUTOCOMPLETE "autocomplete");
    key!(AUTOFOCUS "autofocus");
    key!(AUTOPLAY "autoplay");
    key!(BACKGROUND "background");
    key!(BGCOLOR "bgcolor");
    key!(BORDER "border");
    key!(BUFFERED "buffered");
    key!(CAPTURE "capture");
    key!(CHALLENGE "challenge");
    key!(CHARSET "charset");
    key!(CHECKED "checked");
    key!(CITE "cite");
    key!(CLASS "class");
    key!(CODE "code");
    key!(CODEBASE "codebase");
    key!(COLOR "color");
    key!(COLS "cols");
    key!(COLSPAN "colspan");
    key!(CONTENT "content");
    key!(CONTENTEDITABLE "contenteditable");
    key!(CONTEXTMENU "contextmenu");
    key!(CONTROLS "controls");
    key!(COORDS "coords");
    key!(CROSSORIGIN "crossorigin");
    key!(CSP "csp");
    key!(DATA "data");
    key!(DATETIME "datetime");
    key!(DECODING "decoding");
    key!(DEFAULT "default");
    key!(DEFER "defer");
    key!(DIR "dir");
    key!(DIRNAME "dirname");
    key!(DISABLED "disabled");
    key!(DOWNLOAD "download");
    key!(DRAGGABLE "draggable");
    key!(DROPZONE "dropzone");
    key!(ENCTYPE "enctype");
    key!(ENTERKEYHINT "enterkeyhint");
    key!(FOR "for");
    key!(FORM "form");
    key!(FORMACTION "formaction");
    key!(FORMENCTYPE "formenctype");
    key!(FORMMETHOD "formmethod");
    key!(FORMNOVALIDATE "formnovalidate");
    key!(FORMTARGET "formtarget");
    key!(HEADERS "headers");
    key!(HEIGHT "height");
    key!(HIDDEN "hidden");
    key!(HIGH "high");
    key!(HREF "href");
    key!(HREFLANG "hreflang");
    key!(HTTP_EQUIV "http-equiv");
    key!(ICON "icon");
    key!(ID "id");
    key!(IMPORTANCE "importance");
    key!(INTEGRITY "integrity");
    key!(INTRINSICSIZE "intrinsicsize");
    key!(INPUTMODE "inputmode");
    key!(ISMAP "ismap");
    key!(ITEMPROP "itemprop");
    key!(KEYTYPE "keytype");
    key!(KIND "kind");
    key!(LABEL "label");
    key!(LANG "lang");
    key!(LANGUAGE "language");
    key!(LOADING "loading");
    key!(LIST "list");
    key!(LOOP "loop");
    key!(LOW "low");
    key!(MANIFEST "manifest");
    key!(MAX "max");
    key!(MAXLENGTH "maxlength");
    key!(MINLENGTH "minlength");
    key!(MEDIA "media");
    key!(METHOD "method");
    key!(MIN "min");
    key!(MULTIPLE "multiple");
    key!(MUTED "muted");
    key!(NAME "name");
    key!(NOVALIDATE "novalidate");
    key!(OPEN "open");
    key!(OPTIMUM "optimum");
    key!(PATTERN "pattern");
    key!(PING "ping");
    key!(PLACEHOLDER "placeholder");
    key!(POSTER "poster");
    key!(PRELOAD "preload");
    key!(RADIOGROUP "radiogroup");
    key!(READONLY "readonly");
    key!(REFERRERPOLICY "referrerpolicy");
    key!(REL "rel");
    key!(REQUIRED "required");
    key!(REVERSED "ol");
    key!(ROWS "rows");
    key!(ROWSPAN "rowspan");
    key!(SANDBOX "sandbox");
    key!(SCOPE "scope");
    key!(SCOPED "scoped");
    key!(SELECTED "selected");
    key!(SHAPE "shape");
    key!(SIZE "size");
    key!(SIZES "sizes");
    key!(SLOT "slot");
    key!(SPAN "span");
    key!(SPELLCHECK "spellcheck");
    key!(SRC "src");
    key!(SRCDOC "srcdoc");
    key!(SRCLANG "srclang");
    key!(SRCSET "srcset");
    key!(START "start");
    key!(STEP "step");
    key!(STYLE "style");
    key!(SUMMARY "summary");
    key!(TABINDEX "tabindex");
    key!(TARGET "target");
    key!(TITLE "title");
    key!(TRANSLATE "translate");
    key!(TYPE "type");
    key!(USEMAP "usemap");
    key!(VALUE "value");
    key!(WIDTH "width");
    key!(WRAP "wrap");
}
