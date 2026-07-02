#![cfg(not_wasm)]

use audio::Sound;
use hreads::{from_main, log_spawn, on_main};
use inspect::{AppCommand, InspectorCommand, SERVICE_TYPE, UIRequest, UIResponse, serve};
use log::info;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use refs::manage::DataManager;
use tokio::net::TcpListener;
use ui::{UIManager, ViewData, ViewSubviews, WeakView};

use crate::inspect::view_conversion::{ViewToInspect, weak_to_id};

pub struct InspectService;

impl InspectService {
    pub fn start_listening() {
        log_spawn(async {
            let listener = TcpListener::bind("0.0.0.0:0").await?;
            let port = listener.local_addr()?.port();

            let app_id = UIManager::app_instance_id();

            let mdns = ServiceDaemon::new()?;
            let service = ServiceInfo::new(
                SERVICE_TYPE,
                app_id,
                &format!("{app_id}.local."),
                "",
                port,
                &[("app_id", app_id)][..],
            )?
            .enable_addr_auto();
            mdns.register(service)?;

            info!("Inspect server on port: {port}");

            serve(listener, Self::process_command).await
        });
    }

    fn process_command(command: InspectorCommand) -> AppCommand {
        match command {
            InspectorCommand::PlaySound => {
                on_main(|| {
                    Sound::get("retro.wav").play();
                });

                AppCommand::Ok
            }
            InspectorCommand::UI(ui) => Self::process_ui_command(ui),
        }
    }

    fn process_ui_command(command: UIRequest) -> AppCommand {
        match command {
            UIRequest::SetScale(scale) => {
                from_main(move || {
                    UIManager::set_scale(scale);
                });

                // send_ui dispatches a fresh from_main, so the snapshot runs
                // one frame later, after layout applied the new scale.
                Self::send_ui()
            }
            UIRequest::GetUI => Self::send_ui(),
            UIRequest::EditRule {
                view_id,
                rule_index,
                offset,
                enabled,
            } => {
                from_main(move || {
                    let Some(view) = find_view(UIManager::root_view(), &view_id) else {
                        return;
                    };

                    if rule_index >= view.place().get_rules().len() {
                        return;
                    }

                    let mut rule = view.place().edit_rule(rule_index);
                    rule.set_offset(offset);
                    rule.enabled = enabled;
                });

                Self::send_ui()
            }
        }
    }

    fn send_ui() -> AppCommand {
        from_main(|| {
            let scale = UIManager::scale();
            let root = UIManager::root_view().view_to_inspect();
            UIResponse::SendUI { scale, root }.into()
        })
    }
}

fn find_view(view: WeakView, id: &str) -> Option<WeakView> {
    if weak_to_id(view) == id {
        return Some(view);
    }

    view.subviews().iter().find_map(|sub| find_view(sub.weak(), id))
}
