use super::rule_checker::*;
use super::board::*;

pub struct RuleCheckerObserver {
    rule_checker: *mut RuleChecker,
}

impl BoardObserver for RuleCheckerObserver {
    fn board_updated(&self, event: BoardEvent) {
        unsafe {
            (*self.rule_checker).update_option_evaluation(event);
        }
    }
}

impl RuleCheckerObserver {
    pub fn create_with_detail(rule_checker: *mut RuleChecker) -> Self {
        return RuleCheckerObserver {
            rule_checker,
        }
    }
}