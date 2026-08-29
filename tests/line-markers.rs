//! `# <line> "<file>"` markers: the filename is a C string literal, so it may
//! contain spaces and the escapes `\"` / `\\` (clang emits both).

use cgrammar::lex;

#[test]
fn line_marker_filenames_keep_spaces_and_escapes() {
    let pre = "# 3 \"/tmp/dir with space/m.c\" 1\nint boom;\n";
    let (_, ctx) = lex(pre, Some("/tmp/dir with space/m.c"));
    let at = pre.find("boom").unwrap();
    let sc = ctx.context_at_offset(at).expect("marker parsed");
    assert_eq!(sc.filename, "/tmp/dir with space/m.c");

    let pre = "# 1 \"a\\\"b\\\\c.h\"\nint x;\n";
    let (_, ctx) = lex(pre, Some("m.c"));
    let sc = ctx.context_at_offset(pre.find('x').unwrap()).unwrap();
    assert_eq!(sc.filename, "a\"b\\c.h");

    let pre = "#line 5 foo.c\nint y;\n";
    let (_, ctx) = lex(pre, Some("m.c"));
    let sc = ctx.context_at_offset(pre.find('y').unwrap()).unwrap();
    assert_eq!(sc.filename, "foo.c");
}
