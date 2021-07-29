use std::collections::HashMap;

use cassowary::strength::*;
use cassowary::WeightedRelation::*;
use cassowary::{Constraint, Solver, Variable};

use eframe::egui::Ui;
use eframe::egui::{self, Direction, Pos2};

#[derive(Debug)]
pub struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}
impl Rect {
    /// Returns the x-coordinate for the left side of the rectangle.
    fn left(&self) -> f32 {
        self.x
    }
    /// Returns the x-coordinate for the right side of the rectangle.
    fn right(&self) -> f32 {
        self.x + self.w
    }
    /// Returns the y-coordinate for the top side of the rectangle.
    fn top(&self) -> f32 {
        self.y
    }
    /// Returns the y-coordinate for the bottom side of the rectangle.
    fn bottom(&self) -> f32 {
        self.y + self.h
    }
}

impl From<(u32, u32, u32, u32)> for Rect {
    fn from((x, y, w, h): (u32, u32, u32, u32)) -> Self {
        Rect {
            x: x as f32,
            y: y as f32,
            w: w as f32,
            h: h as f32,
        }
    }
}

impl From<egui::Rect> for Rect {
    fn from(rect: egui::Rect) -> Self {
        let egui::Rect {
            min: Pos2 { x, y },
            max: Pos2 { x: w, y: h },
        } = rect;
        Rect {
            x: x as f32,
            y: y as f32,
            w: w as f32,
            h: h as f32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutConstraint {
    Percentage(u16),
    Ratio(u32, u32),
    Length(u16),
    Max(u16),
    Min(u16),
}
macro_rules! var_struct {
    ($name: ident, {
        $($e: ident),*
    }) => {
        struct $name {
            $($e: cassowary::Variable,)*
        };
        impl Default for $name {
            fn default() -> Self {
                Self {
                    $($e: cassowary::Variable::new(),)*
                }
            }
        };
    };
}

macro_rules! var_map {
    ($($e:expr, $l:literal),*) => {
        {
            let mut hm: HashMap<cassowary::Variable, String> = std::collections::HashMap::new();
            $(hm.insert($e, $l.into());)*
            hm
        }
    };
}

//https://github.com/fdehau/tui-rs/blob/4e76bfa2ca8eb51719d611bb8d3d4094ab8ba398/src/layout.rs
pub struct Layout {
    direction: Direction,
    constraints: Vec<LayoutConstraint>,
}
impl Layout {
    fn default() -> Self {
        Self {
            direction: Direction::LeftToRight,
            constraints: Vec::new(),
        }
    }
    fn constraints<C: Into<Vec<LayoutConstraint>>>(mut self, constraints: C) -> Self {
        self.constraints = constraints.into();
        self
    }
    fn solve(area: Rect, layout: &Layout) -> Vec<Rect> {
        dbg!(area);
        dbg!(&layout.constraints);
        let mut solver = Solver::new();
        let mut variables: HashMap<Variable, (usize, usize)> = HashMap::new();
        // solver.add_constraints(&layout.constraints);
        vec![Rect::from((0, 0, 5, 5))]
    }
    /// Chunks provide the portion of space they should take from the layout
    /// i.e. [1, 4] should map to:
    ///
    /// `chunk[0]` having 1/5 space, and
    /// `chunk[1]` having 4/5 space.
    pub fn horizontal(ctx: &egui::Context, area: Rect, chunks: &[f32]) -> Self {
        let mut solver = Solver::new();

        var_struct!(LeftRightBounds, {left, right});
        let area_bounds = LeftRightBounds::default();
        solver
            .add_constraints(&[
                area_bounds.left | EQ(REQUIRED) | area.left(),
                area_bounds.right | EQ(REQUIRED) | area.right(),
            ])
            .unwrap();
        let total: f32 = chunks.iter().sum();
        let mut curr = 0.0;
        let bounds = chunks.iter().map(|numerator| LeftRightBounds::default());
        for (i, numerator) in chunks.iter().enumerate() {
            let bounds = LeftRightBounds::default();
            let partial_area = (*numerator) / total;
            println!("{} {} {}", i, numerator, partial_area * area.w);
            #[rustfmt::skip]
            solver.add_constraints(&[
                    bounds.right - bounds.left |EQ(STRONG)| partial_area * area.w,
                    bounds.left |GE(REQUIRED)| curr * area.w,
                    bounds.right |GE(REQUIRED)| bounds.left,
                ]).unwrap();
            curr += partial_area;
        }

        Self {
            direction: Direction::LeftToRight,
            constraints: vec![],
            // constraints: chunks.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use cassowary::strength::REQUIRED;
    use cassowary::strength::WEAK;
    use cassowary::Solver;
    use cassowary::Variable;
    use cassowary::WeightedRelation::*;

    use super::Layout;
    use super::Rect;

    #[test]
    fn side_by_side() {
        let mut solver = Solver::new();
        let area = Rect::from((0, 0, 20, 20));
        let box1 = Rect::from((0, 0, 10, 10));
        let box2 = Rect::from((0, 0, 10, 10));
        struct Element {
            left: Variable,
            right: Variable,
        }
        let area = Element {
            left: Variable::new(),
            right: Variable::new(),
        };
        let box1 = Element {
            left: Variable::new(),
            right: Variable::new(),
        };
        let box2 = Element {
            left: Variable::new(),
            right: Variable::new(),
        };
        let map = var_map! {
            area.left, "area.left",
            area.right, "area.right",
            box2.right, "box2.right",
            box2.left, "box2.left",
            box1.right, "box1.right",
            box1.left, "box1.left"
        };
        #[rustfmt::skip]
        solver.add_constraints(&[
            area.right |EQ(REQUIRED)| 20.0,
            area.left |EQ(REQUIRED)| 0.0,
            box1.left |EQ(REQUIRED)| 0.0,
            box2.right |EQ(REQUIRED)| area.right,
            box2.left |GE(REQUIRED)| box1.right,
            box2.left |GE(REQUIRED)| box1.right,
            box1.right - box1.left |EQ(WEAK)| 10.0,
            box2.right - box2.left |EQ(WEAK)| 10.0
        ]).unwrap();
        eprintln!("?");

        assert_eq!(solver.get_value(area.left), 0.0);
        assert_eq!(solver.get_value(area.right), 20.0);
        assert_eq!(solver.get_value(box1.left), 0.0);
        assert_eq!(solver.get_value(box1.right), 10.0);
        assert_eq!(solver.get_value(box2.left), 10.0);
        assert_eq!(solver.get_value(box2.right), 20.0);
    }
}
