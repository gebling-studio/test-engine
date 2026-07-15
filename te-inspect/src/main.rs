use std::{
    collections::HashMap,
    env::temp_dir,
    fs::{read_to_string, write},
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    process::exit,
    time::Duration,
};

use anyhow::{Result, bail};
use base64::{Engine, engine::general_purpose::STANDARD};
use clap::{Parser, Subcommand};
use mdns_sd::{ScopedIp, ServiceDaemon, ServiceEvent};
use refs::{Own, hreads::set_current_thread_as_main};
use serde_json::{Value, from_str, json, to_string, to_string_pretty, to_value};
use test_engine::{
    gm::color::Color,
    inspect::protocol::{
        AppCommand, Client, InspectorCommand, SERVICE_TYPE, UIRequest, UIResponse, ui::ViewRepr,
    },
};
use tokio::time::{Instant, timeout, timeout_at};

const NO_APPS: &str =
    "No running apps discovered. The app must be a debug build running on the same network.";

#[derive(Parser)]
#[command(
    name = "te-inspect",
    about = "Inspect and edit UI of running test-engine apps"
)]
struct Cli {
    /// App id from `apps`. Needed only when several apps run.
    #[arg(long, global = true)]
    app: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List running apps discovered on the local network
    Apps,
    /// Print a compact overview of the view tree: label, frame, id per line
    Tree,
    /// Print full JSON of every view whose label contains the query, or with
    /// this exact id
    View {
        /// Label substring, case insensitive, or an exact view id
        query: String,
    },
    /// Print the whole view tree as JSON
    Ui,
    /// Save a screenshot as PNG and print its path
    Screenshot {
        /// Output file. Defaults to te-screenshot.png in the temp dir.
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Edit a layout rule: offset for Side and Anchor rules, ratio for Relative
    /// rules
    EditRule {
        /// View id from `tree` or `view`
        view_id:    String,
        /// Index into the view's placer rules from `view`
        rule_index: usize,
        offset:     f32,
        /// Disable the rule instead of keeping it applied
        #[arg(long)]
        disable:    bool,
    },
    /// Set the text of a `Label`, `Button` or `TextField`
    SetText { view_id: String, text: String },
    /// Set the background color of a view, components 0 to 1
    SetColor {
        view_id: String,
        r:       f32,
        g:       f32,
        b:       f32,
        #[arg(default_value_t = 1.0)]
        a:       f32,
    },
    /// Set the UI scale of the app
    SetScale { scale: f32 },
    /// Play a sound in the app, to tell which instance is which
    PlaySound,
    /// List all edits applied to the app in this session
    Edits,
    /// Run the app's whole UI test suite in the app and report every failure
    RunTests,
}

// Responses hold Own pointers which must drop on the main thread. The
// current_thread runtime keeps everything on this thread.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    set_current_thread_as_main();

    let cli = Cli::parse();

    if let Command::Apps = cli.command {
        let apps = discover().await?;
        save_cache(&apps)?;
        if apps.is_empty() {
            bail!(NO_APPS);
        }
        for (id, addr) in &apps {
            println!("{id} at {addr}");
        }
        return Ok(());
    }

    let client = connect(cli.app).await?;

    match cli.command {
        Command::Apps => unreachable!(),
        Command::Ui => {
            let (scale, root) = get_ui(&client).await?;
            println!("{}", to_string_pretty(&json!({ "scale": scale, "root": root }))?);
        }
        Command::Tree => {
            let (_, root) = get_ui(&client).await?;
            print_tree(&root, 0);
        }
        Command::View { query } => {
            let (_, root) = get_ui(&client).await?;
            let mut found = vec![];
            find_matches(&root, &query, &mut found)?;
            if found.is_empty() {
                bail!("No view matches: {query}");
            }
            for view in found {
                println!("{}", to_string_pretty(&view)?);
            }
        }
        Command::Screenshot { out } => {
            let AppCommand::Screenshot {
                width,
                height,
                png_base64,
            } = send(&client, InspectorCommand::Screenshot).await?
            else {
                bail!("Unexpected response to screenshot");
            };
            let path = out.unwrap_or_else(|| temp_dir().join("te-screenshot.png"));
            write(&path, STANDARD.decode(png_base64)?)?;
            println!("{width}x{height} saved to {}", path.display());
        }
        Command::PlaySound => {
            send(&client, InspectorCommand::PlaySound).await?;
            println!("ok");
        }
        Command::RunTests => run_tests(&client).await?,
        Command::Edits => {
            let AppCommand::Edits(edits) = send(&client, InspectorCommand::ListEdits).await? else {
                bail!("Unexpected response to edits");
            };
            println!("{}", to_string_pretty(&edits)?);
        }
        Command::SetScale { scale } => {
            send(&client, UIRequest::SetScale(scale).into()).await?;
            println!("ok");
        }
        Command::EditRule {
            view_id,
            rule_index,
            offset,
            disable,
        } => {
            let request = UIRequest::EditRule {
                view_id: view_id.clone(),
                rule_index,
                offset,
                enabled: !disable,
            };
            print_edited(&client, request, &view_id).await?;
        }
        Command::SetText { view_id, text } => {
            let request = UIRequest::SetText {
                view_id: view_id.clone(),
                text,
            };
            print_edited(&client, request, &view_id).await?;
        }
        Command::SetColor { view_id, r, g, b, a } => {
            let request = UIRequest::SetColor {
                view_id: view_id.clone(),
                color:   Color::rgba(r, g, b, a),
            };
            print_edited(&client, request, &view_id).await?;
        }
    }

    Ok(())
}

async fn run_tests(client: &Client) -> Result<()> {
    let AppCommand::TestResults { total, failures } = send(client, InspectorCommand::RunTests).await? else {
        bail!("Unexpected response to run-tests");
    };

    println!("{total} tests, {} failed", failures.len());

    for failure in &failures {
        println!("\n===== {} =====\n{}", failure.name, failure.detail);
    }

    if !failures.is_empty() {
        exit(1);
    }

    Ok(())
}

async fn send(client: &Client, command: InspectorCommand) -> Result<AppCommand> {
    match client.send(command).await? {
        AppCommand::Error(err) => bail!("{err}"),
        response => Ok(response),
    }
}

async fn get_ui(client: &Client) -> Result<(f32, Own<ViewRepr>)> {
    let AppCommand::UI(UIResponse::SendUI { scale, root }) = send(client, UIRequest::GetUI.into()).await?
    else {
        bail!("Unexpected response to get ui");
    };
    Ok((scale, root))
}

/// Prints the fresh post-layout state of the edited view. The whole tree
/// would flood the output, `ui` prints it when needed.
async fn print_edited(client: &Client, request: UIRequest, view_id: &str) -> Result<()> {
    let AppCommand::UI(UIResponse::SendUI { root, .. }) = send(client, request.into()).await? else {
        bail!("Unexpected response to an edit");
    };

    let Some(view) = find_by_id(&root, view_id) else {
        bail!("Edited view {view_id} is gone from the fresh tree");
    };

    println!("{}", to_string_pretty(&view_json(view)?)?);

    Ok(())
}

fn print_tree(view: &ViewRepr, depth: usize) {
    let frame = &view.frame;
    println!(
        "{}{}  [{}, {}] {}x{}  {}",
        "  ".repeat(depth),
        view.label,
        frame.origin.x,
        frame.origin.y,
        frame.size.width,
        frame.size.height,
        view.id,
    );
    for sub in &view.subviews {
        print_tree(sub, depth + 1);
    }
}

fn find_by_id<'a>(view: &'a ViewRepr, id: &str) -> Option<&'a ViewRepr> {
    if view.id == id {
        return Some(view);
    }
    view.subviews.iter().find_map(|sub| find_by_id(sub, id))
}

fn find_matches(view: &ViewRepr, query: &str, found: &mut Vec<Value>) -> Result<()> {
    if view.id == query || view.label.to_lowercase().contains(&query.to_lowercase()) {
        found.push(view_json(view)?);
    }
    for sub in &view.subviews {
        find_matches(sub, query, found)?;
    }
    Ok(())
}

/// Full view JSON with the subview subtrees replaced by their labels,
/// so printing a container does not dump everything under it.
fn view_json(view: &ViewRepr) -> Result<Value> {
    let mut value = to_value(view)?;
    let labels: Vec<&str> = view.subviews.iter().map(|sub| sub.label.as_str()).collect();
    value["subviews"] = json!(labels);
    Ok(value)
}

/// Tries the address cached by the last discovery first and falls back to a
/// fresh mDNS browse, so repeat calls skip the discovery wait.
async fn connect(app: Option<String>) -> Result<Client> {
    if let Some(addr) = cached_addr(app.as_deref())
        && let Ok(Ok(client)) = timeout(Duration::from_secs(1), Client::connect(addr)).await
    {
        return Ok(client);
    }

    let apps = discover().await?;
    save_cache(&apps)?;
    let addr = resolve(&apps, app)?;

    Client::connect(addr).await
}

fn cache_path() -> PathBuf {
    temp_dir().join("te-inspect-apps.json")
}

fn cached_addr(app: Option<&str>) -> Option<SocketAddr> {
    let cache: HashMap<String, SocketAddr> = from_str(&read_to_string(cache_path()).ok()?).ok()?;
    match app {
        Some(id) => cache.get(id).copied(),
        // A single cached app can be trusted without a browse. With several,
        // discover every time, correctness over speed.
        None => {
            if cache.len() == 1 {
                cache.values().next().copied()
            } else {
                None
            }
        }
    }
}

fn save_cache(apps: &HashMap<String, SocketAddr>) -> Result<()> {
    write(cache_path(), to_string(apps)?)?;
    Ok(())
}

/// Browses mDNS until the deadline. Cuts the wait short when something is
/// found: waits a little longer after the first hit to catch the others,
/// then returns.
async fn discover() -> Result<HashMap<String, SocketAddr>> {
    let mdns = ServiceDaemon::new()?;
    let events = mdns.browse(SERVICE_TYPE)?;

    let mut apps = HashMap::new();

    let deadline = Instant::now() + Duration::from_secs(3);
    let mut cutoff = deadline;

    loop {
        let until = deadline.min(cutoff);

        let Ok(Ok(event)) = timeout_at(until, events.recv_async()).await else {
            break;
        };

        let ServiceEvent::ServiceResolved(service) = event else {
            continue;
        };

        let Some(app_id) = service.txt_properties.get_property_val_str("app_id") else {
            continue;
        };

        let ip = service
            .addresses
            .iter()
            .map(ScopedIp::to_ip_addr)
            .find(IpAddr::is_ipv4)
            .or_else(|| service.addresses.iter().next().map(ScopedIp::to_ip_addr));

        let Some(ip) = ip else {
            continue;
        };

        apps.insert(app_id.to_string(), SocketAddr::new(ip, service.port));
        cutoff = Instant::now() + Duration::from_millis(500);
    }

    Ok(apps)
}

fn resolve(apps: &HashMap<String, SocketAddr>, app: Option<String>) -> Result<SocketAddr> {
    let ids = || apps.keys().cloned().collect::<Vec<_>>().join(", ");

    if let Some(id) = app {
        return match apps.get(&id) {
            Some(addr) => Ok(*addr),
            None => bail!("App {id} not found. Running apps: {}", ids()),
        };
    }

    match apps.len() {
        0 => bail!(NO_APPS),
        1 => Ok(*apps.values().next().unwrap()),
        _ => bail!("Multiple apps running, pass --app. Running apps: {}", ids()),
    }
}
