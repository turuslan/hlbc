use std::cell::Cell;
use std::fs;
use std::io::BufReader;
use std::rc::Rc;

use eframe::egui;
use eframe::egui::{CentralPanel, Frame, Margin, ScrollArea, TopBottomPanel, Ui, Vec2};
use egui_dock::{DockArea, NodeIndex, Tree};
use poll_promise::Promise;

use hlbc::fmt::EnhancedFmt;
use hlbc::types::{RefFun, RefGlobal, RefString, RefType};
use hlbc::Bytecode;

use crate::views::{
    AppView, ClassesView, DynamicTabViewer, FunctionsView, GlobalsView, InfoView, StringsView,
    SyncInspectorView,
};

mod views;

pub struct App {
    /// Asynchronous loader for bytecode
    loader: Option<Promise<hlbc::Result<Option<(String, Bytecode)>>>>,
    /// Some when a file is loaded
    ctx: Option<AppCtxHandle>,
    // Dock
    tree: Tree<Box<dyn AppView>>,
    style: egui_dock::Style,
    options_window_open: bool,
    about_window_open: bool,
    status: String,
}

impl App {
    pub fn new(
        loader: Option<Promise<hlbc::Result<Option<(String, Bytecode)>>>>,
        style: egui_dock::Style,
    ) -> Self {
        Self {
            loader,
            ctx: None,
            tree: Tree::new(Vec::new()),
            style,
            options_window_open: false,
            about_window_open: false,
            status: String::from("Loading bytecode ..."),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        {
            if let Some(loader) = self.loader.take() {
                match loader.try_take() {
                    Ok(Ok(Some((file, code)))) => {
                        self.ctx = Some(AppCtxHandle::new(AppCtx::new_from_code(file, code)));
                        self.tree = default_tabs();
                        self.status = String::from("Loaded bytecode successfully");
                    }
                    Ok(Ok(None)) => {
                        // No file has been picked
                    }
                    Ok(Err(e)) => {
                        println!("{e}");
                    }
                    Err(loader) => {
                        self.loader = Some(loader);
                        ctx.request_repaint();
                    }
                }
            }

            if let Some(tab) = self.ctx.as_ref().and_then(|app| app.take_tab_to_open()) {
                self.tree[NodeIndex::root().right()].append_tab(tab);
            }
        }

        TopBottomPanel::top("menu bar")
            .frame(Frame::none().outer_margin(Margin::same(4.0)))
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {
                            #[cfg(target_arch = "wasm32")]
                            {
                                self.loader = Some(Promise::spawn_async(async {
                                    if let Some(file) =
                                        rfd::AsyncFileDialog::new().pick_file().await
                                    {
                                        Some((
                                            file.file_name(),
                                            Bytecode::deserialize(&mut &file.read().await[..])
                                                .unwrap(),
                                        ))
                                    } else {
                                        None
                                    }
                                }));
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                self.loader = Some(Promise::spawn_thread("bg_loader", || {
                                    if let Some(file) = rfd::FileDialog::new().pick_file() {
                                        Ok(Some((
                                            file.display().to_string(),
                                            Bytecode::deserialize(&mut BufReader::new(
                                                fs::File::open(&file)?,
                                            ))?,
                                        )))
                                    } else {
                                        Ok(None)
                                    }
                                }));
                            }
                        }
                        if ui.button("Close").clicked() {
                            self.ctx = None;
                            self.tree = Tree::new(vec![]);
                        }
                    });
                    if let Some(ctx) = &self.ctx {
                        ui.menu_button("Views", |ui| {
                            if ui.button("Functions").clicked() {
                                self.tree[NodeIndex::root().right()]
                                    .append_tab(Box::<FunctionsView>::default());
                            }
                            if ui.button("Info").clicked() {
                                self.tree[NodeIndex::root().right()]
                                    .append_tab(Box::<InfoView>::default());
                            }
                            #[cfg(feature = "search")]
                            if ui.button("Search").clicked() {
                                self.tree[NodeIndex::root().right()]
                                    .append_tab(Box::new(views::SearchView::new(ctx.code())));
                            }
                        });
                    }
                    if ui.button("Options").clicked() {
                        self.options_window_open = !self.options_window_open;
                    }
                    ui.menu_button("Help", |ui| {
                        if ui.button("Wiki").clicked() {
                            webbrowser::open("https://github.com/Gui-Yom/hlbc/wiki")
                                .expect("Failed to open web browser");
                        }
                    });
                    if ui.button("About").clicked() {
                        self.about_window_open = !self.about_window_open;
                    }
                });
            });

        TopBottomPanel::bottom("status bar")
            .exact_height(20.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(appctx) = &self.ctx {
                        let (id, rect) = ui.allocate_space(Vec2::new(120.0, 20.0));
                        Ui::new(ctx.clone(), ui.layer_id(), id, rect, rect)
                            .label(format!("{}", appctx.selected().name(appctx.code())));
                        ui.separator();
                    }
                    ui.label(&self.status);
                });
            });

        egui::Window::new("Options")
            .open(&mut self.options_window_open)
            .show(ctx, |ui| {
                ui.collapsing("Display", |ui| {
                    // TODO max fps
                    // TODO ui theme
                    #[cfg(debug_assertions)]
                    ScrollArea::vertical().show(ui, |ui| {
                        ctx.style_ui(ui);
                    });
                });
                ui.collapsing("Code display", |_ui| {
                    // TODO code font
                    // TODO code font size
                    // TODO code theme
                });
            });

        egui::Window::new("About")
            .open(&mut self.about_window_open)
            .show(ctx, |ui| {
                ui.heading("Hashlink bytecode tools");
                // TODO about page
            });

        if let Some(appctx) = self.ctx.clone() {
            DockArea::new(&mut self.tree)
                .scroll_area_in_tabs(false)
                .style(self.style.clone())
                .show(ctx, &mut DynamicTabViewer(appctx));
        } else {
            CentralPanel::default()
                .frame(Frame::group(ctx.style().as_ref()).outer_margin(Margin::same(4.0)))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Load a bytecode file to start");
                    });
                });
        }
    }
}

fn default_tabs() -> Tree<Box<dyn AppView>> {
    let mut tree: Tree<Box<dyn AppView>> = Tree::new(vec![
        Box::<SyncInspectorView>::default(),
        Box::<InfoView>::default(),
    ]);

    tree.split_left(
        NodeIndex::root(),
        0.2,
        vec![
            Box::<FunctionsView>::default(),
            Box::<ClassesView>::default(),
        ],
    );
    tree.split_below(
        NodeIndex::root().left(),
        0.5,
        vec![Box::<StringsView>::default(), Box::<GlobalsView>::default()],
    );

    tree
}

/// Cheaply cloneable, for single threaded usage.
///
/// Usage warning ! The methods 'lock' the inner RefCell immutably and mutably (RW lock).
/// Be careful of guards (Ref<> and RefMut<>) lifetimes.
#[derive(Clone)]
struct AppCtxHandle(Rc<AppCtx>);

impl AppCtxHandle {
    fn new(appctx: AppCtx) -> Self {
        Self(Rc::new(appctx))
    }

    fn file(&self) -> String {
        self.0.file.clone()
    }

    fn code(&self) -> &Bytecode {
        &self.0.code
    }

    /// mut lock
    fn open_tab(&self, tab: impl AppView + 'static) {
        self.0.new_tab.set(Some(Box::new(tab)));
    }

    fn take_tab_to_open(&self) -> Option<Box<dyn AppView>> {
        self.0.new_tab.take()
    }

    fn selected(&self) -> ItemSelection {
        self.0.selected.get()
    }

    fn set_selected(&self, s: ItemSelection) {
        self.0.selected.set(s);
    }
}

struct AppCtx {
    file: String,
    code: Bytecode,
    selected: Cell<ItemSelection>,
    /// To open a tab from another tab.
    /// This can't be done directly because this would need a mutable reference to a tree and the tree owns the tab.
    new_tab: Cell<Option<Box<dyn AppView>>>,
}

impl AppCtx {
    fn new_from_code(file: String, code: Bytecode) -> Self {
        Self {
            file,
            code,
            selected: Cell::new(ItemSelection::None),
            new_tab: Cell::new(None),
        }
    }
}

#[derive(Clone, Default, Copy, Eq, PartialEq)]
enum ItemSelection {
    Fun(RefFun),
    Class(RefType),
    Global(RefGlobal),
    String(RefString),
    #[default]
    None,
}

impl ItemSelection {
    pub(crate) fn name(&self, code: &Bytecode) -> String {
        match self {
            ItemSelection::Fun(fun) => fun.display::<EnhancedFmt>(code).to_string(),
            ItemSelection::Class(t) => t.display::<EnhancedFmt>(code).to_string(),
            ItemSelection::Global(g) => format!("global@{}", g.0),
            ItemSelection::String(s) => {
                format!("string@{}", s.0)
            }
            _ => String::new(),
        }
    }
}
