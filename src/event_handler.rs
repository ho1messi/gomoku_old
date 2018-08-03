use sciter::dom::Element;
use sciter::dom::event;
use sciter::HELEMENT;
use sciter::value::Value;

use board_controller::*;

pub struct EventHandler {
    root: Option<Element>,
    board_controller: BoardController,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        return EventHandler{
            root: None,
            board_controller: BoardController::new(),
        };
    }

    fn on_cross_point_click(&self, row: i32, col: i32) -> Value {
        let mut value = Value::new();

        value.set_item("current_chess", self.board_controller.get_current_chess());
        self.board_controller.put_chess(row, col);

        value.set_item("have_game_over", self.board_controller.have_game_over());
        if let Some(winner) = self.board_controller.get_winner() {
            value.set_item("winner", winner);
        }

        return value;
    }

    fn on_back_move(&self) -> Value {
        let mut value = Value::new();

        if let Some(coord) = self.board_controller.remove_last_chess() {
            value.set_item("row", coord.0);
            value.set_item("col", coord.1);
        }

        if let Some(last_step) = self.board_controller.get_last_step() {
            value.set_item("last_row", last_step.0);
            value.set_item("last_col", last_step.1);
            value.set_item("last_chess", last_step.2);
        }
        return value;
    }

    fn on_restart_game(&self) -> Value {
        self.board_controller.restart_game();
        return Value::new();
    }
}

impl event::EventHandler for EventHandler {
    fn attached(&mut self, root: HELEMENT) {
        self.root = Some(Element::from(root));
    }

    dispatch_script_call! {
        fn on_cross_point_click(i32, i32);
        fn on_back_move();
        fn on_restart_game();
    }
}