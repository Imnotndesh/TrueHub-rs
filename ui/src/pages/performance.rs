use gtk4::prelude::*;
use gtk4::{
    Stack, Orientation, Align, ScrolledWindow, PolicyType,
    Box as GBox, StackTransitionType, DrawingArea,
};
use libadwaita::prelude::*;
use libadwaita::{
    HeaderBar, ToolbarView,
    ActionRow, PreferencesGroup,
    ViewStack, ViewSwitcher, ViewSwitcherPolicy,
    ViewSwitcherBar,
};
use glib::clone;
use crate::state::AppState;
use crate::runtime;
use api::models::system::{
    ReportingGraphRequest, ReportingGraphName, ReportingGraphQuery,
    ReportingUnit, ReportingGraphResponse,
};
use api::result::ApiResult;
use api::methods::System as SystemMethods;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum MetricType {
    Cpu,
    Memory,
    Temperature,
}

#[derive(Clone, Default)]
struct MetricData {
    cpu: Option<ReportingGraphResponse>,
    memory: Option<ReportingGraphResponse>,
    temperature: Option<ReportingGraphResponse>,
}

pub fn build(
    state: AppState,
    initial_metric: MetricType,
    nav_stack: Stack,
) -> ToolbarView {
    let toolbar_view = ToolbarView::new();

    let header = HeaderBar::new();
    let title = gtk4::Label::builder()
        .label("Performance")
        .css_classes(vec!["title"])
        .build();
    header.set_title_widget(Some(&title));

    let back_btn = gtk4::Button::builder()
        .icon_name("go-previous-symbolic")
        .tooltip_text("Back")
        .build();
    header.pack_start(&back_btn);

    let refresh_btn = gtk4::Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text("Refresh")
        .build();
    header.pack_end(&refresh_btn);

    toolbar_view.add_top_bar(&header);

    let nav = nav_stack.clone();
    back_btn.connect_clicked(move |_| {
        nav.set_transition_type(StackTransitionType::SlideRight);
        nav.set_visible_child_name("home");
    });

    let metric_stack = ViewStack::new();
    let switcher = ViewSwitcher::builder()
        .stack(&metric_stack)
        .policy(ViewSwitcherPolicy::Wide)
        .build();
    header.set_title_widget(Some(&switcher));   // keep the top switcher; you can remove this if you prefer bottom

    let switcher_bar = ViewSwitcherBar::new();
    switcher_bar.set_stack(Some(&metric_stack));
    switcher_bar.set_reveal(true);

    let loading_box = GBox::builder()
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .vexpand(true)
        .build();
    let spinner = gtk4::Spinner::new();
    spinner.start();
    loading_box.append(&spinner);
    loading_box.append(
        &gtk4::Label::builder()
            .label("Fetching performance data…")
            .css_classes(vec!["dim-label"])
            .build()
    );

    let page_stack = Stack::new();
    page_stack.set_transition_type(StackTransitionType::Crossfade);
    page_stack.set_transition_duration(250);
    page_stack.add_named(&loading_box, Some("loading"));
    page_stack.add_named(&metric_stack, Some("content"));
    page_stack.set_visible_child_name("loading");

    // IMPORTANT: Set page_stack directly as content – no outer wrapper
    toolbar_view.set_content(Some(&page_stack));
    toolbar_view.add_bottom_bar(&switcher_bar);

    let data: Arc<Mutex<MetricData>> = Arc::new(Mutex::new(MetricData::default()));

    let load = {
        let state = state.clone();
        let page_stack = page_stack.clone();
        let metric_stack = metric_stack.clone();
        let data = data.clone();
        let initial_metric = initial_metric.clone();

        move || {
            page_stack.set_visible_child_name("loading");

            let (tx, rx) = async_channel::unbounded::<MetricData>();

            glib::MainContext::default().spawn_local(clone!(
                #[strong] page_stack,
                #[strong] metric_stack,
                #[strong] data,
                #[strong] initial_metric,
                async move {
                    if let Ok(new_data) = rx.recv().await {
                        *data.lock().unwrap() = new_data.clone();
                        populate_metric_stack(&metric_stack, &new_data, &initial_metric);
                        page_stack.set_visible_child_name("content");
                    }
                }
            ));

            let client = {
                let lock = state.manager.lock().unwrap();
                lock.as_ref().map(|m| m.client.clone())
            };

            if let Some(client) = client {
                runtime::spawn(
                    async move {
                        let query = ReportingGraphQuery {
                            unit: Some(ReportingUnit::Hour),
                            aggregate: Some(true),
                            ..Default::default()
                        };

                        let fetch = |name: ReportingGraphName| {
                            let client = client.clone();
                            let query = query.clone();
                            async move {
                                let graphs = vec![ReportingGraphRequest::new(name, None)];
                                match client.call::<Vec<ReportingGraphResponse>>(
                                    SystemMethods::GET_GRAPH_DATA,
                                    vec![serde_json::json!(graphs), serde_json::json!(query)],
                                ).await {
                                    ApiResult::Success(mut v) => v.pop(),
                                    _ => None,
                                }
                            }
                        };

                        let (cpu, memory, temperature) = tokio::join!(
                            fetch(ReportingGraphName::Cpu),
                            fetch(ReportingGraphName::Memory),
                            fetch(ReportingGraphName::CpuTemp),
                        );

                        MetricData { cpu, memory, temperature }
                    },
                    tx,
                );
            } else {
                let _ = tx.send_blocking(MetricData::default());
            }
        }
    };

    load();

    {
        let load = load.clone();
        refresh_btn.connect_clicked(move |_| load());
    }

    toolbar_view
}

fn populate_metric_stack(
    stack: &ViewStack,
    data: &MetricData,
    initial: &MetricType,
) {
    while let Some(child) = stack.first_child() {
        stack.remove(&child);
    }

    if let Some(cpu) = &data.cpu {
        let page = metric_page(cpu, "CPU Usage", "%", false, "cpu");
        stack.add_titled_with_icon(&page, Some("cpu"), "CPU", "processor-symbolic");
    }

    if let Some(mem) = &data.memory {
        let page = metric_page(mem, "Memory Usage", "GB", true, "memory");
        stack.add_titled_with_icon(&page, Some("memory"), "Memory", "memory-chip-symbolic");
    }

    if let Some(temp) = &data.temperature {
        let page = metric_page(temp, "CPU Temperature", "°C", false, "temp");
        stack.add_titled_with_icon(&page, Some("temp"), "Temp", "weather-clear-symbolic");
    }

    let name = match initial {
        MetricType::Cpu => "cpu",
        MetricType::Memory => "memory",
        MetricType::Temperature => "temp",
    };
    if stack.child_by_name(name).is_some() {
        stack.set_visible_child_name(name);
    }
}

fn metric_page(
    data: &ReportingGraphResponse,
    title: &str,
    unit: &str,
    is_memory: bool,
    _id: &str,
) -> GBox {
    let page = GBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .vexpand(true)
        .build();

    let inner = GBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .margin_top(12)
        .margin_bottom(24)
        .margin_start(12)
        .margin_end(12)
        .build();

    inner.append(&current_value_group(data, title, unit, is_memory));
    inner.append(&spacer(12));
    inner.append(&chart_group(data, unit, is_memory));
    inner.append(&spacer(12));
    inner.append(&stats_group(data, unit, is_memory));

    scroll.set_child(Some(&inner));
    page.append(&scroll);
    page
}

fn current_value_group(
    data: &ReportingGraphResponse,
    title: &str,
    unit: &str,
    is_memory: bool,
) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title(title);
    group.set_description(Some(&format!("{} data points collected", data.data.len())));

    let current = data.data.last()
        .and_then(|row| row.get(1).copied())
        .unwrap_or(0.0);

    let formatted = format_value(current, unit, is_memory);

    let current_row = ActionRow::builder()
        .title("Current")
        .build();
    current_row.set_icon_name(Some("speedometer-symbolic"));
    current_row.add_suffix(&right_label(&formatted, &["title-4"]));
    group.add(&current_row);

    let points: Vec<f64> = data.data.iter()
        .filter_map(|row| row.get(1).copied())
        .filter(|v| v.is_finite())
        .collect();

    if !points.is_empty() {
        let avg = points.iter().sum::<f64>() / points.len() as f64;
        let min = points.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = points.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        let avg_row = ActionRow::builder().title("Average").build();
        avg_row.add_suffix(&right_label(&format_value(avg, unit, is_memory), &["caption"]));
        group.add(&avg_row);

        let min_row = ActionRow::builder().title("Minimum").build();
        min_row.add_suffix(&right_label(&format_value(min, unit, is_memory), &["caption", "success"]));
        group.add(&min_row);

        let max_row = ActionRow::builder().title("Maximum").build();
        max_row.add_suffix(&right_label(&format_value(max, unit, is_memory), &["caption", "warning"]));
        group.add(&max_row);
    }

    group
}

fn chart_group(
    data: &ReportingGraphResponse,
    unit: &str,
    is_memory: bool,
) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("History");

    let chart = build_line_chart(&data.data, unit, is_memory);
    let chart_row = libadwaita::PreferencesRow::new();
    let wrapper = GBox::builder()
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();
    wrapper.append(&chart);
    chart_row.set_child(Some(&wrapper));
    group.add(&chart_row);

    group
}

fn stats_group(
    data: &ReportingGraphResponse,
    unit: &str,
    is_memory: bool,
) -> PreferencesGroup {
    let group = PreferencesGroup::new();
    group.set_title("Metadata");

    let name_row = ActionRow::builder().title("Graph Name").build();
    name_row.add_suffix(&right_label(&data.name, &["caption", "dim-label"]));
    group.add(&name_row);

    let range_row = ActionRow::builder().title("Time Range").build();
    range_row.add_suffix(&right_label(
        &format!("{} → {}", data.start, data.end),
        &["caption", "monospace", "dim-label"],
    ));
    group.add(&range_row);

    for legend in &data.legend {
        if legend != "time" {
            let leg_row = ActionRow::builder().title("Legend").build();
            leg_row.add_suffix(&right_label(legend, &["caption"]));
            group.add(&leg_row);
        }
    }

    group
}

fn build_line_chart(
    points: &Vec<Vec<f64>>,
    unit: &str,
    is_memory: bool,
) -> DrawingArea {
    let chart_data: Vec<(f64, f64)> = points.iter()
        .filter_map(|row| {
            let t = row.get(0).copied()?;
            let v = row.get(1).copied()?;
            if t.is_finite() && v.is_finite() { Some((t, v)) } else { None }
        })
        .collect();

    let chart_data = Arc::new(chart_data);
    let unit = unit.to_string();

    let drawing = DrawingArea::builder()
        .height_request(200)
        .hexpand(true)
        .build();

    drawing.set_draw_func(move |_, cr, width, height| {
        let w = width as f64;
        let h = height as f64;

        cr.set_source_rgba(0.0, 0.0, 0.0, 0.05);
        cr.rectangle(0.0, 0.0, w, h);
        let _ = cr.fill();

        if chart_data.is_empty() {
            cr.set_source_rgba(0.5, 0.5, 0.5, 0.6);
            cr.move_to(w / 2.0 - 40.0, h / 2.0);
            let _ = cr.show_text("No data");
            return;
        }

        let pad = 8.0;
        let values: Vec<f64> = chart_data.iter().map(|(_, v)| *v).collect();
        let min_v = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_v = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = if (max_v - min_v).abs() < 1e-9 { 1.0 } else { max_v - min_v };

        let n = chart_data.len();

        cr.set_source_rgba(0.25, 0.52, 0.96, 0.15);
        cr.move_to(pad, h - pad);
        for (i, (_, v)) in chart_data.iter().enumerate() {
            let x = pad + (i as f64 / (n - 1).max(1) as f64) * (w - 2.0 * pad);
            let y = h - pad - ((v - min_v) / range) * (h - 2.0 * pad);
            cr.line_to(x, y);
        }
        cr.line_to(w - pad, h - pad);
        cr.close_path();
        let _ = cr.fill();

        cr.set_source_rgba(0.25, 0.52, 0.96, 1.0);
        cr.set_line_width(2.0);

        let (first_t, first_v) = chart_data[0];
        let x0 = pad;
        let y0 = h - pad - ((first_v - min_v) / range) * (h - 2.0 * pad);
        cr.move_to(x0, y0);

        for (i, (_, v)) in chart_data.iter().enumerate().skip(1) {
            let x = pad + (i as f64 / (n - 1).max(1) as f64) * (w - 2.0 * pad);
            let y = h - pad - ((v - min_v) / range) * (h - 2.0 * pad);
            cr.line_to(x, y);
        }
        let _ = cr.stroke();

        if let Some((_, last_v)) = chart_data.last() {
            let x = w - pad;
            let y = h - pad - ((last_v - min_v) / range) * (h - 2.0 * pad);
            cr.set_source_rgba(0.25, 0.52, 0.96, 1.0);
            cr.arc(x, y, 4.0, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();
        }

        cr.set_source_rgba(0.5, 0.5, 0.5, 0.8);
        cr.set_font_size(10.0);
        cr.move_to(pad, h - 2.0);
        let _ = cr.show_text(&format!("{:.1}{}", min_v, unit));
        cr.move_to(pad, 12.0);
        let _ = cr.show_text(&format!("{:.1}{}", max_v, unit));
    });

    drawing
}

fn format_value(value: f64, unit: &str, is_memory: bool) -> String {
    if is_memory {
        format!("{:.2} {}", value / (1024.0 * 1024.0 * 1024.0), unit)
    } else {
        format!("{:.2}{}", value, unit)
    }
}

fn right_label(text: &str, css_classes: &[&str]) -> gtk4::Label {
    let lbl = gtk4::Label::builder()
        .label(text).valign(Align::Center).halign(Align::End).build();
    for c in css_classes { lbl.add_css_class(c); }
    lbl
}

fn spacer(height: i32) -> gtk4::Box {
    let b = gtk4::Box::new(Orientation::Vertical, 0);
    b.set_height_request(height);
    b
}