use std::time::Duration;
use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskType {
    Add10,
    Add20,
    AddQuest10,
    AddQuest20,
    Sub10,
    Sub20,
    SubQuest10,
    SubQuest20,
    Mix,
    Add5Elem,
    Mix5Elem,
}

#[derive(Debug, PartialEq)]
pub struct Task {
    pub parameter1: i32,
    pub parameter2: i32,
    pub task_result: i32,
    pub task_type: TaskType,
}

impl TaskType {
    pub fn get_max_sum(self) -> i32 {
        match self {
            TaskType::Add10 | TaskType::Sub10 | TaskType::AddQuest10 | TaskType::SubQuest10 => 10,
            _ => 20,
        }
    }

    pub fn is_parameter_quest(self) -> bool {
        matches!(
            self,
            TaskType::AddQuest10 | TaskType::AddQuest20 | TaskType::SubQuest10 | TaskType::SubQuest20
        )
    }

    pub fn task_op(self) -> &'static str {
        match self {
            TaskType::Add10 | TaskType::Add20 | TaskType::AddQuest10 | TaskType::AddQuest20 => "+",
            _ => "-",
        }
    }

    pub fn from(i: i32) -> Self {
        match i {
            0 => TaskType::Add10,
            1 => TaskType::Add20,
            2 => TaskType::AddQuest10,
            3 => TaskType::AddQuest20,
            4 => TaskType::Sub10,
            5 => TaskType::Sub20,
            6 => TaskType::SubQuest10,
            7 => TaskType::SubQuest20,
            8 => TaskType::Mix,
            9 => TaskType::Add5Elem,
            _ => TaskType::Mix5Elem,
        }
    }

    pub fn timeDisplayAbacus(self) -> Duration {
        match self {
            TaskType::AddQuest10 | TaskType::AddQuest20 | TaskType::SubQuest10 | TaskType::SubQuest20 | TaskType::Sub10 | TaskType::Sub20 => Duration::from_secs(10),
            _ => Duration::from_secs(5),
        }
    }
}

impl Task {
    pub fn gen_task(task_type: TaskType) -> Self {
        let mut rng = rand::thread_rng();
        let mut task = Task {
            parameter1: 0,
            parameter2: 0,
            task_result: 0,
            task_type,
        };

        let max_parameter = 10;
        let max_sum = task_type.get_max_sum();

        let par1 = rng.gen_range(1..max_parameter);
        let par2 = if max_sum - par1 == 1 {
            1
        } else {
            let pmin = std::cmp::min(max_sum - par1, max_parameter);
            rng.gen_range(1..=pmin)
        };
        let result = par1 + par2;
        let mut ctask_type = task_type;
        if task_type == TaskType::Mix {
            let rnd_type = [TaskType::Add20, TaskType::AddQuest20, TaskType::Sub20, TaskType::SubQuest20];
            ctask_type = rnd_type[rng.gen_range(0..rnd_type.len())];
        }

        match ctask_type {
            TaskType::Sub10 | TaskType::Sub20 | TaskType::SubQuest10 | TaskType::SubQuest20 => {
                let sum = result;
                let add1 = par1;
                let add2 = par2;
                task.parameter1 = sum;
                task.parameter2 = add1;
                task.task_result = add2;
            }
            _ => {
                task.parameter1 = par1;
                task.parameter2 = par2;
                task.task_result = result;
            }
        }

        task.task_type = ctask_type;
        task
    }

    pub fn user_expected_result(&self) -> i32 {
        if self.task_type.is_parameter_quest() {
            self.parameter2
        } else {
            self.task_result
        }
    }

    pub fn check_result(&self, user_result: i32) -> bool {
        self.user_expected_result() == user_result
    }
}