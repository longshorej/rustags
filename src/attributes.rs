use core::*;

pub fn accept(value: &str) -> Fragment { attribute("accept", value) }
pub fn accept_charset(value: &str) -> Fragment { attribute("accept-charset", value) }
pub fn accesskey(value: &str) -> Fragment { attribute("accesskey", value) }
pub fn action(value: &str) -> Fragment { attribute("action", value) }
pub fn align(value: &str) -> Fragment { attribute("align", value) }
pub fn alt(value: &str) -> Fragment { attribute("alt", value) }
pub fn async(value: &str) -> Fragment { attribute("async", value) }
pub fn autocomplete(value: &str) -> Fragment { attribute("autocomplete", value) }
pub fn autofocus(value: &str) -> Fragment { attribute("autofocus", value) }
pub fn autoplay(value: &str) -> Fragment { attribute("autoplay", value) }
pub fn bgcolor(value: &str) -> Fragment { attribute("bgcolor", value) }
pub fn border(value: &str) -> Fragment { attribute("border", value) }
pub fn charset(value: &str) -> Fragment { attribute("charset", value) }
pub fn checked(value: &str) -> Fragment { attribute("checked", value) }
pub fn cite(value: &str) -> Fragment { attribute("cite", value) }
pub fn class(value: &str) -> Fragment { attribute("class", value) }
pub fn color(value: &str) -> Fragment { attribute("color", value) }
pub fn cols(value: &str) -> Fragment { attribute("cols", value) }
pub fn colspan(value: &str) -> Fragment { attribute("colspan", value) }
pub fn content(value: &str) -> Fragment { attribute("content", value) }
pub fn contenteditable(value: &str) -> Fragment { attribute("contenteditable", value) }
pub fn contextmenu(value: &str) -> Fragment { attribute("contextmenu", value) }
pub fn controls(value: &str) -> Fragment { attribute("controls", value) }
pub fn coords(value: &str) -> Fragment { attribute("coords", value) }
pub fn data(value: &str) -> Fragment { attribute("data", value) }
pub fn data_(value: &str) -> Fragment { attribute("data-*", value) }
pub fn datetime(value: &str) -> Fragment { attribute("datetime", value) }
pub fn default(value: &str) -> Fragment { attribute("default", value) }
pub fn defer(value: &str) -> Fragment { attribute("defer", value) }
pub fn dir(value: &str) -> Fragment { attribute("dir", value) }
pub fn dirname(value: &str) -> Fragment { attribute("dirname", value) }
pub fn disabled(value: &str) -> Fragment { attribute("disabled", value) }
pub fn download(value: &str) -> Fragment { attribute("download", value) }
pub fn draggable(value: &str) -> Fragment { attribute("draggable", value) }
pub fn dropzone(value: &str) -> Fragment { attribute("dropzone", value) }
pub fn enctype(value: &str) -> Fragment { attribute("enctype", value) }
pub fn for_(value: &str) -> Fragment { attribute("for", value) }
pub fn form(value: &str) -> Fragment { attribute("form", value) }
pub fn formaction(value: &str) -> Fragment { attribute("formaction", value) }
pub fn headers(value: &str) -> Fragment { attribute("headers", value) }
pub fn height(value: &str) -> Fragment { attribute("height", value) }
pub fn hidden(value: &str) -> Fragment { attribute("hidden", value) }
pub fn high(value: &str) -> Fragment { attribute("high", value) }
pub fn href(value: &str) -> Fragment { attribute("href", value) }
pub fn hreflang(value: &str) -> Fragment { attribute("hreflang", value) }
pub fn http_equiv(value: &str) -> Fragment { attribute("http-equiv", value) }
pub fn id(value: &str) -> Fragment { attribute("id", value) }
pub fn ismap(value: &str) -> Fragment { attribute("ismap", value) }
pub fn kind(value: &str) -> Fragment { attribute("kind", value) }
pub fn label(value: &str) -> Fragment { attribute("label", value) }
pub fn lang(value: &str) -> Fragment { attribute("lang", value) }
pub fn list(value: &str) -> Fragment { attribute("list", value) }
pub fn _loop(value: &str) -> Fragment { attribute("loop", value) }
pub fn low(value: &str) -> Fragment { attribute("low", value) }
pub fn max(value: &str) -> Fragment { attribute("max", value) }
pub fn maxlength(value: &str) -> Fragment { attribute("maxlength", value) }
pub fn media(value: &str) -> Fragment { attribute("media", value) }
pub fn method(value: &str) -> Fragment { attribute("method", value) }
pub fn min(value: &str) -> Fragment { attribute("min", value) }
pub fn multiple(value: &str) -> Fragment { attribute("multiple", value) }
pub fn muted(value: &str) -> Fragment { attribute("muted", value) }
pub fn name(value: &str) -> Fragment { attribute("name", value) }
pub fn novalidate(value: &str) -> Fragment { attribute("novalidate", value) }
pub fn onabort(value: &str) -> Fragment { attribute("onabort", value) }
pub fn onafterprint(value: &str) -> Fragment { attribute("onafterprint", value) }
pub fn onbeforeprint(value: &str) -> Fragment { attribute("onbeforeprint", value) }
pub fn onbeforeunload(value: &str) -> Fragment { attribute("onbeforeunload", value) }
pub fn onblur(value: &str) -> Fragment { attribute("onblur", value) }
pub fn oncanplay(value: &str) -> Fragment { attribute("oncanplay", value) }
pub fn oncanplaythrough(value: &str) -> Fragment { attribute("oncanplaythrough", value) }
pub fn onchange(value: &str) -> Fragment { attribute("onchange", value) }
pub fn onclick(value: &str) -> Fragment { attribute("onclick", value) }
pub fn oncontextmenu(value: &str) -> Fragment { attribute("oncontextmenu", value) }
pub fn oncopy(value: &str) -> Fragment { attribute("oncopy", value) }
pub fn oncuechange(value: &str) -> Fragment { attribute("oncuechange", value) }
pub fn oncut(value: &str) -> Fragment { attribute("oncut", value) }
pub fn ondblclick(value: &str) -> Fragment { attribute("ondblclick", value) }
pub fn ondrag(value: &str) -> Fragment { attribute("ondrag", value) }
pub fn ondragend(value: &str) -> Fragment { attribute("ondragend", value) }
pub fn ondragenter(value: &str) -> Fragment { attribute("ondragenter", value) }
pub fn ondragleave(value: &str) -> Fragment { attribute("ondragleave", value) }
pub fn ondragover(value: &str) -> Fragment { attribute("ondragover", value) }
pub fn ondragstart(value: &str) -> Fragment { attribute("ondragstart", value) }
pub fn ondrop(value: &str) -> Fragment { attribute("ondrop", value) }
pub fn ondurationchange(value: &str) -> Fragment { attribute("ondurationchange", value) }
pub fn onemptied(value: &str) -> Fragment { attribute("onemptied", value) }
pub fn onended(value: &str) -> Fragment { attribute("onended", value) }
pub fn onerror(value: &str) -> Fragment { attribute("onerror", value) }
pub fn onfocus(value: &str) -> Fragment { attribute("onfocus", value) }
pub fn onhashchange(value: &str) -> Fragment { attribute("onhashchange", value) }
pub fn oninput(value: &str) -> Fragment { attribute("oninput", value) }
pub fn oninvalid(value: &str) -> Fragment { attribute("oninvalid", value) }
pub fn onkeydown(value: &str) -> Fragment { attribute("onkeydown", value) }
pub fn onkeypress(value: &str) -> Fragment { attribute("onkeypress", value) }
pub fn onkeyup(value: &str) -> Fragment { attribute("onkeyup", value) }
pub fn onload(value: &str) -> Fragment { attribute("onload", value) }
pub fn onloadeddata(value: &str) -> Fragment { attribute("onloadeddata", value) }
pub fn onloadedmetadata(value: &str) -> Fragment { attribute("onloadedmetadata", value) }
pub fn onloadstart(value: &str) -> Fragment { attribute("onloadstart", value) }
pub fn onmousedown(value: &str) -> Fragment { attribute("onmousedown", value) }
pub fn onmousemove(value: &str) -> Fragment { attribute("onmousemove", value) }
pub fn onmouseout(value: &str) -> Fragment { attribute("onmouseout", value) }
pub fn onmouseover(value: &str) -> Fragment { attribute("onmouseover", value) }
pub fn onmouseup(value: &str) -> Fragment { attribute("onmouseup", value) }
pub fn onmousewheel(value: &str) -> Fragment { attribute("onmousewheel", value) }
pub fn onoffline(value: &str) -> Fragment { attribute("onoffline", value) }
pub fn ononline(value: &str) -> Fragment { attribute("ononline", value) }
pub fn onpagehide(value: &str) -> Fragment { attribute("onpagehide", value) }
pub fn onpageshow(value: &str) -> Fragment { attribute("onpageshow", value) }
pub fn onpaste(value: &str) -> Fragment { attribute("onpaste", value) }
pub fn onpause(value: &str) -> Fragment { attribute("onpause", value) }
pub fn onplay(value: &str) -> Fragment { attribute("onplay", value) }
pub fn onplaying(value: &str) -> Fragment { attribute("onplaying", value) }
pub fn onpopstate(value: &str) -> Fragment { attribute("onpopstate", value) }
pub fn onprogress(value: &str) -> Fragment { attribute("onprogress", value) }
pub fn onratechange(value: &str) -> Fragment { attribute("onratechange", value) }
pub fn onreset(value: &str) -> Fragment { attribute("onreset", value) }
pub fn onresize(value: &str) -> Fragment { attribute("onresize", value) }
pub fn onscroll(value: &str) -> Fragment { attribute("onscroll", value) }
pub fn onsearch(value: &str) -> Fragment { attribute("onsearch", value) }
pub fn onseeked(value: &str) -> Fragment { attribute("onseeked", value) }
pub fn onseeking(value: &str) -> Fragment { attribute("onseeking", value) }
pub fn onselect(value: &str) -> Fragment { attribute("onselect", value) }
pub fn onshow(value: &str) -> Fragment { attribute("onshow", value) }
pub fn onstalled(value: &str) -> Fragment { attribute("onstalled", value) }
pub fn onstorage(value: &str) -> Fragment { attribute("onstorage", value) }
pub fn onsubmit(value: &str) -> Fragment { attribute("onsubmit", value) }
pub fn onsuspend(value: &str) -> Fragment { attribute("onsuspend", value) }
pub fn ontimeupdate(value: &str) -> Fragment { attribute("ontimeupdate", value) }
pub fn ontoggle(value: &str) -> Fragment { attribute("ontoggle", value) }
pub fn onunload(value: &str) -> Fragment { attribute("onunload", value) }
pub fn onvolumechange(value: &str) -> Fragment { attribute("onvolumechange", value) }
pub fn onwaiting(value: &str) -> Fragment { attribute("onwaiting", value) }
pub fn onwheel(value: &str) -> Fragment { attribute("onwheel", value) }
pub fn open(value: &str) -> Fragment { attribute("open", value) }
pub fn optimum(value: &str) -> Fragment { attribute("optimum", value) }
pub fn pattern(value: &str) -> Fragment { attribute("pattern", value) }
pub fn placeholder(value: &str) -> Fragment { attribute("placeholder", value) }
pub fn poster(value: &str) -> Fragment { attribute("poster", value) }
pub fn preload(value: &str) -> Fragment { attribute("preload", value) }
pub fn readonly(value: &str) -> Fragment { attribute("readonly", value) }
pub fn rel(value: &str) -> Fragment { attribute("rel", value) }
pub fn required(value: &str) -> Fragment { attribute("required", value) }
pub fn reversed(value: &str) -> Fragment { attribute("reversed", value) }
pub fn rows(value: &str) -> Fragment { attribute("rows", value) }
pub fn rowspan(value: &str) -> Fragment { attribute("rowspan", value) }
pub fn sandbox(value: &str) -> Fragment { attribute("sandbox", value) }
pub fn scope(value: &str) -> Fragment { attribute("scope", value) }
pub fn scoped(value: &str) -> Fragment { attribute("scoped", value) }
pub fn selected(value: &str) -> Fragment { attribute("selected", value) }
pub fn shape(value: &str) -> Fragment { attribute("shape", value) }
pub fn size(value: &str) -> Fragment { attribute("size", value) }
pub fn sizes(value: &str) -> Fragment { attribute("sizes", value) }
pub fn span(value: &str) -> Fragment { attribute("span", value) }
pub fn spellcheck(value: &str) -> Fragment { attribute("spellcheck", value) }
pub fn src(value: &str) -> Fragment { attribute("src", value) }
pub fn srcdoc(value: &str) -> Fragment { attribute("srcdoc", value) }
pub fn srclang(value: &str) -> Fragment { attribute("srclang", value) }
pub fn srcset(value: &str) -> Fragment { attribute("srcset", value) }
pub fn start(value: &str) -> Fragment { attribute("start", value) }
pub fn step(value: &str) -> Fragment { attribute("step", value) }
pub fn style(value: &str) -> Fragment { attribute("style", value) }
pub fn tabindex(value: &str) -> Fragment { attribute("tabindex", value) }
pub fn target(value: &str) -> Fragment { attribute("target", value) }
pub fn title(value: &str) -> Fragment { attribute("title", value) }
pub fn translate(value: &str) -> Fragment { attribute("translate", value) }
pub fn _type(value: &str) -> Fragment { attribute("type", value) }
pub fn usemap(value: &str) -> Fragment { attribute("usemap", value) }
pub fn value(value: &str) -> Fragment { attribute("value", value) }
pub fn width(value: &str) -> Fragment { attribute("width", value) }
pub fn wrap(value: &str) -> Fragment { attribute("wrap", value) }
