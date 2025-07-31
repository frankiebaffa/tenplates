> _Alright, fine. I can't stack ten plates, but I'll stack templated text files
> like it's nobody's business._  
> - **Me**

## <a id="tags">Tags</a>

### <a id="t-add">Add</a>

Adds together an addend stored in [context](#g-context) and a templated
addend.

```tenplate
{% set x %}5{% /set %}\
{% add x %}10{% /add %}\
{{ x }}
```

```txt
15
```

The following other tag(s) were used in this example.

- [_get_](#t-get)
- [_set_](#t-set)

### <a id="t-assert">Assert</a>

Verifies that a [condition](#conditions) is truthy before continuing, will throw
at compile-time otherwise.

```tenplate
{% assert "1" /%}
```

### <a id="t-call">Call</a>

Processes an external file inline, modifying the existing [context](#g-context)
along the way.

```tenplate
{# ./functions/header.tenplate #}\
{% fn header(lvl, txt) %}\
    <h{{ lvl }}>{{ txt }}</h{{ lvl }}>\
{% /fn %}\
```

```tenplate
{% call "./functions/header.tenplate" /%}\
{% exec header("2", "Hello") /%}
```

```txt
<h2>Hello</h2>
```

The following other tag(s) were used in this example.

- [_exec_](#t-exec)
- [_fn_](#t-fn)

### <a id="comment">Comment</a>

Instructs the compiler to skip all content contained within the open/close tags.

```tenplate
{# this is a comment #}
```

### <a id="t-compile">Compile</a>

Processes an external file inline without modifying the existing
[context](#g-context).

```tenplate
{# ./set/name.tenplate #}\
{% set name %}Frankie{% /set %}\
```

```tenplate
{% set name %}Matthew{% /set %}\
{% compile "./set/name.tenplate" /%}{{ name }}
```

```txt
Matthew
```

The following other tag(s) were used in this example.

- [_set_](#t-set)

### <a id="t-div">Div</a>

Performs division on a dividend in [context](#g-context) and a templated
divisor.

```tenplate
{% set x %}4{% /set %}\
{% div x %}2{% /div %}
```

```txt
2
```

The following other tag(s) were used in this example.

- [_set_](#t-set)

### <a id="t-exec">Exec</a>

Executes a function already read into the current [context](#g-context).
Arguments can be excluded from right-to-left.

```tenplate
{% fn commas(a, b, c) %}\
    {{ a }}, {{ b }}{% if c %}, {{ c }}{% /if %}\
{% /fn %}\

{% set d %}foo{% /set %}\
{% set e %}bar{% /set %}\

{% exec commas(d, e, "baz") /%}
{% exec commas(d, e) /%}
```

```txt
foo, bar, baz
foo, bar
```

The following other tag(s) were used in this example.

- [_fn_](#t-fn)
- [_set_](#t-set)

### <a id="t-extend">Extend</a>

Sets a single file as an outer template to process with the result of the
current file. The [context](#g-context) will be passed along and the
[content](#g-content) will be assigned to the special [context](#g-context)
[variable](#g-variable) `CONTENT`. If the extend tag is used multiple times
within the same template, the last tag used wins.

```tenplate
{# ../papa.tenplate #}\

{% assert name /%}\
{% assert paragraph /%}\

<h1>{{ name }}</h1>
<p>{{ paragraph }}</p>\

{% if CONTENT %}
<hr>
<p>{{ CONTENT }}</p>\
{% /if %}
```

```tenplate
{% extend "../papa.tenplate" /%}\

{% set name %}Frankie{% /set %}\
{% set paragraph %}This is a paragraph.{% /set %}\

And here is some output content.
```

```txt
<h1>Frankie</h1>
<p>This is a paragraph.</p>
<hr>
<p>And here is some output content.</p>
```

The following other tag(s) were used in this example.

- [_assert_](#t-assert)
- [_if_](#t-if)
- [_set_](#t-set)

### <a id="t-fn">Fn</a>

Registers a [function](#g-function) in [context](#g-context) which can be called
using the [exec](#t-exec) tag. A function can have anywhere from 0 to _n_
arguments.

```tenplate
{% fn commas(one, two, three) %}\
    {{ one }}, {{ two }}{% if three %}, {{ three }}{% /if %}.\
{% /fn %}\
{% exec commas("First", "Second", "Third") /%}
{% exec commas("First", "Second") /%}
```

```txt
First, Second, Third.
First, Second.
```

The following other tag(s) were used in this example.

- [_exec_](#t-exec)
- [_if_](#t-if)

### <a id="t-fordir">Fordir / Else</a>

Loops through each directory within a given directory. The element
[variable](#g-variable) will contain the path of the directory. If a name is
given for the loop [variable](#g-variable) (`as dir_loop` in the example below),
the [loop context](#loop-context) will be stored with the given variable as a
prefix. The `else` condition is triggered when no elements are found for the
loop.

Assume the following file stucture for the next example.

```txt
./
 \
  a-dir/
       \
        First/
        Second/
        Third/
```

```tenplate
{% fordir d in "./a-dir" as dir_loop %}\
    {% if dir_loop.isfirst %}{% else %}, {% /if %}\
    "{{ d }}"\
{% else %}\
    {# no directories in "./a-dir" #}\
{% /fordir %}
```

```txt
"./a-dir/First", "./a-dir/Second", "./a-dir/Third"
```

The following other tag(s) were used in this example.

- [_if_](#t-if)

### <a id="t-foreach">Foreach / Else</a>

Loops through each value in a given variable in [context](#g-context). See
[set](#t-set) for info on how a [variable](#g-variable) can have multiple
values. The optional [loop context](#loop-context) definition behaves
identically to the [fordir](#t-fordir) tag.

```tenplate
The siblings are \
{% set names %}Matthew{% /set %}\
{% set names %}Frankie{% /set %}\
{% set names %}Karina{% /set %}\
{% foreach name in items as name_loop %}\
    {% if name_loop.isfirst %}{% else %}, {% /if %}\
        {% if name_loop.islast %}and {% /if %}\
        {{ name }}\
    {% /if %}\
    {% if name_loop.islast %}.{% /if %}\
{% else %}\
    {# no items #}
{% /foreach %}
```

```txt
The siblings are Matthew, Frankie, and Karina.
```

The following other tag(s) were used in this example.

- [_if_](#t-if)

### <a id="t-forfile">Forfile / Else</a>

Loops through each file in a given directory. The element
[variable](#g-variable) will contain the path. The optional
[loop context](#loop-context) definition behaves identically to the
[fordir](#t-fordir) tag.

Assume the following file stucture and contents for the next example.

```txt
./
 \
  sibling.tenplate
  siblings/
          \
           first.tenplate
           second.tenplate
           third.tenplate
```

```tenplate
{# ./sibling.tenplate #}\
{% assert sibling.filepath /%}\
{% call sibling.filepath /%}\
{% assert sibling.name /%}\
{% assert sibling.description /%}\
<tr><td>{{ sibling.name }}</td><td>{{ sibling.description }}.</td></tr>\
```

```tenplate
{# ./siblings/first.tenplate #}\
{% set sibling.name %}Matthew{% /set %}\
{% set sibling.description %}The elder{% /set %}\
```

```tenplate
{# ./siblings/second.tenplate #}\
{% set sibling.name %}Frankie{% /set %}\
{% set sibling.description %}The poor middle-child{% /set %}
```

```tenplate
{# ./siblings/third.tenplate #}\
{% set sibling.name %}Karina{% /set %}\
{% set sibling.description %}Da baby{% /set %}\
```

```tenplate
<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>\
        {% set sibsdir %}{% path "./siblings" /%}{% /set %}\
        {% set sibtemplate %}{% path "./sibling.tenplate" /%}{% /set %}\
        {% forfile sibling.filepath in sibsdir %}
        {% compile sibtemplate /%}\
        {% else %}\
            {# no files in "./a-dir" #}\
        {% /fordir %}
    </tbody>
</table>
```

```txt
<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Description</th>
        </tr>
    </thead>
    <tbody>
        <tr><td>Matthew</td><td>The elder.</td></tr>
        <tr><td>Frankie</td><td>The poor middle-child.</td></tr>
        <tr><td>Karina</td><td>Da baby.</td></tr>
    </tbody>
</table>
```

The following other tag(s) were used in this example.

- [_call_](#t-call)
- [_compile_](#t-compile)
- [_path_](#t-path)
- [_set_](#t-set)

### <a id="t-get">Get</a>

Gets a value from a [variable](#g-variable) in [context](#g-context).

```tenplate
{% set msg %}Hi{% /set %}\
{{ msg }}
```

```txt
Hi
```

The following other tag(s) were used in this example.

- [_set_](#t-set)

### <a id="t-if">If / Else</a>

Compiles one of two code-paths depending on whether the [condition](#conditions)
evaluates to true or false. The `else` tag is an optional inclusion.

```tenplate
{% if "1" %}\
    True\
{% else %}\
    False\
{% /if %}
```

```txt
True
```

### <a id="t-include">Include</a>

Includes a file inline with compilation. Useful for including files which
contain `Ten Plates` syntax.

```tenplate
{# ./includes/file.tenplate #}\
{% set name %}Frankie{% /set %}\
```

```tenplate
{% include "./includes/file.tenplate" /%}
```

```txt
{# ./includes/file.tenplate#}\
{% set name %}Frankie{% /set %}\
```

The following other tag(s) were used in this example.

- [_set_](#t-set)

### <a id="t-mod">Mod</a>

Performs modulo operation on a dividend in [context](#g-context) and a templated
divisor.

```tenplate
{% set x %}4{% /set %}\
{% mod x %}2{% /mod %}
```

```txt
0
```

The following other tag(s) were used in this example.

- [_set_](#t-set)

### <a id="t-mul">Mul</a>

Performs multiplication on a factor in [context](#g-context) and a templated
factor.

```tenplate
{% set x %}4{% /set %}\
{% mul x %}2{% /mul %}
```

```txt
8
```

The following other tag(s) were used in this example.

- [_set_](#t-set)

### <a id="t-nth">Nth</a>

Retrieves the _n_-th element from an array of values.

```tenplate
{% set arr %}One{% /set %}\
{% set arr %}Two{% /set %}\
{% set arr %}Three{% /set %}\
{% nth arr %}1{% /nth %}
```

```txt
Two
```

The following other tag(s) were used in this example.

- [_set_](#t-set)

### <a id="t-path">Path</a>

Computes the canonical path for a given path. The entry **must** exist in the
file system to avoid throwing an error.

```tenplate
{% path "./file.txt" /%}
```

```txt
/home/user/file.txt
```

### <a id="t-set">Set</a>

Sets a value for a [variable](#g-variable) in [context](#g-context). When
multiple values are set for a given [variable](#g-variable), the previous value
is not overwritten, but is masked by the new value. These values can then be
iterated over in the order in which they were set using the
[for-each](#t-foreach) tag.

```tenplate
{% set v %}1{% /set %}\
{{ v }}
```

```txt
1
```

The following other tag(s) were used in this example.

- [_get_](#t-get)

### <a id="t-sub">Sub</a>

Performs subtraction on a minuend in [context](#g-context) and a templated
subtracahend.

```tenplate
{% set x %}4{% /set %}\
{% sub x %}2{% /sub %}
```

```txt
2
```

The following other tag(s) were used in this example.

- [_set_](#t-set)

## <a id="conditions">Conditions</a>

A set of one or more of logical assertions evaluating to true or false. These
can be nested using parenthetical notation or conjoined using the
short-circuiting _and_ or _or_ operators. The values contained within conditions
are evaluated in their _string_ form so `Ten Plates` performs boolean casting
on all values.

```tenplate
{# true #}{% assert "1" /%}
{# true #}{% assert "Hello, World!" /%}

{# false #}{% assert "0" /%}
{# false #}{% assert "" /%}
{# false #}{% assert a /%}

{# true #}{% set a %}1{% /set %}
{# true #}{% assert "1" == a /%}

{# true #}{% set b %}0{% /set %}
{# true #}{% assert a || b /%}

{# true #}{% assert (a && b) || "1" /%}

{# true #}{% set d %}500{% /set %}
{# true #}{% assert d > a /%}

{# true #}{% assert "501" > d /%}

{# true #}{% assert "501" >= d /%}

{# true #}{% assert "501" != d /%}

{# true #}{% assert "501" <= d /%}
{# true #}{% assert "501" < d /%}
```

## <a id="glossary">Glossary</a>

<a id="g-content">**Content**</a>: The final output of a tenplate.

<a id="g-context">**Context**</a>: Functions, values, and other data currently
in-scope and usable.

<a id="g-function">**Function**</a>: A block of tenplate keyed with a given
name for future retrieval and compilation against an optional set of named
arguments.

<a id="g-variable">**Variable**</a>: A value in context keyed with a given
name for future retrieval.
