use {
    axum::{
        extract::{ DefaultBodyLimit, Form, Multipart, Query, },
        http::{ header, HeaderMap, StatusCode, },
        Router,
        routing::{ get, post, },
        serve,
    },
    std::{
        collections::{ BTreeMap, HashMap, },
        env::args,
        fs::{ create_dir_all, File, },
        io::Write,
        path::PathBuf,
        process::{ Command, Output, },
        sync::OnceLock,
    },
    tempfile::TempDir,
    tenplates_core::{ Context, Tenplates },
    tokio::net::TcpListener,
};

const HELP: &str = include_str!("../resources/help.txt");

fn string_from_dkv(dkv: String) -> (String, String) {
    let mut route = String::new();
    let mut path = String::new();

    let mut iter = dkv.chars();
    let dlim = iter.next().unwrap();
    let mut split = false;
    for c in iter {
        if !split && c == dlim {
            split = true;
            continue;
        }

        if !split {
            route.push(c);
        }
        else {
            path.push(c);
        }
    }

    (route, path)
}

fn from_dkv(dkv: String) -> (PathBuf, PathBuf) {
    let (route, path) = string_from_dkv(dkv);

    (route.into(), path.into())
}

fn print_stderr(o: &Output) {
    let err = String::from_utf8(o.stderr.to_owned()).unwrap();
    let err = err.trim();

    if !err.is_empty() {
        eprintln!("{err}");
    }
}

static LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn base26(mut dec: usize) -> String {
    let mut out = String::new();

    loop {
        let rem = dec % LETTERS.len();
        dec = dec / LETTERS.len();

        out = format!("{}{out}", LETTERS[rem]);

        if dec == 0 {
            break;
        }
        else {
            dec -= 1;
        }
    }

    out
}

static VARS: OnceLock<Vec<(String, String)>> = OnceLock::new();
static NAME: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() {
    let mut long_args = args();
    long_args.next(); // burn prog name

    let mut ip = String::from("localhost");
    let mut port = String::from("8080");
    let mut gets = Vec::new();
    let mut urlencodeds = Vec::new();
    let mut multiparts = Vec::new();
    let mut variables = Vec::new();

    while let Some(long_arg) = long_args.next() {
        if long_arg.starts_with("--") {
            let long_arg = &long_arg[2..];

            match long_arg {
                "get" => gets.push(from_dkv(long_args.next().unwrap())),
                "help" => {
                    println!("{HELP}");
                    return;
                },
                "ip" => ip = long_args.next().unwrap(),
                "multipart" => multiparts.push(from_dkv(long_args.next().unwrap())),
                "name" => {
                    NAME.get_or_init(|| long_args.next().unwrap());
                },
                "port" => port = long_args.next().unwrap(),
                "set" => variables.push(string_from_dkv(long_args.next().unwrap())),
                "urlencoded" => urlencodeds.push(from_dkv(long_args.next().unwrap())),
                unknown => panic!("Unknown argument --{unknown}"),
            }
        }
        else if long_arg.starts_with('-') {
            let mut short_args = long_arg[1..].chars();

            while let Some(short_arg) = short_args.next() {
                match short_arg {
                    'g' => {
                        assert!(short_args.next().is_none());
                        gets.push(from_dkv(long_args.next().unwrap()));
                    },
                    'h' => {
                        println!("{HELP}");
                        return;
                    },
                    'i' => {
                        assert!(short_args.next().is_none());
                        ip = long_args.next().unwrap();
                    },
                    'm' => {
                        assert!(short_args.next().is_none());
                        multiparts.push(from_dkv(long_args.next().unwrap()));
                    },
                    'n' => {
                        assert!(short_args.next().is_none());
                        NAME.get_or_init(|| long_args.next().unwrap());
                    },
                    'p' => {
                        assert!(short_args.next().is_none());
                        port = long_args.next().unwrap();
                    },
                    's' => {
                        assert!(short_args.next().is_none());
                        variables.push(string_from_dkv(long_args.next().unwrap()));
                    },
                    'u' => {
                        assert!(short_args.next().is_none());
                        urlencodeds.push(from_dkv(long_args.next().unwrap()));
                    },
                    unknown => panic!("Unknown argument -{unknown}"),
                }
            }
        }
        else {
            panic!("Unknown argument {long_arg}");
        }
    }

    VARS.get_or_init(move || variables);

    let mut app  = Router::new();

    for (route_pb, path) in gets {
        let route = route_pb.to_str().unwrap();

        app = app.route(route, get(async |query: Query<BTreeMap<String, String>>| -> (StatusCode, HeaderMap, Vec<u8>) {
            let mut output = Vec::<u8>::new();

            let mut context = Context::default();
            for (key, value) in query.iter() {
                context.add_variable(format!("params.{key}"), "", value);
            }

            for (key, value) in VARS.get_or_init(|| Vec::new()).iter() {
                context.add_variable(key, "", value);
            }

            match Tenplates::compile_file_with_ctx(path, &mut output, context) {
                Ok(_) => {
                    let mut headers = HeaderMap::new();
                    headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap());

                    (StatusCode::OK, headers, output)
                },
                Err(e) => {
                    eprintln!("{e}");
                    let headers = HeaderMap::new();

                    (StatusCode::NOT_FOUND, headers, Vec::new())
                },
            }
        }));
    }

    for (route_pb, path) in urlencodeds {
        let route = route_pb.to_str().unwrap();

        app = app.route(route, post(async |form: Form<Vec<(String, String)>>| -> (StatusCode, HeaderMap) {
            let tmp = TempDir::with_prefix(NAME.get_or_init(|| "tenplates-serve".to_owned())).unwrap();

            let mut key_map = HashMap::<String, usize>::new();

            for (key, value) in form.iter() {
                key_map.insert(key.to_owned(), key_map.get(key).map(|v| *v + 1).unwrap_or(0));

                let filename = base26(*key_map.get(key).unwrap());

                let mut path: PathBuf = tmp.path().into();
                path.push(key);

                create_dir_all(&path).unwrap();

                path.push(filename);

                let mut file = File::create(&path).unwrap();
                file.write_all(value.as_bytes()).unwrap();
                drop(file);
            }

            let mut headers = HeaderMap::new();

            match Command::new(path).arg(tmp.path()).output() {
                Ok(o) => {
                    tmp.close().unwrap();

                    print_stderr(&o);

                    let out = String::from_utf8(o.stdout).unwrap();
                    let out = out.trim_end();

                    if !out.is_empty() {
                        headers.insert(header::LOCATION, out.parse().unwrap());
                        (StatusCode::SEE_OTHER, headers)
                    }
                    else {
                        (StatusCode::OK, headers)
                    }
                },
                Err(e) => {
                    tmp.close().unwrap();

                    eprintln!("{e:?}");

                    (StatusCode::NOT_FOUND, headers)
                },
            }
        }));
    }

    for (route_pb, path) in multiparts {
        let route = route_pb.to_str().unwrap();

        app = app
            .route(route, post(async |mut form: Multipart| -> (StatusCode, HeaderMap) {
                let tmp = TempDir::with_prefix(NAME.get_or_init(|| "tenplates-serve".to_owned())).unwrap();

                let mut key_map = HashMap::<String, usize>::new();

                while let Some(field) = form.next_field().await.unwrap() {
                    let key = field.name().unwrap().to_string();
                    key_map.insert(key.to_owned(), key_map.get(&key).map(|v| *v + 1).unwrap_or(0));

                    let filename = base26(*key_map.get(&key).unwrap());

                    let bytes = field.bytes().await.unwrap();

                    let mut path: PathBuf = tmp.path().into();
                    path.push(key);

                    create_dir_all(&path).unwrap();

                    path.push(filename);

                    let mut file = File::create(path).unwrap();
                    file.write_all(&bytes).unwrap();
                    drop(file);
                }

                let mut headers = HeaderMap::new();

                match Command::new(path).arg(tmp.path()).output() {
                    Ok(o) => {
                        tmp.close().unwrap();

                        print_stderr(&o);

                        let out = String::from_utf8(o.stdout).unwrap();
                        let out = out.trim_end();

                        if !out.is_empty() {
                            headers.insert(header::LOCATION, out.parse().unwrap());
                            (StatusCode::SEE_OTHER, headers)
                        }
                        else {
                            (StatusCode::OK, headers)
                        }
                    },
                    Err(e) => {
                        tmp.close().unwrap();

                        eprintln!("{e:?}");

                        (StatusCode::NOT_FOUND, headers)
                    },
                }
            }))
            .layer(DefaultBodyLimit::max(10000000)); // 10 MB
    }

    let listener = TcpListener::bind(&format!("{ip}:{port}"))
        .await
        .unwrap();

    serve(listener, app).await.unwrap();
}
