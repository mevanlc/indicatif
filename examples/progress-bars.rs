use std::thread;
use std::time::Duration;

use clap::{Parser, ValueEnum};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

const STEPS: u64 = 12;
const STEP_DELAY: Duration = Duration::from_millis(70);
const TICK_INTERVAL: Duration = Duration::from_millis(80);
const SCENE_PAUSE: Duration = Duration::from_millis(350);

const ALL_SCENES: &[Scene] = &[
    Scene::Single,
    Scene::Indeterminate,
    Scene::Transition,
    Scene::Multiple,
    Scene::Lifecycle,
    Scene::Inline,
    Scene::Color,
    Scene::Templates,
];

/// A visual tour of progress-bar rendering, layout, and lifecycle behavior.
#[derive(Debug, Parser)]
#[command(about, long_about = None)]
struct Args {
    /// Print available scenes and exit
    #[arg(long)]
    list: bool,

    /// Run only this scene; pass more than once to run several scenes
    #[arg(long = "scene", value_enum, value_name = "SCENE")]
    scenes: Vec<Scene>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum Scene {
    /// One standalone determinate progress bar
    Single,
    /// A steady-ticking bar without a known length
    Indeterminate,
    /// Switching a bar between indeterminate and determinate modes
    Transition,
    /// Stacked bars with explicit insertion order
    Multiple,
    /// Retained versus transient completed bars
    Lifecycle,
    /// Log lines emitted safely while bars are active
    Inline,
    /// Filled and remaining segments with distinct colors
    Color,
    /// Prefixes, messages, metrics, and wide layout elements
    Templates,
}

impl Scene {
    fn name(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Indeterminate => "indeterminate",
            Self::Transition => "transition",
            Self::Multiple => "multiple",
            Self::Lifecycle => "lifecycle",
            Self::Inline => "inline",
            Self::Color => "color",
            Self::Templates => "templates",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Single => "one standalone determinate progress bar",
            Self::Indeterminate => "a steady-ticking spinner with no known length",
            Self::Transition => "switching between unknown and known lengths",
            Self::Multiple => "stacked bars with explicit insertion order",
            Self::Lifecycle => "retained versus transient completed bars",
            Self::Inline => "safe log output while progress bars are active",
            Self::Color => "two-color bar segments and colored rows",
            Self::Templates => "text, metrics, and wide template placement",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Single => "Single determinate bar",
            Self::Indeterminate => "Indeterminate bar",
            Self::Transition => "Indeterminate/determinate transition",
            Self::Multiple => "Multiple stacked bars",
            Self::Lifecycle => "Retained and transient completion",
            Self::Inline => "Inline progress-safe output",
            Self::Color => "Color and style",
            Self::Templates => "Template text placement",
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.list {
        list_scenes();
        return;
    }

    let scenes = if args.scenes.is_empty() {
        ALL_SCENES
    } else {
        &args.scenes
    };

    for scene in scenes {
        run_scene(*scene);
    }
}

fn list_scenes() {
    println!("Available scenes:");
    for scene in ALL_SCENES {
        println!("  {:<15} {}", scene.name(), scene.description());
    }
    println!();
    println!("Run all scenes: cargo run --example progress-bars");
    println!("Run one scene:  cargo run --example progress-bars -- --scene multiple");
}

fn run_scene(scene: Scene) {
    heading(scene.title());
    match scene {
        Scene::Single => single_scene(),
        Scene::Indeterminate => indeterminate_scene(),
        Scene::Transition => transition_scene(),
        Scene::Multiple => multiple_scene(),
        Scene::Lifecycle => lifecycle_scene(),
        Scene::Inline => inline_scene(),
        Scene::Color => color_scene(),
        Scene::Templates => templates_scene(),
    }
}

fn heading(title: &str) {
    eprintln!();
    eprintln!("== {title} ==");
}

fn style(template: &str) -> ProgressStyle {
    ProgressStyle::with_template(template).expect("showcase templates are valid")
}

fn pause() {
    thread::sleep(STEP_DELAY);
}

fn settle() {
    thread::sleep(SCENE_PAUSE);
}

fn single_scene() {
    let progress = ProgressBar::new(STEPS);
    progress.set_style(
        style("{prefix:.bold} [{bar:28.cyan/blue}] {pos:>2}/{len:2} {msg}").progress_chars("=>-"),
    );
    progress.set_prefix("single");
    progress.set_message("one task");

    for _ in 0..STEPS {
        progress.inc(1);
        pause();
    }

    progress.finish_with_message("complete");
    settle();
    progress.finish_and_clear();
}

fn indeterminate_scene() {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        style("{prefix:.bold} {spinner:.magenta} {wide_msg}")
            .tick_strings(&[".", "o", "O", "o", " "]),
    );
    spinner.set_prefix("spinner");
    spinner.set_message("waiting for metadata");
    spinner.enable_steady_tick(TICK_INTERVAL);

    thread::sleep(Duration::from_millis(800));

    spinner.finish_with_message("metadata discovered");
    spinner.disable_steady_tick();
    settle();
    spinner.finish_and_clear();
}

fn transition_scene() {
    let progress = ProgressBar::new_spinner();
    progress.set_style(
        style("{prefix:.bold} {spinner:.yellow} {wide_msg}")
            .tick_strings(&[".", "o", "O", "o", " "]),
    );
    progress.set_prefix("transition");
    progress.set_message("size is unknown");
    progress.enable_steady_tick(TICK_INTERVAL);

    thread::sleep(Duration::from_millis(500));

    progress.set_length(STEPS);
    progress.set_style(
        style("{prefix:.bold} [{bar:24.green/blue}] {pos:>2}/{len:2} {msg}").progress_chars("=>-"),
    );
    progress.set_message("size discovered");

    for _ in 0..STEPS / 2 {
        progress.inc(1);
        pause();
    }

    progress.unset_length();
    progress.set_style(
        style("{prefix:.bold} {spinner:.yellow} {wide_msg}")
            .tick_strings(&[".", "o", "O", "o", " "]),
    );
    progress.set_message("more work discovered");
    thread::sleep(Duration::from_millis(450));

    progress.set_length(STEPS);
    progress.set_style(
        style("{prefix:.bold} [{bar:24.green/blue}] {pos:>2}/{len:2} {msg}").progress_chars("=>-"),
    );
    progress.set_message("known work resumed");

    for _ in 0..STEPS / 2 {
        progress.inc(1);
        pause();
    }

    progress.finish_with_message("complete");
    progress.disable_steady_tick();
    settle();
    progress.finish_and_clear();
}

fn multiple_scene() {
    let multi = MultiProgress::new();
    let total = multi.add(ProgressBar::new(STEPS));
    let compile = multi.insert_before(&total, ProgressBar::new(STEPS));
    let fetch = multi.insert_before(&compile, ProgressBar::new(STEPS));

    let bar_style =
        style("{prefix:<10} [{bar:20.cyan/blue}] {pos:>2}/{len:2} {msg}").progress_chars("=>-");
    fetch.set_style(bar_style.clone());
    compile.set_style(bar_style.clone());
    total.set_style(bar_style);
    fetch.set_prefix("fetch");
    compile.set_prefix("compile");
    total.set_prefix("total");
    fetch.set_message("download dependencies");
    compile.set_message("build sources");
    total.set_message("track all work");
    multi
        .println("Bars were inserted before the total bar to form a stable stack.")
        .expect("writing through MultiProgress succeeds");

    for step in 0..STEPS {
        fetch.set_position(((step + 1) * 2).min(STEPS));
        compile.inc(1);
        total.inc(1);
        if step == STEPS / 2 {
            multi
                .println("A MultiProgress keeps output above every active bar.")
                .expect("writing through MultiProgress succeeds");
        }
        pause();
    }

    fetch.finish_with_message("dependencies ready");
    compile.finish_with_message("sources built");
    total.finish_with_message("all work complete");
    settle();
    multi.clear().expect("clearing MultiProgress succeeds");
}

fn lifecycle_scene() {
    let multi = MultiProgress::new();
    let retained = multi.add(ProgressBar::new(STEPS));
    let transient = multi.insert_before(&retained, ProgressBar::new(STEPS));

    let bar_style =
        style("{prefix:<10} [{bar:20.green/yellow}] {pos:>2}/{len:2} {msg}").progress_chars("=>-");
    retained.set_style(bar_style.clone());
    transient.set_style(bar_style);
    retained.set_prefix("retained");
    transient.set_prefix("transient");
    retained.set_message("will remain visible");
    transient.set_message("will be cleared");

    for _ in 0..STEPS {
        retained.inc(1);
        transient.inc(1);
        pause();
    }

    retained.finish_with_message("still visible after finishing");
    transient.finish_and_clear();
    multi
        .println("The retained result remains while the transient bar disappears.")
        .expect("writing through MultiProgress succeeds");
    settle();
    multi.clear().expect("clearing MultiProgress succeeds");
}

fn inline_scene() {
    let multi = MultiProgress::new();
    let first = multi.add(ProgressBar::new(STEPS));
    let second = multi.add(ProgressBar::new(STEPS));

    let bar_style =
        style("{prefix:<10} [{bar:20.magenta/blue}] {pos:>2}/{len:2} {msg}").progress_chars("=>-");
    first.set_style(bar_style.clone());
    second.set_style(bar_style);
    first.set_prefix("first");
    second.set_prefix("second");
    first.set_message("processing");
    second.set_message("processing");

    for step in 0..STEPS {
        first.inc(1);
        second.set_position(((step + 1) * 2).min(STEPS));
        if step == 3 {
            multi
                .println("MultiProgress::println writes above all of its bars.")
                .expect("writing through MultiProgress succeeds");
        }
        if step == STEPS / 2 {
            first.println("ProgressBar::println is also safe for a bar in a MultiProgress.");
        }
        pause();
    }

    first.finish_with_message("first complete");
    second.finish_with_message("second complete");
    settle();
    multi.clear().expect("clearing MultiProgress succeeds");
}

fn color_scene() {
    let multi = MultiProgress::new();
    let warm = multi.add(ProgressBar::new(STEPS));
    let cool = multi.add(ProgressBar::new(STEPS));

    warm.set_style(
        style("{prefix:.red.bold} [{bar:24.red/yellow}] {pos:>2}/{len:2} {msg}")
            .progress_chars("=>-"),
    );
    cool.set_style(
        style("{prefix:.blue.bold} [{bar:24.blue/cyan}] {pos:>2}/{len:2} {msg}")
            .progress_chars("=>-"),
    );
    warm.set_prefix("warm");
    cool.set_prefix("cool");
    warm.set_message("filled and remaining use different styles");
    cool.set_message("rows can use different palettes");

    for _ in 0..STEPS {
        warm.inc(1);
        cool.inc(1);
        pause();
    }

    warm.finish_with_message("warm palette complete");
    cool.finish_with_message("cool palette complete");
    settle();
    multi.clear().expect("clearing MultiProgress succeeds");
}

fn templates_scene() {
    let multi = MultiProgress::new();
    let prefix_before = multi.add(ProgressBar::new(STEPS));
    let message_before = multi.add(ProgressBar::new(STEPS));
    let metrics_after = multi.add(ProgressBar::new(STEPS));
    let wide_bar = multi.add(ProgressBar::new(STEPS));
    let wide_message = multi.add(ProgressBar::new_spinner());

    prefix_before.set_style(style("{prefix:<12} [{bar:18.cyan/blue}] {msg}").progress_chars("=>-"));
    message_before.set_style(
        style("{msg:<16} [{bar:18.green/yellow}] {pos:>2}/{len:2}").progress_chars("=>-"),
    );
    metrics_after.set_style(
        style("{prefix:>12} [{bar:18.magenta/blue}] {percent:>3}% {eta}").progress_chars("=>-"),
    );
    wide_bar.set_style(style("{prefix:<12} {wide_bar:.yellow/blue} {percent:>3}%"));
    wide_message.set_style(
        style("{prefix:<12} {spinner:.cyan} {wide_msg}").tick_strings(&[".", "o", "O", "o", " "]),
    );

    prefix_before.set_prefix("before bar");
    prefix_before.set_message("after bar");
    message_before.set_message("message first");
    metrics_after.set_prefix("metrics last");
    wide_bar.set_prefix("wide bar");
    wide_message.set_prefix("wide message");
    wide_message.set_message("takes the remaining terminal width");
    wide_message.enable_steady_tick(TICK_INTERVAL);

    for step in 0..STEPS {
        prefix_before.inc(1);
        message_before.inc(1);
        metrics_after.inc(1);
        wide_bar.inc(1);
        if step == STEPS / 2 {
            wide_message.set_message("remains after the spinner on the same line");
        }
        pause();
    }

    prefix_before.finish_with_message("complete");
    message_before.finish();
    metrics_after.finish();
    wide_bar.finish();
    wide_message.finish_with_message("wide message complete");
    wide_message.disable_steady_tick();
    settle();
    multi.clear().expect("clearing MultiProgress succeeds");
}
