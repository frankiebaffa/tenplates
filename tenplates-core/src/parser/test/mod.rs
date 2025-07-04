use {
    crate::{
        context::Context,
        parser::{ Parser, TemplateParser },
    },
    std::path::PathBuf,
};

#[test]
fn parse_chars_1() {
    let mut output = Vec::<u8>::new();
    let input = "this is just some input";
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(input, &output_str);
}

#[test]
fn parse_chars_2() {
    let mut output = Vec::<u8>::new();
    let input = "this is some input with a < sign";
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(input, &output_str);
}

#[test]
fn parse_extend() {
    let mut output = Vec::<u8>::new();
    let input = "Some text, and a tag: {% extend \"./resources/template.txt\" /%}";
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("**Some text, and a tag: **\n", &output_str);
}

#[test]
fn parse_set_text() {
    let mut output = Vec::<u8>::new();
    let input = r#"Some text, and a tag: {% set hello %}Hello, World!{% /set %}"#;
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    let ctx = parser.take_context().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("Some text, and a tag: ", &output_str);

    let hello = ctx.value("hello").unwrap();
    let against_hello = "Hello, World!";
    assert_eq!(&against_hello, hello);
}

#[test]
fn parse_set_text_esc() {
    let mut output = Vec::<u8>::new();
    let input = "Some text, and a tag: {% set hello %}\\{%Hello, World %}{% /set %}";
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    let ctx = parser.take_context().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("Some text, and a tag: ", &output_str);

    let hello = ctx.value("hello").unwrap();
    let against_hello = "{%Hello, World %}";
    assert_eq!(&against_hello, hello);
}

#[test]
fn parse_escape_1() {
    let input = r#"This should be included: \{% extend "../something.txt" /%}"#;
    let mut output = Vec::new();
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(
        "This should be included: {% extend \"../something.txt\" /%}",
        &output_str
    );
}

#[test]
#[should_panic]
fn parse_unknown_tag() {
    let input = "{% execut `something` %}";
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
}

#[test]
#[should_panic]
fn parse_unexpected_eof_in_tag_name() {
    let input = "{% execut";
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
}

#[test]
fn parse_include_1() {
    let input = "File: {% include \"./resources/parse-include-1.txt\" /%}";
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output = String::from_utf8(output).unwrap();

    assert_eq!(
        "File: {% execute `this should just be included` %}",
        output
    );
}

#[test]
fn parse_include_2() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_include_2/item.tenplate"),
        &mut output,
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();

    assert_eq!("The name of the item is \"Item.\"", output_str);
}

#[test]
fn parse_comment_1() {
    let input = "Here is some text{# and a comment #} and some more text.";
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output = String::from_utf8(output).unwrap();

    assert_eq!(
        "Here is some text and some more text.",
        output,
    );
}

#[test]
fn parse_comment_2() {
    let input = "Here is some text{# and a comment \\#} with escaped #} and some more text.";
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output = String::from_utf8(output).unwrap();

    assert_eq!(
        "Here is some text and some more text.",
        output,
    );
}

#[test]
fn parse_file_1() {
    let input_file = PathBuf::from("./resources/extended.txt");
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(Context::default(), input_file, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output = String::from_utf8(output).unwrap();

    assert_eq!(
        "**User(1): test.user, User(2): second.user**\n",
        output
    );
}

#[test]
fn parse_compile_1() {
    let input_file = PathBuf::from("./resources/compile-page.tenplate");
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(Context::default(), input_file, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output = String::from_utf8(output).unwrap();

    assert_eq!(
        concat!(
            "Compile this: Below are the paragraphs.\n",
            "This is some text here.\n",
            "And some more text.\n",
            "Here is another paragraph. Blah."
        ),
        &output
    );
}

#[test]
fn parse_for_multiple() {
    let mut output = Vec::<u8>::new();
    let mut input = String::new();
    input.push_str(r#"Some text, and a tag: \
        {% set users %}test.user{% /set %}\
        {% set users %}second.user{% /set %}\
        {% foreach user in users %}|{{ user }}{% /foreach %}"#);
    let mut parser = TemplateParser::new(Context::default(), input.as_str(), &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(
        "Some text, and a tag: |test.user|second.user",
        output_str,
    );
}

#[test]
fn parse_for_none_else() {
    let mut output = Vec::<u8>::new();
    let mut input = String::new();
    input.push_str(r#"Some text, and a tag: \
        {% foreach user in users %}|{{ user }}{% else %}No users.{%/ foreach %}\
    "#);
    let mut parser = TemplateParser::new(Context::default(), input.as_str(), &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(
        "Some text, and a tag: No users.",
        output_str,
    );
}

#[test]
fn parse_for_none_no_else() {
    let mut output = Vec::<u8>::new();
    let mut input = String::new();
    input.push_str(r#"Some text, and a tag: \
        {% foreach user in users %}|{{ user.username }}{%/ foreach %}\
    "#);
    let mut parser = TemplateParser::new(Context::default(), input.as_str(), &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(
        "Some text, and a tag: ",
        output_str,
    );
}

#[test]
#[should_panic]
fn parse_assert_1() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("id", "./", "1");
    let input = r#"{% assert id == "0" /%}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
}

#[test]
fn parse_assert_2() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("id", "./", "1");
    let input = r#"{% assert id == "1" /%}True"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("True", output_str);
}

#[test]
fn parse_if_1() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("id", "./", "1");
    let input = r#"{% if id == "0" %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("False", output_str);
}

#[test]
fn parse_if_2() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("id", "./", "2");
    context.add_variable("id2", "./", "5");
    let input = r#"{% if id == "1" || id2 > "4" %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("True", output_str);
}

#[test]
fn parse_if_3() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("id1", "./", "2");
    context.add_variable("id2", "./", "5");
    context.add_variable("id3", "./", "10");
    // str(10) < str(2.5)
    let input = r#"{% if (id1 == "0" || id2 > "4") && "2.5" < id3 %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("False", output_str);
}

#[test]
fn parse_if_4() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("id1", "./", "2");
    context.add_variable("id2", "./", "5");
    context.add_variable("id3", "./", "10");
    let input = r#"{% if id1 == "2" || id2 < "4" || id3 < "2.5" %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("True", output_str);
}

// should evaluate identically to 4
#[test]
fn parse_if_5() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("id1", "./", "2");
    context.add_variable("id2", "./", "5");
    context.add_variable("id3", "./", "10");
    let input = r#"{% if (id1 == "2" || id2 < "4") || id3 < "2.5" %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("True", output_str);
}

#[test]
fn parse_if_truthy_1() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("id", "./", "2");
    let input = r#"{% if id %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("True", output_str);
}

#[test]
fn parse_if_truthy_2() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("id", "./", "0");
    let input = r#"{% if id %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("False", output_str);
}

#[test]
fn parse_if_truthy_3() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("a", "./", "1");
    context.add_variable("b", "./", "0");
    let input = r#"{% if b || a %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("True", output_str);
}

// should evaluate to the same as 3
#[test]
fn parse_if_truthy_4() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("a", "./", "1");
    context.add_variable("b", "./", "0");
    let input = r#"{% if (b || a) %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("True", output_str);
}

#[test]
fn parse_if_mixed_1() {
    let mut output = Vec::<u8>::new();
    let mut context = Context::default();
    context.add_variable("a", "./", "1");
    context.add_variable("b", "./", "0");
    let input = r#"{% if (b || (b == "0" && a)) %}True{% else %}False{% /if %}"#;
    let mut parser = TemplateParser::new(context, input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("True", output_str);
}

#[test]
fn nested_parse_if_and_for() {
    let mut output = Vec::<u8>::new();
    let mut input = String::new();
    input.push_str(r#"Some text, and a tag: \
    {% set users %}test.user{% /set %}\
    {% set users %}second.user{% /set %}\
    {% foreach user in users as loop %}\
        {% if loop.isfirst %}{% else %}, {% /if %}\
        {{ user }}\
        {% if loop.islast %}.{% /if %}\
    {% else %}\
        No users.\
    {%/ foreach %}"#);
    let mut parser = TemplateParser::new(Context::default(), input.as_str(), &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(
        "Some text, and a tag: test.user, second.user.",
        output_str,
    );
}

#[test]
fn parse_complex_1() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_complex_1/page.tenplate"),
        &mut output
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(
        concat!(
            "TEMPLATE START\n",
            "Matthew is 33 year(s) old.\n",
            "Frankie is 31 year(s) old.\n",
            "Karina is 27 year(s) old.\n",
            "TEMPLATE END\n",
        ),
        output_str
    );
}

#[test]
fn parse_complex_2() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_complex_2/page.tenplate"),
        &mut output
    ).unwrap();
    parser.parse().unwrap();
    let split = parser.context().unwrap().value(&"split");
    assert_eq!(None, split);
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("<h1><span>The H</span><span>eader</span></h1>", output_str);
}

#[test]
#[should_panic]
fn unexpected_eof() {
    let mut output = Vec::<u8>::new();
    let input = r#"{% let str1 = "First section"#;
    let mut parser = TemplateParser::new(Context::default(), input, &mut output).unwrap();
    parser.parse().unwrap();
    drop(parser);
}

#[test]
fn parse_call_1() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_call_1/page.tenplate"),
        &mut output
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output = String::from_utf8(output).unwrap();
    assert_eq!(
        "1\n", output
    );
}

#[test]
fn parse_complex_3() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_complex_3/page.tenplate"),
        &mut output
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("name: Frankie\nage: 31\nchildren: ", output_str);
}

#[test]
fn parse_complex_4() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_complex_4/page.tenplate"),
        &mut output
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("#ffb4a1", output_str);
}

#[test]
fn call_compile_chain_1() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/call_compile_chain_1/page.tenplate"),
        &mut output
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("1", output_str);
}

#[test]
fn parse_complex_5() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_complex_5/person/frankie.tenplate"),
        &mut output,
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(
        concat!(
            "<h1 class=\"lead split\"><span>Frankie</span></h1>\n",
            "<div class=\"child\">\n",
            "\t<table class=\"lr\">\n",
            "\t<tbody>\n",
            "\t\t<tr>\n",
            "\t\t\t<td>Child</td>\n",
            "\t\t\t<td><strong>Frankie Jr</strong></td>\n",
            "\t\t</tr>\n",
            "\t</tbody>\n",
            "</table>\n",
            "\t</div>\n",
            "<div class=\"child\">\n",
            "\t<table class=\"lr\">\n",
            "\t<tbody>\n",
            "\t\t<tr>\n",
            "\t\t\t<td>Child</td>\n",
            "\t\t\t<td><strong>Frankie II</strong></td>\n",
            "\t\t</tr>\n",
            "\t</tbody>\n",
            "</table>\n",
            "\t</div>\n",
        ),
        output_str
    );
}

#[test]
fn parse_complex_6() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_complex_6/allsiblings.tenplate"),
        &mut output
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(
        include_str!("../../../resources/parse_complex_6/against.txt"),
        output_str
    );
}

#[test]
fn parse_function_1() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_function_1/page.tenplate"),
        &mut output,
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("one, two, three", output_str);
}

#[test]
fn parse_function_2() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_function_2/page.tenplate"),
        &mut output,
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();
    assert_eq!("One, Two, Third\nFourth, Fifth\n", output_str);
}

#[test]
fn parse_path_1() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_path_1/page.tenplate"),
        &mut output,
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();

    assert_eq!("Frankie", output_str);
}

#[test]
fn parse_path_2() {
    let mut output = Vec::<u8>::new();
    let mut parser = TemplateParser::new(
        Context::default(),
        PathBuf::from("./resources/parse_path_2/item.tenplate"),
        &mut output,
    ).unwrap();
    parser.parse().unwrap();
    drop(parser);
    let output_str = String::from_utf8(output).unwrap();

    assert_eq!("Frankie is 31 year(s) old.", output_str);
}

#[test]
fn parse_assert_eq_1() {
	let mut output = Vec::<u8>::new();
	let mut parser = TemplateParser::new(
		Context::default(),
		PathBuf::from("./resources/assert_not_eq_1/test.tenplate"),
		&mut output,
	).unwrap();
	parser.parse().unwrap();
	drop(parser);
	let output_str = String::from_utf8(output).unwrap();

	assert_eq!(
		include_str!("../../../resources/assert_not_eq_1/against.txt"),
		output_str
	);
}
