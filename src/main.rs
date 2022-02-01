#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use wasm_bindgen::{prelude::*, JsCast};

use console_error_panic_hook::set_once as set_panic_hook;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::boxed::Box;
use web_sys::{window, CanvasRenderingContext2d};
#[derive(Clone)]
pub struct App {
    ctx: CanvasRenderingContext2d,
    width: f64,
    height: f64,
    palette: [String; 10],
    central_point: (f64, f64),
}
impl App {
    pub fn new() -> Self {
        let document = window()
            .and_then(|win| win.document())
            .expect("Could not access document");
        let canvas = document
            .get_element_by_id("canvas")
            .expect("Couldn't get canvas element")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Could not cast into canvas element");
        let canvas_parent = document
            .get_element_by_id("canvas_parent")
            .expect("Couldn't get canvas parent element")
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Could not cast into canvas parent element");
        let ctx = canvas
            .get_context("2d")
            .expect("Could not get context")
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let c_w_h = canvas_parent.get_bounding_client_rect();
        let width = c_w_h.width();
        let height = c_w_h.height();
        let palette = [
            String::from("#E07E69"),
            String::from("#536E8F"),
            String::from("#FED700"),
            String::from("#CB4204"),
            String::from("#272727"),
            String::from("#56325C"),
            String::from("#9BA77C"),
            String::from("#88D4D1"),
            String::from("#DEAF48"),
            String::from("#FFFFFF"),
        ];

        // canvas.style().set_property("width", &format!("{}px", width)).expect("Unable to set style width");
        // canvas.style().set_property("height", &format!("{}px", height)).expect("Unable to set style height");
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let mut rng = thread_rng();
        let central_point_x = rng.gen_range((width * 0.3)..(width - (width * 0.3)));
        let central_point_y = rng.gen_range((height * 0.3)..(height - (height * 0.3)));

        let app = App {
            ctx,
            width,
            height,
            palette,
            central_point: (central_point_x as f64, central_point_y as f64),
        };
        let app_obj = app.clone();
        let re_draw = Closure::wrap(Box::new(move || {
            app.clone().draw();
        }) as Box<dyn Fn()>);
        canvas.set_onclick(Some(re_draw.as_ref().unchecked_ref()));

        re_draw.forget();
        app_obj
    }
    pub fn draw(&self) -> () {
        let mut rng = thread_rng();
        self.bg();
        let total =
            rng.gen_range((self.width * self.height) * 0.002..((self.width * self.height) * 0.005));
        for _ in 0..total as usize {
            match rng.gen_range(0.0..1.0) {
                x if (0.0..0.6).contains(&x) => self.dots(),
                x if (0.6..0.9).contains(&x) => self.arcs(),
                _ => self.lines(),
            }
        }
    }

    pub fn next_random(&self) -> (f64, f64) {
        let mut rng = thread_rng();
        let angle = rng.gen_range(0.0..(std::f64::consts::PI * 2.0));
        let radius = match rng.gen_range(0.0..1.0) {
            x if (0.0..0.3).contains(&x) => {
                rng.gen_range((self.central_point.0)..(self.central_point.0 + (self.width * 0.3)))
            }
            x if (0.3..0.7).contains(&x) => {
                rng.gen_range((self.central_point.0)..(self.central_point.0 + (self.height * 0.6)))
            }
            _ => rng.gen_range((self.central_point.0)..(self.central_point.0 + self.height * 0.9)),
        };
        let point_x = angle.cos() * radius;
        let point_y = angle.sin() * radius;

        (point_x, point_y)
    }

    pub fn bg(&self) -> () {
        let gradient = self
            .ctx
            .create_linear_gradient(0.0, 0.0, self.width, self.height);
        gradient
            .add_color_stop(0.0, "#E6D5C0")
            .expect("Unable to add color step");
        gradient
            .add_color_stop(0.5, "#EBE3D8")
            .expect("Unable to add color step");
        gradient
            .add_color_stop(1.0, "#E6D5C0")
            .expect("Unable to add color step");

        self.ctx.set_fill_style(&JsValue::from(gradient));
        self.ctx.fill_rect(0.0, 0.0, self.width, self.height);
    }

    pub fn lines(&self) -> () {
        let mut rng = thread_rng();

        let p_initial = self.next_random();
        let p_end = self.next_random();

        let weight = rng.gen_range(1..3);

        self.ctx.begin_path();
        self.ctx.move_to(p_initial.0 as f64, p_initial.1 as f64);
        self.ctx.line_to(p_end.0 as f64, p_end.1 as f64);

        self.ctx.set_line_width(weight as f64);
        self.ctx.set_stroke_style(&JsValue::from_str("black"));
        self.ctx.stroke();
    }

    pub fn arcs(&self) -> () {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..self.width as u64);
        let y = rng.gen_range(0..self.height as u64);
        let r = rng.gen_range(5..40);
        let start = rng.gen_range(0..360u64);
        let end = rng.gen_range(0..360u64);
        let with_fill = rng.gen_bool(1.0 / 3.0);
        let with_stroke = rng.gen_bool(1.0 / 3.0);
        let with_alpha = rng.gen_bool(1.0 / 3.0);
        let col = self
            .palette
            .choose(&mut rand::thread_rng())
            .expect("Unable to pick color palette.");
        self.ctx.begin_path();
        self.ctx
            .arc(x as f64, y as f64, r as f64, start as f64, end as f64)
            .expect("Unable to create an arc.");

        if with_fill {
            if with_alpha {
                self.ctx.set_global_alpha(0.5);
            } else {
                self.ctx.set_global_alpha(1.0);
            }
            self.ctx.set_fill_style(&JsValue::from_str(col));
            self.ctx.fill();
        }

        if with_stroke {
            self.ctx.set_line_width(1.0);
            self.ctx.set_stroke_style(&JsValue::from_str("black"));
            self.ctx.stroke();
        }
    }

    pub fn dots(&self) -> () {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..self.width as u64);
        let y = rng.gen_range(0..self.height as u64);
        let r = rng.gen_range(1..3);
        let with_stroke = rng.gen_bool(1.0 / 3.0);
        let col = self
            .palette
            .choose(&mut rand::thread_rng())
            .expect("Unable to pick color palette.");
        self.ctx.begin_path();
        self.ctx
            .ellipse(x as f64, y as f64, r as f64, r as f64, 0.0, 0.0, 360.0)
            .expect("unable to create ellipse");
        if with_stroke {
            self.ctx.set_stroke_style(&JsValue::from_str("black"));
            self.ctx.stroke();
        }
        self.ctx.set_global_alpha(1.0);
        self.ctx.set_fill_style(&JsValue::from_str(col));
        self.ctx.fill();
    }
}

fn start_app() {
    let app = App::new();
    app.draw();
}

fn main() {
    set_panic_hook();
    start_app();
}
