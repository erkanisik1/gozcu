slint::include_modules!();

use slint::VecModel;
use std::{cell::Cell, rc::Rc};
use sysinfo::{Pid, ProcessesToUpdate, Signal, System, Users};

// ---- Dil ----

#[derive(Clone, Copy, PartialEq)]
enum Lang {
    Tr,
    En,
}

impl Lang {
    fn index(&self) -> i32 {
        match self {
            Lang::Tr => 0,
            Lang::En => 1,
        }
    }

    fn from_name(name: &str) -> Option<Lang> {
        match name {
            "Türkçe" => Some(Lang::Tr),
            "English" => Some(Lang::En),
            _ => None,
        }
    }

    fn detect() -> Lang {
        let loc = std::env::var("LANG")
            .or_else(|_| std::env::var("LC_ALL"))
            .unwrap_or_default()
            .to_lowercase();
        if loc.starts_with("en") {
            Lang::En
        } else {
            Lang::Tr // öntanımlı Türkçe
        }
    }

    fn apply(&self) {
        // .slint içindeki @tr dizelerini seçilen dile geçir
        match self {
            Lang::En => {
                let _ = slint::select_bundled_translation("en");
            }
            Lang::Tr => {
                let _ = slint::select_bundled_translation("");
            }
        }
    }

    fn state_text(&self, s: sysinfo::ProcessStatus) -> &'static str {
        use sysinfo::ProcessStatus::*;
        match (self, s) {
            (Lang::Tr, Run) => "Çalışıyor",
            (Lang::Tr, Sleep) => "Uykuda",
            (Lang::Tr, Stop) => "Durduruldu",
            (Lang::Tr, Zombie) => "Zombi",
            (Lang::Tr, Idle) => "Boşta",
            (Lang::Tr, Dead) => "Ölü",
            (Lang::Tr, Tracing) => "İzleniyor",
            (Lang::Tr, Parked) => "Beklemede",
            (Lang::Tr, UninterruptibleDiskSleep) => "Disk bekliyor",
            (Lang::Tr, Waking) | (Lang::Tr, Wakekill) => "Uyanıyor",
            (Lang::En, Run) => "Running",
            (Lang::En, Sleep) => "Sleeping",
            (Lang::En, Stop) => "Stopped",
            (Lang::En, Zombie) => "Zombie",
            (Lang::En, Idle) => "Idle",
            (Lang::En, Dead) => "Dead",
            (Lang::En, Tracing) => "Tracing",
            (Lang::En, Parked) => "Parked",
            (Lang::En, UninterruptibleDiskSleep) => "Disk wait",
            (Lang::En, Waking) | (Lang::En, Wakekill) => "Waking",
            _ => "?",
        }
    }

    fn signal_label(&self, sig: &str) -> &'static str {
        match (self, sig) {
            (Lang::Tr, "kill") => "SIGKILL (öldür)",
            (Lang::Tr, "term") => "SIGTERM (sonlandır)",
            (Lang::Tr, "stop") => "SIGSTOP (duraklat)",
            (Lang::Tr, "cont") => "SIGCONT (devam)",
            (Lang::En, "kill") => "SIGKILL (kill)",
            (Lang::En, "term") => "SIGTERM (terminate)",
            (Lang::En, "stop") => "SIGSTOP (pause)",
            (Lang::En, "cont") => "SIGCONT (resume)",
            _ => "signal",
        }
    }
}

// ---- Sıralama ----

#[derive(Clone, Copy, PartialEq)]
enum SortKey {
    Cpu,
    Memory,
    Pid,
}

impl SortKey {
    fn next(self) -> Self {
        match self {
            SortKey::Cpu => SortKey::Memory,
            SortKey::Memory => SortKey::Pid,
            SortKey::Pid => SortKey::Cpu,
        }
    }
    fn label(&self, lang: Lang) -> &'static str {
        match (lang, self) {
            (Lang::Tr, SortKey::Cpu) => "CPU",
            (Lang::Tr, SortKey::Memory) => "BELLEK",
            (Lang::Tr, SortKey::Pid) => "PID",
            (Lang::En, SortKey::Cpu) => "CPU",
            (Lang::En, SortKey::Memory) => "MEMORY",
            (Lang::En, SortKey::Pid) => "PID",
        }
    }
}

/// Durum: süreç bilgisi + önbelleğe alınmış kullanıcı listesi + dil.
struct Refresher {
    sys: System,
    users: Users,
    users_at: std::time::Instant,
    last_sig: u64,
    lang: Lang,
}

impl Refresher {
    fn new(lang: Lang) -> Self {
        let sys = System::new();
        Refresher {
            sys,
            users: Users::new_with_refreshed_list(),
            users_at: std::time::Instant::now(),
            last_sig: 0,
            lang,
        }
    }

    fn refresh(&mut self, ui: &AppWindow, model: &VecModel<ProcessItem>, sort: SortKey) {
        // with_user olmadan süreçlerin kullanıcı ID'si okunmaz (KULLANICI sütunu boş kalır)
        let kind = sysinfo::ProcessRefreshKind::nothing()
            .with_memory()
            .with_cpu()
            .with_user(sysinfo::UpdateKind::OnlyIfNotSet);
        self.sys
            .refresh_processes_specifics(ProcessesToUpdate::All, true, kind);
        if self.users_at.elapsed() >= std::time::Duration::from_secs(60) {
            self.users = Users::new_with_refreshed_list();
            self.users_at = std::time::Instant::now();
        }
        let users = &self.users;
        let filter = ui.get_filter().to_lowercase();
        let lang = self.lang;

        let mut rows: Vec<(u32, String, String, f32, f64, &'static str)> = self
            .sys
            .processes()
            .iter()
            .filter(|(_, p)| {
                if filter.is_empty() {
                    return true;
                }
                let name = p.name().to_string_lossy().to_lowercase();
                let hit_name = name.contains(&filter);
                let hit_pid = p.pid().as_u32().to_string().contains(&filter);
                let hit_user = p
                    .user_id()
                    .and_then(|uid| users.get_user_by_id(uid))
                    .map(|u| u.name().to_lowercase().contains(&filter))
                    .unwrap_or(false);
                hit_name || hit_user || hit_pid
            })
            .map(|(pid, p)| {
                let user = p
                    .user_id()
                    .and_then(|uid| users.get_user_by_id(uid))
                    .map(|u| u.name().to_string())
                    .unwrap_or_else(|| "?".into());
                (
                    pid.as_u32(),
                    p.name().to_string_lossy().to_string(),
                    user,
                    p.cpu_usage(),
                    p.memory() as f64 / 1024.0 / 1024.0,
                    lang.state_text(p.status()),
                )
            })
            .collect();

        match sort {
            SortKey::Cpu => rows.sort_by(|a, b| {
                b.3.partial_cmp(&a.3)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal))
            }),
            SortKey::Memory => rows.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal)),
            SortKey::Pid => rows.sort_by(|a, b| a.0.cmp(&b.0)),
        }

        let items: Vec<ProcessItem> = rows
            .into_iter()
            .map(|(pid, name, user, cpu, mem, state)| ProcessItem {
                pid: pid as i32,
                name: name.into(),
                user: user.into(),
                cpu: format!("{:.1}", cpu).into(),
                mem: format!("{:.1}", mem).into(),
                state: state.into(),
            })
            .collect();

        let selected = ui.get_selected_pid();
        if selected > 0 && items.iter().all(|i| i.pid != selected) {
            ui.set_selected_pid(0);
        }

        // Veri değişmediyse arayüzü yeniden kurma (gereksiz yeniden çizim yükünden kaçınır)
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        for i in &items {
            (i.pid, i.cpu.as_str(), i.mem.as_str(), i.state.as_str()).hash(&mut h);
        }
        let sig = h.finish();
        if sig != self.last_sig {
            self.last_sig = sig;
            model.set_vec(items);
        }
    }
}

fn signal_of(sig: &str) -> Signal {
    match sig {
        "kill" => Signal::Kill,
        "term" => Signal::Term,
        "stop" => Signal::Stop,
        "cont" => Signal::Continue,
        _ => Signal::Term,
    }
}

fn signal_name(sig: &str) -> &'static str {
    match sig {
        "kill" => "KILL",
        "stop" => "STOP",
        "cont" => "CONT",
        _ => "TERM",
    }
}

/// root (veya başka bir kullanıcı) sahibi süreçlere pkexec üzerinden sinyal gönderir.
/// KDE'de polkit-kde-agent parola penceresi açar; parola doğrulanırsa sinyal root olarak gönderilir.
fn send_signal_pkexec(pid: i32, sig: &str) -> Result<(), String> {
    let kill = if std::path::Path::new("/bin/kill").exists() {
        "/bin/kill"
    } else {
        "kill"
    };
    let out = std::process::Command::new("pkexec")
        .arg(kill)
        .arg("-s")
        .arg(signal_name(sig))
        .arg(pid.to_string())
        .output()
        .map_err(|e| format!("pkexec çalıştırılamadı: {e}"))?;
    if out.status.success() {
        return Ok(());
    }
    let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
    if err.is_empty() {
        Err(format!("yetki alınamadı (pkexec kod {})", out.status))
    } else {
        Err(err)
    }
}

fn send_signal(ui: &AppWindow, sys: &mut System, pid: i32, sig: &str, lang: Lang) {
    if pid <= 0 {
        ui.set_status(
            match lang {
                Lang::Tr => "Önce listeden bir süreç seçin",
                Lang::En => "Select a process from the list first",
            }
            .into(),
        );
        ui.set_status_error(true);
        return;
    }
    let label = lang.signal_label(sig);
    let euid = unsafe { libc::geteuid() };
    match sys.process(Pid::from_u32(pid as u32)) {
        Some(p) => {
            let name = p.name().to_string_lossy().to_string();
            let target_uid = p.user_id().map(|u| **u);
            // Başka kullanıcıya (root dahil) ait süreçlerde sinyal yalnızca yetki yükseltmeyle gönderilebilir
            let needs_elevated = euid != 0 && target_uid != Some(euid);
            let result = if needs_elevated {
                send_signal_pkexec(pid, sig)
            } else {
                match p.kill_with(signal_of(sig)) {
                    Some(true) => Ok(()),
                    _ => Err(match lang {
                        Lang::Tr => "yetki yok".to_string(),
                        Lang::En => "permission denied".to_string(),
                    }),
                }
            };
            match result {
                Ok(()) => {
                    ui.set_status(
                        if needs_elevated {
                            match lang {
                                Lang::Tr => format!("{} (PID {}) sürecine {} gönderildi (pkexec ile yetki yükseltildi)", name, pid, label),
                                Lang::En => format!("{} sent to {} (PID {}) (elevated via pkexec)", label, name, pid),
                            }
                        } else {
                            match lang {
                                Lang::Tr => format!("{} (PID {}) sürecine {} gönderildi", name, pid, label),
                                Lang::En => format!("{} sent to {} (PID {})", label, name, pid),
                            }
                        }
                        .into(),
                    );
                    ui.set_status_error(false);
                }
                Err(e) => {
                    ui.set_status(
                        match lang {
                            Lang::Tr => format!("{} gönderilemedi — {}", label, e),
                            Lang::En => format!("Could not send {} — {}", label, e),
                        }
                        .into(),
                    );
                    ui.set_status_error(true);
                }
            }
        }
        None => {
            ui.set_status(
                match lang {
                    Lang::Tr => format!("PID {} bulunamadı (süreç bitmiş olabilir)", pid),
                    Lang::En => format!("PID {} not found (process may have exited)", pid),
                }
                .into(),
            );
            ui.set_status_error(true);
        }
    }
}

fn main() {
    let lang = Rc::new(Cell::new(Lang::detect()));
    lang.get().apply();

    let ui = AppWindow::new().unwrap();
    let state = Rc::new(std::cell::RefCell::new(Refresher::new(lang.get())));
    let model = Rc::new(VecModel::<ProcessItem>::default());
    ui.set_processes(model.clone().into());
    let sort = Rc::new(Cell::new(SortKey::Cpu));
    ui.set_sort_label(sort.get().label(lang.get()).into());
    ui.set_lang_index(lang.get().index());
    ui.set_status(
        match lang.get() {
            Lang::Tr => "Hazır — bir süreç seçip eylem düğmelerini kullanın",
            Lang::En => "Ready — right-click a process for actions",
        }
        .into(),
    );

    // Yenile
    {
        let weak = ui.as_weak();
        let state = state.clone();
        let model = model.clone();
        let sort = sort.clone();
        ui.on_refresh_requested(move || {
            let ui = weak.unwrap();
            state.borrow_mut().refresh(&ui, &model, sort.get());
        });
    }

    // Sıralama değiştir
    {
        let weak = ui.as_weak();
        let state = state.clone();
        let model = model.clone();
        let sort = sort.clone();
        let lang = lang.clone();
        ui.on_sort_clicked(move || {
            let ui = weak.unwrap();
            sort.set(sort.get().next());
            let label = sort.get().label(lang.get());
            ui.set_sort_label(label.into());
            ui.set_status(
                match lang.get() {
                    Lang::Tr => format!("Sıralama: {}", label),
                    Lang::En => format!("Sort: {}", label),
                }
                .into(),
            );
            ui.set_status_error(false);
            state.borrow_mut().refresh(&ui, &model, sort.get());
        });
    }

    // Dil değiştir
    {
        let weak = ui.as_weak();
        let state = state.clone();
        let model = model.clone();
        let sort = sort.clone();
        let lang = lang.clone();
        ui.on_language_selected(move |name| {
            let ui = weak.unwrap();
            let new_lang = match Lang::from_name(&name) {
                Some(l) => l,
                None => return, // tanınmayan dil; listede yok
            };
            if new_lang == lang.get() {
                return;
            }
            lang.set(new_lang);
            new_lang.apply();
            state.borrow_mut().lang = new_lang;
            ui.set_lang_index(new_lang.index());
            ui.set_sort_label(sort.get().label(new_lang).into());
            ui.set_status(
                match new_lang {
                    Lang::Tr => "Dil: Türkçe",
                    Lang::En => "Language: English",
                }
                .into(),
            );
            ui.set_status_error(false);
            state.borrow_mut().refresh(&ui, &model, sort.get());
        });
    }

    // Sinyal gönder
    {
        let weak = ui.as_weak();
        let state = state.clone();
        let model = model.clone();
        let sort = sort.clone();
        let lang = lang.clone();
        ui.on_signal_requested(move |pid, sig| {
            let ui = weak.unwrap();
            let sig = sig.to_string();
            send_signal(&ui, &mut state.borrow_mut().sys, pid, &sig, lang.get());
            state.borrow_mut().refresh(&ui, &model, sort.get());
        });
    }

    // İlk listeleme
    // İsteğe bağlı: `gozcu --filter firefox` ile arama önceden uygulanarak açılır
    let args: Vec<String> = std::env::args().collect();
    if let Some(pos) = args.iter().position(|a| a == "--filter") {
        if let Some(f) = args.get(pos + 1) {
            ui.set_filter(f.clone().into());
        }
    }
    state.borrow_mut().refresh(&ui, &model, sort.get());

    // Saniyede bir otomatik yenile
    {
        let weak = ui.as_weak();
        let state = state.clone();
        let model = model.clone();
        let sort = sort.clone();
        let timer = slint::Timer::default();
        timer.start(
            slint::TimerMode::Repeated,
            std::time::Duration::from_secs(1),
            move || {
                if let Some(ui) = weak.upgrade() {
                    state.borrow_mut().refresh(&ui, &model, sort.get());
                }
            },
        );
        // timer'ı canlı tutmak için akışın sonuna kadar sakla
        std::mem::forget(timer);
    }

    ui.run().unwrap();
}
