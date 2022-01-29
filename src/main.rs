use core::fmt;
use eframe::{egui, epi};
use math::Matrix;
use strum::{Display, EnumIter, IntoEnumIterator};
use std::hash::Hash;

mod math;

const N: usize = 4;

#[derive(Display, EnumIter, Clone, Copy, PartialEq, Eq)]
enum Operation {
    #[strum(to_string = "\u{00d7}")]
    Mul,
    #[strum(to_string = "+")]
    Add,
    #[strum(to_string = "\u{2212}")]
    Sub,
}

impl Operation {
    fn calc<const N: usize>(&self, m1: Matrix<N>, m2: Matrix<N>) -> Matrix<N> {
        match self {
            Self::Mul => m1 * m2,
            Self::Add => m1 + m2,
            Self::Sub => m1 - m2,
        }
    }
}

struct App {
    m1: Matrix<N>,
    m2: Matrix<N>,
    result: Matrix<N>,
    operation: Operation,
}

impl Default for App {
    fn default() -> Self {
        let operation = Operation::Mul;
        let m1 = Matrix::identity();
        let m2 = Matrix::identity();
        let result = operation.calc(m1, m2);

        Self {
            m1,
            m2,
            result,
            operation,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Matrix Calculator"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui_matrix_edit(ui, &mut self.m1).changed()
                    || ui_enum_select(ui, "operation_select", &mut self.operation)
                        .map(|r| r.changed())
                        .unwrap_or(false)
                    || ui_matrix_edit(ui, &mut self.m2).changed()
                {
                    self.recalc();
                }

                ui.label("=");
                ui_matrix(ui, &self.result);
            });
        });
    }
}

impl App {
    fn recalc(&mut self) {
        self.result = self.operation.calc(self.m1, self.m2);
    }
}

fn ui_matrix_edit<const N: usize>(ui: &mut egui::Ui, m: &mut Matrix<N>) -> egui::Response {
    ui.vertical(|ui| {
        let mut r = (0..N)
            .map(|i| {
                ui.horizontal(|ui| {
                    (0..N)
                        .map(|j| ui.add_sized([40.0, 20.0], egui::DragValue::new(&mut m[(i, j)])))
                        .reduce(|a, b| a | b)
                        .unwrap()
                })
            })
            .map(|r| r.inner)
            .reduce(|a, b| a | b)
            .unwrap();

        ui.horizontal(|ui| {
            if ui.button("Zero").clicked() {
                *m = Matrix::default();
                r.mark_changed();
            }

            if ui.button("Identity").clicked() {
                *m = Matrix::identity();
                r.mark_changed();
            }
        });

        r
    })
    .inner
}

fn ui_matrix<const N: usize>(ui: &mut egui::Ui, m: &Matrix<N>) {
    ui.vertical(|ui| {
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        for i in 0..N {
            ui.horizontal(|ui| {
                for j in 0..N {
                    ui.add_sized([40.0, 20.0], egui::Label::new(format!("{}", m[(i, j)])));
                }
            });
        }
    });
}

fn ui_enum_select<E, I>(ui: &mut egui::Ui, id: I, current: &mut E) -> Option<egui::Response>
where
    E: IntoEnumIterator + Clone + Eq + fmt::Display,
    I: Hash,
{
    egui::ComboBox::from_id_source(id)
        .selected_text(format!("{}", current))
        .show_ui(ui, |ui| {
            E::iter()
                .map(|variant| {
                    ui.selectable_value(current, variant.clone(), format!("{}", variant))
                })
                .reduce(|a, b| a | b)
                .unwrap()
        })
        .inner
}

fn main() {
    let app = App::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), options);
}
