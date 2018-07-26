use super::rule_checker::*;
use super::board::*;

pub struct RuleCheckerObserver <'a>{
    rule_checker: *mut RuleChecker<'a>,
}

impl<'a> BoardObserver for RuleCheckerObserver<'a> {
    fn board_updated(&self, row: usize, col: usize, op: BoardOperation) {
        unsafe {
            (*self.rule_checker).update_option_evaluation(row, col, op);
        }
    }
}

impl<'a> RuleCheckerObserver<'a> {
    pub fn create_with_detail(rule_checker: *mut RuleChecker<'a>) -> Self {
        return RuleCheckerObserver {
            rule_checker,
        }
    }
}